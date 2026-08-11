//! SQLite database file format writer/reader — v12 (pack sqlite-experiment-c-to-rust@12).
//!
//! HONESTY GATE: files this module writes must be openable + queryable by the
//! pinned C amalgamation (fileformat2.html). Real format only — no private dump.
//!
//! v7 adds:
//!  - multi-leaf table b-trees with an INTERIOR root page (0x05) for tables that
//!    exceed one 4096 leaf,
//!  - INTEGER PRIMARY KEY = rowid (column stored NULL, value carried by rowid),
//!    so C enforces FK against `id INTEGER PRIMARY KEY` parents natively,
//!  - durable schema rows for tables (with their real CREATE sql: REFERENCES / FK
//!    kept) and triggers (type='trigger', rootpage 0, sql text).
//!
//! v12 pays the v7 disk debt:
//!  - real overflow page chains for payloads exceeding one leaf cell (write + read,
//!    SQLite local/spill formula, 4-byte next pointers, integrity_check-clean),
//!  - real index b-trees (leaf 0x0a) for UNIQUE column autoindexes, multi-column
//!    UNIQUE table constraints, and explicit UNIQUE/plain CREATE INDEX — persisted
//!    in sqlite_schema exactly as C expects (autoindexes with NULL sql), so UNIQUE
//!    survives reopen and C enforces duplicates against Rust-written files,
//!  - BLOB values (serial type 12+2n).
//! Remaining honest limits: single-leaf index b-trees (pinned index content is
//! small), no expression/partial indexes, indexes not used for lookups (scan +
//! constraint checks), no WAL, no freelist (files are fully rewritten on save).

use crate::store::Val;
use std::path::Path;

const PAGE: usize = 4096;
const HEADER: &[u8; 16] = b"SQLite format 3\0";

pub struct TableImage {
    pub name: String,
    pub sql: String,             // persisted CREATE TABLE text (constraints kept, UNIQUE stripped by store)
    pub rows: Vec<(i64, Vec<Val>)>,
}
pub struct TriggerImage {
    pub name: String,
    pub tbl: String,
    pub sql: String,
}
pub struct IndexImage {
    pub name: String,
    pub tbl: String,
    pub sql: Option<String>,                 // None for sqlite_autoindex_* rows
    pub entries: Vec<(Vec<Val>, i64)>,       // (key column values, rowid)
}
#[derive(Default)]
pub struct DbImage {
    pub tables: Vec<TableImage>,
    pub triggers: Vec<TriggerImage>,
    pub indexes: Vec<IndexImage>,
}

// ---------------- varint / serial types ----------------
fn put_varint(out: &mut Vec<u8>, v: u64) {
    let mut g = Vec::new();
    let mut x = v;
    loop { g.push((x & 0x7f) as u8); x >>= 7; if x == 0 { break; } }
    for i in (0..g.len()).rev() { let mut b = g[i]; if i != 0 { b |= 0x80; } out.push(b); }
}
fn get_varint(buf: &[u8], pos: &mut usize) -> u64 {
    let mut v = 0u64;
    for _ in 0..9 { let b = buf[*pos]; *pos += 1; v = (v << 7) | (b & 0x7f) as u64; if b & 0x80 == 0 { break; } }
    v
}
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
    let mut st = Vec::new();
    let mut body = Vec::new();
    for v in vals {
        match v {
            Val::Null => put_varint(&mut st, 0),
            Val::Int(i) => { let (t, b) = int_serial(*i); put_varint(&mut st, t); body.extend_from_slice(&b); }
            Val::Text(t) => { let b = t.as_bytes(); put_varint(&mut st, 13 + 2 * b.len() as u64); body.extend_from_slice(b); }
            Val::Blob(b) => { put_varint(&mut st, 12 + 2 * b.len() as u64); body.extend_from_slice(b); }
        }
    }
    let mut header = Vec::new();
    put_varint(&mut header, st.len() as u64 + 1);
    if header.len() != 1 { header.clear(); put_varint(&mut header, st.len() as u64 + 2); }
    header.extend_from_slice(&st);
    header.extend_from_slice(&body);
    header
}
fn decode_record(payload: &[u8]) -> Vec<Val> {
    let mut pos = 0;
    let hlen = get_varint(payload, &mut pos) as usize;
    let mut sts = Vec::new();
    while pos < hlen { sts.push(get_varint(payload, &mut pos)); }
    let mut body = hlen;
    let mut out = Vec::new();
    for st in sts {
        match st {
            0 => out.push(Val::Null),
            8 => out.push(Val::Int(0)),
            9 => out.push(Val::Int(1)),
            1 => { out.push(Val::Int(payload[body] as i8 as i64)); body += 1; }
            2 => { out.push(Val::Int(i16::from_be_bytes([payload[body], payload[body+1]]) as i64)); body += 2; }
            3 => { let mut x = ((payload[body] as i64)<<16)|((payload[body+1] as i64)<<8)|payload[body+2] as i64;
                   if x & 0x80_0000 != 0 { x -= 0x100_0000; } out.push(Val::Int(x)); body += 3; }
            4 => { out.push(Val::Int(i32::from_be_bytes([payload[body],payload[body+1],payload[body+2],payload[body+3]]) as i64)); body += 4; }
            5 => { let mut x=0i64; for k in 0..6 { x=(x<<8)|payload[body+k] as i64; } if x & 0x8000_0000_0000!=0 { x-=0x1_0000_0000_0000; } out.push(Val::Int(x)); body += 6; }
            6 => { let mut a=[0u8;8]; a.copy_from_slice(&payload[body..body+8]); out.push(Val::Int(i64::from_be_bytes(a))); body += 8; }
            n if n >= 13 && n % 2 == 1 => { let l=((n-13)/2) as usize; out.push(Val::Text(String::from_utf8_lossy(&payload[body..body+l]).into_owned())); body += l; }
            n if n >= 12 => { let l=((n-12)/2) as usize; out.push(Val::Blob(payload[body..body+l].to_vec())); body += l; }
            _ => out.push(Val::Null),
        }
    }
    out
}

// ---------------- ipk detection ----------------
pub fn ipk_index(sql: &str) -> Option<usize> {
    // column i is INTEGER PRIMARY KEY (alias for rowid)
    let open = sql.find('(')?;
    let close = sql.rfind(')')?;
    let inner = &sql[open + 1..close];
    let mut idx = 0;
    let mut depth = 0;
    let mut cur = String::new();
    let mut cols: Vec<String> = Vec::new();
    for ch in inner.chars() {
        match ch {
            '(' => { depth += 1; cur.push(ch); }
            ')' => { depth -= 1; cur.push(ch); }
            ',' if depth == 0 => { cols.push(cur.clone()); cur.clear(); }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() { cols.push(cur); }
    for c in &cols {
        if c.to_ascii_uppercase().contains("INTEGER PRIMARY KEY") { return Some(idx); }
        idx += 1;
    }
    None
}
fn cols_of(sql: &str) -> Vec<String> {
    let open = match sql.find('(') { Some(o) => o, None => return Vec::new() };
    let close = sql.rfind(')').unwrap_or(sql.len());
    let inner = &sql[open + 1..close];
    let mut out = Vec::new();
    let mut depth = 0;
    let mut cur = String::new();
    for ch in inner.chars() {
        match ch { '(' => { depth += 1; cur.push(ch); } ')' => { depth -= 1; cur.push(ch); }
                   ',' if depth == 0 => { out.push(cur.clone()); cur.clear(); } _ => cur.push(ch) }
    }
    if !cur.trim().is_empty() { out.push(cur); }
    out.iter().filter_map(|c| c.trim().split_whitespace().next().map(|s| s.to_string())).collect()
}

// ---------------- cell + page builders ----------------
const USABLE: usize = PAGE;                       // no reserved bytes
const MAX_LOCAL_LEAF: usize = USABLE - 35;        // table leaf spill threshold (4061)
const MIN_LOCAL: usize = (USABLE - 12) * 32 / 255 - 23; // 489
const OV_CAP: usize = USABLE - 4;                 // overflow data per page

/// local (in-cell) byte count for a table-leaf payload (fileformat2.html)
fn local_of(payload: usize) -> usize {
    if payload <= MAX_LOCAL_LEAF { return payload; }
    let surplus = MIN_LOCAL + (payload - MIN_LOCAL) % OV_CAP;
    if surplus <= MAX_LOCAL_LEAF { surplus } else { MIN_LOCAL }
}

/// build a table-leaf cell; payloads exceeding one leaf get a real overflow chain
/// (pages appended to `datapages`; page numbers are 2 + index).
fn table_cell(rowid: i64, vals: &[Val], datapages: &mut Vec<[u8; PAGE]>) -> Vec<u8> {
    let rec = encode_record(vals);
    let mut c = Vec::new();
    put_varint(&mut c, rec.len() as u64);
    put_varint(&mut c, rowid as u64);
    let local = local_of(rec.len());
    c.extend_from_slice(&rec[..local]);
    if local < rec.len() {
        // allocate the overflow chain now; chain pointers filled as we go
        let mut off = local;
        let mut chunks: Vec<&[u8]> = Vec::new();
        while off < rec.len() {
            let end = (off + OV_CAP).min(rec.len());
            chunks.push(&rec[off..end]);
            off = end;
        }
        let first = (2 + datapages.len()) as u32;
        for (i, chunk) in chunks.iter().enumerate() {
            let mut page = [0u8; PAGE];
            let next = if i + 1 < chunks.len() { (2 + datapages.len() + 1) as u32 } else { 0 };
            page[0..4].copy_from_slice(&next.to_be_bytes());
            page[4..4 + chunk.len()].copy_from_slice(chunk);
            datapages.push(page);
        }
        c.extend_from_slice(&first.to_be_bytes());
    }
    c
}

/// sort key across SQLite storage classes (NULL < INT < TEXT < BLOB; binary collation)
fn val_ord(a: &Val, b: &Val) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    let rank = |v: &Val| match v { Val::Null => 0, Val::Int(_) => 1, Val::Text(_) => 2, Val::Blob(_) => 3 };
    match rank(a).cmp(&rank(b)) {
        Equal => match (a, b) {
            (Val::Int(x), Val::Int(y)) => x.cmp(y),
            (Val::Text(x), Val::Text(y)) => x.as_bytes().cmp(y.as_bytes()),
            (Val::Blob(x), Val::Blob(y)) => x.cmp(y),
            _ => Equal,
        },
        o => o,
    }
}

/// index-leaf page (0x0a): cells = varint(payload) + record(key cols + rowid)
fn index_leaf_page(entries: &[(Vec<Val>, i64)]) -> [u8; PAGE] {
    let mut cells: Vec<Vec<u8>> = Vec::new();
    for (key, rowid) in entries {
        let mut vals = key.clone();
        vals.push(Val::Int(*rowid));
        let rec = encode_record(&vals);
        assert!(rec.len() <= (USABLE - 12) * 64 / 255 - 23,
                "v12 limit: index payload must fit one leaf cell (no index overflow)");
        let mut c = Vec::new();
        put_varint(&mut c, rec.len() as u64);
        c.extend_from_slice(&rec);
        cells.push(c);
    }
    let mut page = [0u8; PAGE];
    let mut content = PAGE;
    let mut ptrs = Vec::new();
    for cell in &cells { content -= cell.len(); page[content..content+cell.len()].copy_from_slice(cell); ptrs.push(content as u16); }
    page[0] = 0x0a;
    page[3..5].copy_from_slice(&(cells.len() as u16).to_be_bytes());
    let cs: u16 = if cells.is_empty() { PAGE as u16 } else { content as u16 };
    page[5..7].copy_from_slice(&cs.to_be_bytes());
    let mut p = 8;
    for ptr in ptrs { page[p..p+2].copy_from_slice(&ptr.to_be_bytes()); p += 2; }
    page
}
fn leaf_page(cells: &[Vec<u8>], header_off: usize) -> [u8; PAGE] {
    let mut page = [0u8; PAGE];
    let mut content = PAGE;
    let mut ptrs = Vec::new();
    for cell in cells { content -= cell.len(); page[content..content+cell.len()].copy_from_slice(cell); ptrs.push(content as u16); }
    page[header_off] = 0x0d;
    page[header_off+3..header_off+5].copy_from_slice(&(cells.len() as u16).to_be_bytes());
    let cs: u16 = if cells.is_empty() { PAGE as u16 } else { content as u16 };
    page[header_off+5..header_off+7].copy_from_slice(&cs.to_be_bytes());
    let mut p = header_off + 8;
    for ptr in ptrs { page[p..p+2].copy_from_slice(&ptr.to_be_bytes()); p += 2; }
    page
}
fn interior_page(children: &[(u32, i64)], rightmost: u32) -> [u8; PAGE] {
    // children: (left_child_page, key = largest rowid in that child); rightmost = last child page
    let mut page = [0u8; PAGE];
    let mut cells: Vec<Vec<u8>> = Vec::new();
    for (child, key) in children {
        let mut c = Vec::new();
        c.extend_from_slice(&child.to_be_bytes());
        put_varint(&mut c, *key as u64);
        cells.push(c);
    }
    let mut content = PAGE;
    let mut ptrs = Vec::new();
    for cell in &cells { content -= cell.len(); page[content..content+cell.len()].copy_from_slice(cell); ptrs.push(content as u16); }
    page[0] = 0x05;
    page[3..5].copy_from_slice(&(cells.len() as u16).to_be_bytes());
    let cs: u16 = if cells.is_empty() { PAGE as u16 } else { content as u16 };
    page[5..7].copy_from_slice(&cs.to_be_bytes());
    page[8..12].copy_from_slice(&rightmost.to_be_bytes());
    let mut p = 12;
    for ptr in ptrs { page[p..p+2].copy_from_slice(&ptr.to_be_bytes()); p += 2; }
    page
}

fn chunk_leaves(cells: Vec<(i64, Vec<u8>)>) -> Vec<Vec<(i64, Vec<u8>)>> {
    // greedily fill leaves so header(8) + ptrs(2*n) + content stays under PAGE
    let mut leaves = Vec::new();
    let mut cur: Vec<(i64, Vec<u8>)> = Vec::new();
    let mut used = 8usize;
    for (rid, cell) in cells {
        let add = cell.len() + 2;
        if !cur.is_empty() && used + add > PAGE - 16 {
            leaves.push(std::mem::take(&mut cur));
            used = 8;
        }
        used += add;
        cur.push((rid, cell));
    }
    if !cur.is_empty() { leaves.push(cur); }
    if leaves.is_empty() { leaves.push(Vec::new()); }
    leaves
}

pub fn write_db(path: &Path, img: &DbImage) -> std::io::Result<()> {
    let mut datapages: Vec<[u8; PAGE]> = Vec::new(); // page number = 2 + index
    let mut schema_cells: Vec<Vec<u8>> = Vec::new();
    let mut schema_rowid = 1i64;

    for t in &img.tables {
        let ipk = ipk_index(&t.sql);
        // build cells (rowid-ascending)
        let mut cellvec: Vec<(i64, Vec<u8>)> = t.rows.iter().map(|(rid, vals)| {
            let (rowid, recvals): (i64, Vec<Val>) = match ipk {
                Some(i) => {
                    let rowid = match vals.get(i) { Some(Val::Int(v)) => *v, _ => *rid };
                    let mut rv = vals.clone();
                    if i < rv.len() { rv[i] = Val::Null; }
                    (rowid, rv)
                }
                None => (*rid, vals.clone()),
            };
            (rowid, (rowid, recvals))
        }).collect::<Vec<(i64, (i64, Vec<Val>))>>().into_iter().map(|(rid, (rowid, rv))| (rid, table_cell(rowid, &rv, &mut datapages))).collect();
        cellvec.sort_by_key(|(rid, _)| *rid);

        let leaves = chunk_leaves(cellvec);
        let rootpage;
        if leaves.len() == 1 {
            rootpage = 2 + datapages.len() as i64;
            let cells: Vec<Vec<u8>> = leaves[0].iter().map(|(_, c)| c.clone()).collect();
            datapages.push(leaf_page(&cells, 0));
        } else {
            // reserve interior root, then leaves
            let root_idx = datapages.len();
            rootpage = 2 + root_idx as i64;
            datapages.push([0u8; PAGE]); // placeholder
            let mut children: Vec<(u32, i64)> = Vec::new();
            let mut leaf_nos: Vec<u32> = Vec::new();
            let mut maxkeys: Vec<i64> = Vec::new();
            for leaf in &leaves {
                let no = (2 + datapages.len()) as u32;
                let cells: Vec<Vec<u8>> = leaf.iter().map(|(_, c)| c.clone()).collect();
                datapages.push(leaf_page(&cells, 0));
                leaf_nos.push(no);
                maxkeys.push(leaf.last().map(|(rid, _)| *rid).unwrap_or(0));
            }
            for i in 0..leaf_nos.len() - 1 { children.push((leaf_nos[i], maxkeys[i])); }
            let rightmost = *leaf_nos.last().unwrap();
            datapages[root_idx] = interior_page(&children, rightmost);
        }
        let rec = vec![Val::Text("table".into()), Val::Text(t.name.clone()),
                       Val::Text(t.name.clone()), Val::Int(rootpage), Val::Text(t.sql.clone())];
        schema_cells.push(table_cell(schema_rowid, &rec, &mut datapages));
        schema_rowid += 1;
    }
    for ix in &img.indexes {
        let mut entries = ix.entries.clone();
        entries.sort_by(|(ka, ra), (kb, rb)| {
            for i in 0..ka.len().min(kb.len()) {
                let o = val_ord(&ka[i], &kb[i]);
                if o != std::cmp::Ordering::Equal { return o; }
            }
            ra.cmp(rb)
        });
        let rootpage = 2 + datapages.len() as i64;
        datapages.push(index_leaf_page(&entries));
        let sqlval = match &ix.sql { Some(s) => Val::Text(s.clone()), None => Val::Null };
        let rec = vec![Val::Text("index".into()), Val::Text(ix.name.clone()),
                       Val::Text(ix.tbl.clone()), Val::Int(rootpage), sqlval];
        schema_cells.push(table_cell(schema_rowid, &rec, &mut datapages));
        schema_rowid += 1;
    }
    for tg in &img.triggers {
        let rec = vec![Val::Text("trigger".into()), Val::Text(tg.name.clone()),
                       Val::Text(tg.tbl.clone()), Val::Int(0), Val::Text(tg.sql.clone())];
        schema_cells.push(table_cell(schema_rowid, &rec, &mut datapages));
        schema_rowid += 1;
    }

    let mut page1 = leaf_page(&schema_cells, 100);
    let npages = (1 + datapages.len()) as u32;
    write_header(&mut page1, npages);
    let mut buf = Vec::with_capacity(npages as usize * PAGE);
    buf.extend_from_slice(&page1);
    for p in &datapages { buf.extend_from_slice(p); }
    std::fs::write(path, buf)
}

fn write_header(page1: &mut [u8; PAGE], npages: u32) {
    page1[0..16].copy_from_slice(HEADER);
    page1[16..18].copy_from_slice(&(PAGE as u16).to_be_bytes());
    page1[18] = 1; page1[19] = 1; page1[20] = 0; page1[21] = 64; page1[22] = 32; page1[23] = 32;
    page1[24..28].copy_from_slice(&1u32.to_be_bytes());
    page1[28..32].copy_from_slice(&npages.to_be_bytes());
    page1[40..44].copy_from_slice(&1u32.to_be_bytes()); // schema cookie
    page1[44..48].copy_from_slice(&4u32.to_be_bytes()); // schema format 4
    page1[56..60].copy_from_slice(&1u32.to_be_bytes()); // utf8
    page1[92..96].copy_from_slice(&1u32.to_be_bytes());
    page1[96..100].copy_from_slice(&3_054_000u32.to_be_bytes());
}

// ---------------- reader (leaf + interior; also parses C files) ----------------
fn read_table_pages(buf: &[u8], page_size: usize, root: usize, ipk: Option<usize>, out: &mut Vec<(i64, Vec<Val>)>) {
    if root < 1 || root * page_size > buf.len() { return; }
    let page = &buf[(root - 1) * page_size..root * page_size];
    let header_off = if root == 1 { 100 } else { 0 };
    match page[header_off] {
        0x0d => {
            let ncell = u16::from_be_bytes([page[header_off+3], page[header_off+4]]) as usize;
            for i in 0..ncell {
                let pp = header_off + 8 + i * 2;
                let off = u16::from_be_bytes([page[pp], page[pp+1]]) as usize;
                let mut pos = off;
                let paylen = get_varint(page, &mut pos) as usize;
                let rowid = get_varint(page, &mut pos) as i64;
                let local = local_of(paylen);
                let payload: Vec<u8> = if local == paylen {
                    page[pos..pos + paylen].to_vec()
                } else {
                    // reassemble the overflow chain
                    let mut data = page[pos..pos + local].to_vec();
                    let mut next = u32::from_be_bytes([page[pos+local], page[pos+local+1],
                                                       page[pos+local+2], page[pos+local+3]]) as usize;
                    while next != 0 && data.len() < paylen && next * page_size <= buf.len() {
                        let op = &buf[(next - 1) * page_size..next * page_size];
                        let take = (paylen - data.len()).min(page_size - 4);
                        data.extend_from_slice(&op[4..4 + take]);
                        next = u32::from_be_bytes([op[0], op[1], op[2], op[3]]) as usize;
                    }
                    data
                };
                let mut vals = decode_record(&payload);
                if let Some(i) = ipk { if i < vals.len() { vals[i] = Val::Int(rowid); } }
                out.push((rowid, vals));
            }
        }
        0x05 => {
            let ncell = u16::from_be_bytes([page[3], page[4]]) as usize;
            for i in 0..ncell {
                let pp = 12 + i * 2;
                let off = u16::from_be_bytes([page[pp], page[pp+1]]) as usize;
                let child = u32::from_be_bytes([page[off], page[off+1], page[off+2], page[off+3]]) as usize;
                read_table_pages(buf, page_size, child, ipk, out);
            }
            let rightmost = u32::from_be_bytes([page[8], page[9], page[10], page[11]]) as usize;
            read_table_pages(buf, page_size, rightmost, ipk, out);
        }
        _ => {}
    }
}

pub fn read_db(path: &Path) -> std::io::Result<DbImage> {
    let buf = std::fs::read(path)?;
    let mut img = DbImage::default();
    if buf.len() < 100 || &buf[0..16] != HEADER { return Ok(img); }
    let page_size = { let v = u16::from_be_bytes([buf[16], buf[17]]) as usize; if v == 1 { 65536 } else { v } };
    // schema = page 1 (may itself be interior if huge; our schemas are tiny → leaf)
    let mut schema_rows: Vec<(i64, Vec<Val>)> = Vec::new();
    read_table_pages(&buf, page_size, 1, None, &mut schema_rows);
    for (_r, rec) in schema_rows {
        if rec.len() < 5 { continue; }
        let ty = match &rec[0] { Val::Text(t) => t.clone(), _ => continue };
        let name = match &rec[1] { Val::Text(t) => t.clone(), _ => continue };
        let tbl = match &rec[2] { Val::Text(t) => t.clone(), _ => continue };
        let sql = match &rec[4] { Val::Text(t) => t.clone(), _ => String::new() };
        if ty == "table" {
            let root = match &rec[3] { Val::Int(i) => *i as usize, _ => continue };
            let ipk = ipk_index(&sql);
            let mut rows = Vec::new();
            read_table_pages(&buf, page_size, root, ipk, &mut rows);
            img.tables.push(TableImage { name, sql, rows });
        } else if ty == "trigger" {
            img.triggers.push(TriggerImage { name, tbl, sql });
        } else if ty == "index" {
            let sqlopt = if matches!(&rec[4], Val::Null) { None } else { Some(sql) };
            img.indexes.push(IndexImage { name, tbl, sql: sqlopt, entries: Vec::new() });
        }
    }
    Ok(img)
}

pub fn parse_cols(sql: &str) -> Vec<String> { cols_of(sql) }
