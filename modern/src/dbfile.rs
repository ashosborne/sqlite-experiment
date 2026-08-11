//! Minimal SQLite database file format writer/reader (pack v6, engine-files).
//!
//! HONESTY GATE (pack v6): a file this module writes must be openable and
//! queryable by the pinned C amalgamation (fileformat2.html). It is NOT a
//! private dump — it is the real on-disk format: 100-byte database header,
//! table b-tree LEAF pages, and records with SQLite serial types.
//!
//! Deliberate limits (documented, not hidden): page size 4096; single leaf
//! page per table (no interior pages, no overflow — tiny data only); columns
//! reconstructed as `CREATE TABLE t(c1,c2,...)`; INTEGER/TEXT/NULL values only;
//! no WAL, no freelist, no incremental vacuum. Advanced kitchen features
//! (UNIQUE/FK/triggers) are NOT persisted this version — file mode serves the
//! engine-files subset; the in-memory kitchen keeps the rest.

use crate::store::Val;
use std::path::Path;

const PAGE: usize = 4096;
const HEADER: &[u8; 16] = b"SQLite format 3\0";

// ---------------- varint ----------------
fn put_varint(out: &mut Vec<u8>, v: u64) {
    // Values here are always < 2^56, so the 8-bit 9th-byte special case never triggers.
    let mut groups = Vec::new();
    let mut x = v;
    loop {
        groups.push((x & 0x7f) as u8);
        x >>= 7;
        if x == 0 {
            break;
        }
    }
    for i in (0..groups.len()).rev() {
        let mut b = groups[i];
        if i != 0 {
            b |= 0x80;
        }
        out.push(b);
    }
}

fn get_varint(buf: &[u8], pos: &mut usize) -> u64 {
    let mut v: u64 = 0;
    for _ in 0..9 {
        let b = buf[*pos];
        *pos += 1;
        v = (v << 7) | (b & 0x7f) as u64;
        if b & 0x80 == 0 {
            break;
        }
    }
    v
}

// ---------------- serial types ----------------
fn int_serial(v: i64) -> (u64, Vec<u8>) {
    match v {
        0 => (8, vec![]),
        1 => (9, vec![]),
        _ if (-128..=127).contains(&v) => (1, vec![v as i8 as u8]),
        _ if (-32768..=32767).contains(&v) => (2, (v as i16).to_be_bytes().to_vec()),
        _ if (-8_388_608..=8_388_607).contains(&v) => (3, (v as i32).to_be_bytes()[1..].to_vec()),
        _ if (-2_147_483_648..=2_147_483_647).contains(&v) => (4, (v as i32).to_be_bytes().to_vec()),
        _ => (6, v.to_be_bytes().to_vec()),
    }
}

fn encode_record(vals: &[Val]) -> Vec<u8> {
    let mut stypes = Vec::new();
    let mut body = Vec::new();
    for v in vals {
        match v {
            Val::Null => put_varint(&mut stypes, 0),
            Val::Int(i) => {
                let (st, b) = int_serial(*i);
                put_varint(&mut stypes, st);
                body.extend_from_slice(&b);
            }
            Val::Text(t) => {
                let bytes = t.as_bytes();
                put_varint(&mut stypes, 13 + 2 * bytes.len() as u64);
                body.extend_from_slice(bytes);
            }
        }
    }
    // header length is a varint that counts itself
    let mut hlen = stypes.len() as u64 + 1;
    let mut header = Vec::new();
    put_varint(&mut header, hlen);
    if header.len() != 1 {
        // header-length varint took >1 byte; recompute once (enough for our sizes)
        hlen = stypes.len() as u64 + header.len() as u64;
        header.clear();
        put_varint(&mut header, hlen);
    }
    header.extend_from_slice(&stypes);
    header.extend_from_slice(&body);
    header
}

fn decode_record(payload: &[u8]) -> Vec<Val> {
    let mut pos = 0usize;
    let hlen = get_varint(payload, &mut pos) as usize;
    let mut stypes = Vec::new();
    while pos < hlen {
        stypes.push(get_varint(payload, &mut pos));
    }
    let mut body = hlen;
    let mut out = Vec::new();
    for st in stypes {
        match st {
            0 => out.push(Val::Null),
            8 => out.push(Val::Int(0)),
            9 => out.push(Val::Int(1)),
            1 => { out.push(Val::Int(payload[body] as i8 as i64)); body += 1; }
            2 => { out.push(Val::Int(i16::from_be_bytes([payload[body], payload[body+1]]) as i64)); body += 2; }
            3 => {
                let b = [payload[body], payload[body+1], payload[body+2]];
                let mut x = ((b[0] as i64) << 16) | ((b[1] as i64) << 8) | b[2] as i64;
                if x & 0x80_0000 != 0 { x -= 0x100_0000; }
                out.push(Val::Int(x)); body += 3;
            }
            4 => {
                let x = i32::from_be_bytes([payload[body],payload[body+1],payload[body+2],payload[body+3]]);
                out.push(Val::Int(x as i64)); body += 4;
            }
            5 => {
                let mut x: i64 = 0;
                for k in 0..6 { x = (x << 8) | payload[body+k] as i64; }
                if x & 0x8000_0000_0000 != 0 { x -= 0x1_0000_0000_0000; }
                out.push(Val::Int(x)); body += 6;
            }
            6 => {
                let mut a=[0u8;8]; a.copy_from_slice(&payload[body..body+8]);
                out.push(Val::Int(i64::from_be_bytes(a))); body += 8;
            }
            n if n >= 13 && n % 2 == 1 => {
                let len = ((n - 13) / 2) as usize;
                out.push(Val::Text(String::from_utf8_lossy(&payload[body..body+len]).into_owned()));
                body += len;
            }
            n if n >= 12 && n % 2 == 0 => {
                // blob: not produced by us; render as empty text placeholder
                let len = ((n - 12) / 2) as usize;
                out.push(Val::Text(String::from_utf8_lossy(&payload[body..body+len]).into_owned()));
                body += len;
            }
            _ => out.push(Val::Null),
        }
    }
    out
}

// ---------------- leaf page assembly ----------------
fn build_leaf(cells: &[Vec<u8>], header_off: usize) -> [u8; PAGE] {
    let mut page = [0u8; PAGE];
    let mut content = PAGE;
    let mut ptrs = Vec::new();
    for cell in cells {
        content -= cell.len();
        page[content..content + cell.len()].copy_from_slice(cell);
        ptrs.push(content as u16);
    }
    page[header_off] = 0x0d; // leaf table b-tree
    let ncell = cells.len() as u16;
    page[header_off + 3..header_off + 5].copy_from_slice(&ncell.to_be_bytes());
    let cstart: u16 = if cells.is_empty() { PAGE as u16 } else { content as u16 };
    page[header_off + 5..header_off + 7].copy_from_slice(&cstart.to_be_bytes());
    let mut p = header_off + 8;
    for ptr in ptrs {
        page[p..p + 2].copy_from_slice(&ptr.to_be_bytes());
        p += 2;
    }
    page
}

fn table_cell(rowid: i64, vals: &[Val]) -> Vec<u8> {
    let rec = encode_record(vals);
    let mut cell = Vec::new();
    put_varint(&mut cell, rec.len() as u64);
    put_varint(&mut cell, rowid as u64);
    cell.extend_from_slice(&rec);
    cell
}

/// A table to persist: name, column names, and (rowid, values) rows.
pub struct TableImage {
    pub name: String,
    pub cols: Vec<String>,
    pub rows: Vec<(i64, Vec<Val>)>,
}

pub fn write_db(path: &Path, tables: &[TableImage]) -> std::io::Result<()> {
    let mut pages: Vec<[u8; PAGE]> = Vec::new();
    // page indices: 0 = page1 (schema); tables get pages 2..
    let mut schema_cells = Vec::new();
    for (i, t) in tables.iter().enumerate() {
        let rootpage = (i + 2) as i64;
        let sql = format!("CREATE TABLE {}({})", t.name, t.cols.join(","));
        let rec = vec![
            Val::Text("table".into()),
            Val::Text(t.name.clone()),
            Val::Text(t.name.clone()),
            Val::Int(rootpage),
            Val::Text(sql),
        ];
        schema_cells.push(table_cell((i + 1) as i64, &rec));
    }
    // table data pages (single leaf each)
    let mut data_pages = Vec::new();
    for t in tables {
        let mut cells = Vec::new();
        let mut rows = t.rows.clone();
        rows.sort_by_key(|(rid, _)| *rid); // cells must be rowid-ascending
        for (rid, vals) in &rows {
            cells.push(table_cell(*rid, vals));
        }
        data_pages.push(build_leaf(&cells, 0));
    }
    // page 1 = header + schema leaf (b-tree header at offset 100)
    let mut page1 = build_leaf(&schema_cells, 100);
    let npages = (1 + tables.len()) as u32;
    write_header(&mut page1, npages);
    pages.push(page1);
    pages.extend(data_pages);

    let mut buf = Vec::with_capacity(pages.len() * PAGE);
    for p in &pages {
        buf.extend_from_slice(p);
    }
    std::fs::write(path, buf)
}

fn write_header(page1: &mut [u8; PAGE], npages: u32) {
    page1[0..16].copy_from_slice(HEADER);
    page1[16..18].copy_from_slice(&(PAGE as u16).to_be_bytes()); // page size 4096
    page1[18] = 1; // write version (rollback journal)
    page1[19] = 1; // read version
    page1[20] = 0; // reserved space
    page1[21] = 64;
    page1[22] = 32;
    page1[23] = 32;
    page1[24..28].copy_from_slice(&1u32.to_be_bytes()); // file change counter
    page1[28..32].copy_from_slice(&npages.to_be_bytes()); // in-header db size (pages)
    // 32..40 freelist = 0
    page1[40..44].copy_from_slice(&1u32.to_be_bytes()); // schema cookie
    page1[44..48].copy_from_slice(&4u32.to_be_bytes()); // schema format 4
    // 48..56 = 0
    page1[56..60].copy_from_slice(&1u32.to_be_bytes()); // text encoding UTF-8
    // 60..92 = 0 (user version / vacuum / appid / reserved)
    page1[92..96].copy_from_slice(&1u32.to_be_bytes()); // version-valid-for = change counter
    page1[96..100].copy_from_slice(&3_054_000u32.to_be_bytes()); // sqlite version number
}

// ---------------- reader (also parses files C wrote) ----------------
fn read_leaf_cells(page: &[u8], header_off: usize) -> Vec<(i64, Vec<Val>)> {
    // only leaf table pages (0x0d); interior pages unsupported (tiny-data limit)
    if page[header_off] != 0x0d {
        return Vec::new();
    }
    let ncell = u16::from_be_bytes([page[header_off + 3], page[header_off + 4]]) as usize;
    let mut out = Vec::new();
    for i in 0..ncell {
        let pp = header_off + 8 + i * 2;
        let off = u16::from_be_bytes([page[pp], page[pp + 1]]) as usize;
        let mut pos = off;
        let paylen = get_varint(page, &mut pos) as usize;
        let rowid = get_varint(page, &mut pos) as i64;
        let payload = &page[pos..pos + paylen];
        out.push((rowid, decode_record(payload)));
    }
    out
}

fn parse_cols(sql: &str) -> Vec<String> {
    // reconstruct column names from "CREATE TABLE name(c1, c2 TYPE, ...)"
    let open = match sql.find('(') { Some(o) => o, None => return Vec::new() };
    let close = sql.rfind(')').unwrap_or(sql.len());
    let inner = &sql[open + 1..close];
    let mut cols = Vec::new();
    let mut depth = 0;
    let mut cur = String::new();
    for ch in inner.chars() {
        match ch {
            '(' => { depth += 1; cur.push(ch); }
            ')' => { depth -= 1; cur.push(ch); }
            ',' if depth == 0 => { cols.push(cur.clone()); cur.clear(); }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() { cols.push(cur); }
    cols.iter()
        .filter_map(|c| c.trim().split_whitespace().next().map(|s| s.to_string()))
        .collect()
}

pub fn read_db(path: &Path) -> std::io::Result<Vec<TableImage>> {
    let buf = std::fs::read(path)?;
    if buf.len() < 100 || &buf[0..16] != HEADER {
        return Ok(Vec::new()); // not a db / empty -> caller starts fresh
    }
    let page_size = {
        let v = u16::from_be_bytes([buf[16], buf[17]]) as usize;
        if v == 1 { 65536 } else { v }
    };
    let page_at = |n: usize| -> &[u8] { &buf[(n - 1) * page_size..n * page_size] };
    // schema on page 1, b-tree header at offset 100
    let schema_rows = read_leaf_cells(&buf[0..page_size], 100);
    let mut tables = Vec::new();
    for (_rid, rec) in schema_rows {
        if rec.len() < 5 {
            continue;
        }
        let is_table = matches!(&rec[0], Val::Text(t) if t == "table");
        if !is_table {
            continue;
        }
        let name = match &rec[1] { Val::Text(t) => t.clone(), _ => continue };
        let rootpage = match &rec[3] { Val::Int(i) => *i as usize, _ => continue };
        let sql = match &rec[4] { Val::Text(t) => t.clone(), _ => continue };
        let cols = parse_cols(&sql);
        let rows = if rootpage >= 1 && rootpage * page_size <= buf.len() {
            read_leaf_cells(page_at(rootpage), 0)
        } else {
            Vec::new()
        };
        tables.push(TableImage { name, cols, rows });
    }
    Ok(tables)
}
