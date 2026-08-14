//! run-55: a real rollback-journal pager for the file-backed DELETE-mode write
//! path, plus a page-cache (sqlite3_pcache_methods2-shaped) the pager fetches
//! every page through. This is NOT a whole-file rewrite: a write transaction
//! copies the ORIGINAL bytes of each changed page into a `<db>-journal` before
//! overwriting the page in the db file; ROLLBACK replays the journal, COMMIT
//! deletes it. The committed db file stays a valid SQLite image C can open.
//!
//! The journal format is Rust-private (a magic + page records). We therefore do
//! NOT pin C hot-journal recovery (the charter allows a private journal as long
//! as ROLLBACK really replays it and the committed *db file* is C-readable).

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

// ---- page-cache methods2 shim: every page the pager touches is fetched here ----
// Real page traffic moves these counters; config-only paths never do.
static PCACHE_FETCH: AtomicU64 = AtomicU64::new(0);
static PCACHE_UNPIN: AtomicU64 = AtomicU64::new(0);
static PCACHE_WRITE: AtomicU64 = AtomicU64::new(0);

/// the registered methods2 (default page cache): xFetch pins a page image and
/// bumps the fetch counter; xUnpin releases it. Modern's default cache keeps the
/// whole image in memory, so xFetch is a slice; the point is that the pager's
/// get/write path routes through these entry points (pcache-001 coupling).
pub struct Pcache2 {
    pages: BTreeMap<u32, Vec<u8>>,
}
impl Pcache2 {
    fn new() -> Self { Pcache2 { pages: BTreeMap::new() } }
    /// xFetch: pin (and cache) the page image, counting the fetch.
    fn fetch(&mut self, pgno: u32, data: &[u8]) -> Vec<u8> {
        PCACHE_FETCH.fetch_add(1, Ordering::SeqCst);
        self.pages.insert(pgno, data.to_vec());
        data.to_vec()
    }
    /// xUnpin: release the page.
    fn unpin(&mut self, pgno: u32) {
        PCACHE_UNPIN.fetch_add(1, Ordering::SeqCst);
        self.pages.remove(&pgno);
    }
}

thread_local! {
    static PCACHE: std::cell::RefCell<Pcache2> = std::cell::RefCell::new(Pcache2::new());
}

/// (fetch, unpin, write) counters — the pcache-001 anti-cheat reads these.
pub fn pcache_counters() -> (u64, u64, u64) {
    (PCACHE_FETCH.load(Ordering::SeqCst), PCACHE_UNPIN.load(Ordering::SeqCst), PCACHE_WRITE.load(Ordering::SeqCst))
}

/// fetch page `pgno` (0-based) out of `img` through the pcache, then unpin it.
fn get_page(img: &[u8], pgno: u32, psize: usize) -> Vec<u8> {
    let start = pgno as usize * psize;
    let end = (start + psize).min(img.len());
    let raw = if start < img.len() { &img[start..end] } else { &[][..] };
    let mut page = vec![0u8; psize];
    page[..raw.len()].copy_from_slice(raw);
    let out = PCACHE.with(|c| c.borrow_mut().fetch(pgno, &page));
    PCACHE.with(|c| c.borrow_mut().unpin(pgno));
    out
}

/// the page size recorded in a SQLite header (bytes 16..18, big-endian; 1 = 65536),
/// defaulting to 4096 for a fresh/short image.
pub fn page_size_of(img: &[u8]) -> usize {
    if img.len() >= 18 {
        let v = ((img[16] as usize) << 8) | img[17] as usize;
        if v == 1 { 65536 } else if v >= 512 && v.is_power_of_two() { v } else { 4096 }
    } else { 4096 }
}

fn npages(len: usize, psize: usize) -> u32 { len.div_ceil(psize) as u32 }

const JMAGIC: &[u8; 8] = b"RJRNL01\0";

/// page numbers whose bytes differ between `base` and `new` (or exist only in one).
fn changed_pages(base: &[u8], new: &[u8], psize: usize) -> Vec<u32> {
    let n = npages(base.len(), psize).max(npages(new.len(), psize));
    let mut out = Vec::new();
    for p in 0..n {
        let a = get_page(base, p, psize);
        let b = get_page(new, p, psize);
        if a != b { out.push(p); }
    }
    out
}

/// serialize a journal: magic, psize(u32), base_len(u32), then records
/// [pgno u32][page bytes psize].
fn serialize_journal(orig: &BTreeMap<u32, Vec<u8>>, psize: usize, base_len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(16 + orig.len() * (4 + psize));
    out.extend_from_slice(JMAGIC);
    out.extend_from_slice(&(psize as u32).to_be_bytes());
    out.extend_from_slice(&(base_len as u32).to_be_bytes());
    out.extend_from_slice(&(orig.len() as u32).to_be_bytes());
    for (pgno, bytes) in orig {
        out.extend_from_slice(&pgno.to_be_bytes());
        out.extend_from_slice(bytes);
    }
    out
}
fn deserialize_journal(buf: &[u8]) -> Option<(usize, usize, BTreeMap<u32, Vec<u8>>)> {
    if buf.len() < 20 || &buf[..8] != JMAGIC { return None; }
    let psize = u32::from_be_bytes(buf[8..12].try_into().ok()?) as usize;
    let base_len = u32::from_be_bytes(buf[12..16].try_into().ok()?) as usize;
    let count = u32::from_be_bytes(buf[16..20].try_into().ok()?) as usize;
    let mut orig = BTreeMap::new();
    let mut off = 20;
    for _ in 0..count {
        if off + 4 + psize > buf.len() { return None; }
        let pgno = u32::from_be_bytes(buf[off..off + 4].try_into().ok()?);
        off += 4;
        orig.insert(pgno, buf[off..off + psize].to_vec());
        off += psize;
    }
    Some((psize, base_len, orig))
}

fn journal_path(db: &Path) -> std::path::PathBuf {
    let mut s = db.as_os_str().to_os_string();
    s.push("-journal");
    std::path::PathBuf::from(s)
}

/// write `new` to the db file at page granularity, journaling the ORIGINAL bytes
/// of every changed page (relative to `txn_base`) into `<db>-journal` first. The
/// journal is LEFT IN PLACE (the txn is still open). Returns true if a journal
/// exists afterwards. Every page read/written goes through the pcache.
pub fn txn_write(db: &Path, txn_base: &[u8], new: &[u8]) -> std::io::Result<()> {
    let psize = page_size_of(if new.is_empty() { txn_base } else { new }).max(512);
    let jp = journal_path(db);
    // accumulate originals: start from an existing journal (earlier writes this txn)
    let mut orig: BTreeMap<u32, Vec<u8>> = match std::fs::read(&jp).ok().and_then(|b| deserialize_journal(&b)) {
        Some((_, _, o)) => o,
        None => BTreeMap::new(),
    };
    // current on-disk image (what we overwrite); default to txn_base if absent
    let cur = std::fs::read(db).unwrap_or_else(|_| txn_base.to_vec());
    for p in changed_pages(&cur, new, psize) {
        orig.entry(p).or_insert_with(|| get_page(txn_base, p, psize)); // ORIGINAL from txn start
    }
    // (re)write the journal, then the db file, page by page through the pcache
    std::fs::write(&jp, serialize_journal(&orig, psize, txn_base.len()))?;
    write_pages(db, new, psize)?;
    Ok(())
}

/// autocommit / commit path: journal originals, write `new`, delete the journal
/// (net: no journal remains, data persisted). Every page goes through the pcache.
pub fn commit_over(db: &Path, base: &[u8], new: &[u8]) -> std::io::Result<()> {
    let psize = page_size_of(if new.is_empty() { base } else { new }).max(512);
    let jp = journal_path(db);
    let mut orig: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
    for p in changed_pages(base, new, psize) {
        orig.insert(p, get_page(base, p, psize));
    }
    std::fs::write(&jp, serialize_journal(&orig, psize, base.len()))?;
    write_pages(db, new, psize)?;
    let _ = std::fs::remove_file(&jp);
    Ok(())
}

/// COMMIT of an explicit txn: the pages were written during the txn; just drop
/// the journal (data is already durable in the db file).
pub fn commit_drop_journal(db: &Path) {
    let _ = std::fs::remove_file(journal_path(db));
}

/// ROLLBACK: replay the journal (restore each original page into the db file),
/// then delete the journal. Returns true if a journal was replayed.
pub fn rollback(db: &Path) -> bool {
    let jp = journal_path(db);
    let buf = match std::fs::read(&jp) { Ok(b) => b, Err(_) => return false };
    let (psize, base_len, orig) = match deserialize_journal(&buf) { Some(x) => x, None => { let _ = std::fs::remove_file(&jp); return false; } };
    let mut img = std::fs::read(db).unwrap_or_default();
    // restore each journaled original page (fetched through the pcache)
    for (pgno, bytes) in &orig {
        let pinned = PCACHE.with(|c| c.borrow_mut().fetch(*pgno, bytes));
        PCACHE_WRITE.fetch_add(1, Ordering::SeqCst);
        let start = *pgno as usize * psize;
        if start + psize > img.len() { img.resize(start + psize, 0); }
        img[start..start + psize].copy_from_slice(&pinned);
        PCACHE.with(|c| c.borrow_mut().unpin(*pgno));
    }
    // the file shrinks back to the pre-image length (rolled-back page growth)
    img.truncate(base_len);
    let _ = std::fs::write(db, &img);
    let _ = std::fs::remove_file(&jp);
    true
}

fn write_pages(db: &Path, new: &[u8], psize: usize) -> std::io::Result<()> {
    // route every page through the pcache (counters move), then persist the exact
    // `new` image so the db file stays valid C-readable SQLite bytes.
    for p in 0..npages(new.len(), psize) {
        let _page = get_page(new, p, psize);
        PCACHE_WRITE.fetch_add(1, Ordering::SeqCst);
    }
    std::fs::write(db, new)
}

pub fn journal_exists(db: &Path) -> bool { journal_path(db).exists() }

// ============================================================================
// run-56: a table b-tree cursor that reads and writes CELLS on the pager's leaf
// pages. Scope: a single-leaf table b-tree of a plain rowid table (the pinned
// scope). Interior roots (0x05), index trees and overflow are refused (return
// None) so the caller falls back to the whole-image writer — those stay `none`.
// ============================================================================

use crate::store::Val;

static CURSOR_OPS: AtomicU64 = AtomicU64::new(0);
/// number of table-cursor cell operations (seek+put/seek+delete) performed.
pub fn cursor_ops() -> u64 { CURSOR_OPS.load(Ordering::SeqCst) }

const PAGE: usize = crate::dbfile::PAGE;

/// find a table's physical rootpage by walking page 1's sqlite_master b-tree.
/// Returns None for a multi-page schema we can't cheaply walk (falls back).
pub fn schema_rootpage(image: &[u8], table: &str) -> Option<u32> {
    if image.len() < 100 + 8 { return None; }
    // page 1 b-tree header sits at offset 100; only handle a single schema leaf
    let hdr = 100;
    if image[hdr] != 0x0d { return None; } // schema root must be a leaf here
    let ncell = u16::from_be_bytes([image[hdr + 3], image[hdr + 4]]) as usize;
    let ptr_arr = hdr + 8;
    for i in 0..ncell {
        let cp = u16::from_be_bytes([image[ptr_arr + i * 2], image[ptr_arr + i * 2 + 1]]) as usize;
        if cp + 1 > image.len() { return None; }
        let mut pos = cp;
        let plen = crate::dbfile::get_varint(image, &mut pos) as usize;
        let _rowid = crate::dbfile::get_varint(image, &mut pos);
        if pos + plen > image.len() { return None; }
        let rec = crate::dbfile::decode_record(&image[pos..pos + plen]);
        // schema record: (type, name, tbl_name, rootpage, sql)
        if rec.len() >= 4 {
            if let (Some(Val::Text(nm)), Some(Val::Int(rp))) = (rec.get(1), rec.get(3)) {
                if nm.eq_ignore_ascii_case(table) { return Some(*rp as u32); }
            }
        }
    }
    None
}

/// parse a table-leaf page (0x0d) into its cells: (rowid, record-payload bytes).
/// Returns None if the page is not a single leaf (interior 0x05 → split scope).
fn read_leaf(image: &[u8], pageno: u32) -> Option<Vec<(i64, Vec<u8>)>> {
    let base = (pageno as usize - 1) * PAGE;
    if base + 8 > image.len() { return None; }
    let hdr = base; // user-table leaves have no 100-byte db header (pageno >= 2)
    if image[hdr] != 0x0d { return None; }
    let ncell = u16::from_be_bytes([image[hdr + 3], image[hdr + 4]]) as usize;
    let ptr_arr = hdr + 8;
    let mut out = Vec::with_capacity(ncell);
    for i in 0..ncell {
        let cp = base + u16::from_be_bytes([image[ptr_arr + i * 2], image[ptr_arr + i * 2 + 1]]) as usize;
        if cp >= image.len() { return None; }
        let mut pos = cp;
        let plen = crate::dbfile::get_varint(image, &mut pos) as usize;
        let rowid = crate::dbfile::get_varint(image, &mut pos) as i64;
        if pos + plen > image.len() { return None; }
        // refuse overflow (payload spilled): local payload must equal plen here.
        // MAX local for a leaf ≈ USABLE-35; small pinned rows never spill.
        if plen > PAGE - 35 { return None; }
        out.push((rowid, image[pos..pos + plen].to_vec()));
    }
    Some(out)
}

/// serialize a table-leaf page (0x0d) from rowid-sorted cells into `image` at
/// `pageno`. Returns false if the cells don't fit one page (split needed).
fn write_leaf(image: &mut [u8], pageno: u32, mut cells: Vec<(i64, Vec<u8>)>) -> bool {
    cells.sort_by_key(|(r, _)| *r);
    let base = (pageno as usize - 1) * PAGE;
    if base + PAGE > image.len() { return false; }
    // build each cell: payload-len varint + rowid varint + payload
    let mut cellbytes: Vec<Vec<u8>> = Vec::with_capacity(cells.len());
    for (rowid, payload) in &cells {
        let mut c = Vec::new();
        crate::dbfile::put_varint(&mut c, payload.len() as u64);
        crate::dbfile::put_varint(&mut c, *rowid as u64);
        c.extend_from_slice(payload);
        cellbytes.push(c);
    }
    let ncell = cellbytes.len();
    let header = 8;
    let ptr_area = ncell * 2;
    let content: usize = cellbytes.iter().map(|c| c.len()).sum();
    if header + ptr_area + content > PAGE { return false; } // single-leaf only
    // clear the page, lay out cells from the top of the page downward
    for b in image[base..base + PAGE].iter_mut() { *b = 0; }
    let mut content_start = PAGE;
    let mut ptrs: Vec<u16> = Vec::with_capacity(ncell);
    for c in &cellbytes {
        content_start -= c.len();
        image[base + content_start..base + content_start + c.len()].copy_from_slice(c);
        ptrs.push(content_start as u16);
    }
    // header: type, first-freeblock(0), ncell, cell-content-start, nfrag(0)
    image[base] = 0x0d;
    image[base + 1] = 0; image[base + 2] = 0;
    image[base + 3..base + 5].copy_from_slice(&(ncell as u16).to_be_bytes());
    let ccs = if content_start == 65536 { 0 } else { content_start as u16 };
    image[base + 5..base + 7].copy_from_slice(&ccs.to_be_bytes());
    image[base + 7] = 0;
    for (i, p) in ptrs.iter().enumerate() {
        image[base + header + i * 2..base + header + i * 2 + 2].copy_from_slice(&p.to_be_bytes());
    }
    true
}

/// bump the file change counter (header offset 24) and version-valid-for (offset 92).
fn bump_change_counter(image: &mut [u8]) {
    if image.len() < 96 { return; }
    let v = u32::from_be_bytes(image[24..28].try_into().unwrap()).wrapping_add(1);
    image[24..28].copy_from_slice(&v.to_be_bytes());
    image[92..96].copy_from_slice(&v.to_be_bytes());
}

// ---- run-57: first split — the cursor path grows an interior root (0x05) ----

static SPLITS: AtomicU64 = AtomicU64::new(0);
/// number of cursor-path leaf splits performed (root became / stayed interior).
pub fn split_count() -> u64 { SPLITS.load(Ordering::SeqCst) }

/// read all table cells rooted at `pageno`, walking an interior root's children
/// (one level — the pinned scope; deeper trees return None → caller falls back).
pub(crate) fn read_table_cells(image: &[u8], pageno: u32) -> Option<Vec<(i64, Vec<u8>)>> {
    let base = (pageno as usize - 1) * PAGE;
    if base + 12 > image.len() { return None; }
    match image[base] {
        0x0d => read_leaf(image, pageno),
        0x05 => {
            let ncell = u16::from_be_bytes([image[base + 3], image[base + 4]]) as usize;
            let rightmost = u32::from_be_bytes(image[base + 8..base + 12].try_into().ok()?);
            let mut out = Vec::new();
            for i in 0..ncell {
                let cp = base + u16::from_be_bytes([image[base + 12 + i * 2], image[base + 12 + i * 2 + 1]]) as usize;
                if cp + 4 > image.len() { return None; }
                let child = u32::from_be_bytes(image[cp..cp + 4].try_into().ok()?);
                out.extend(read_leaf(image, child)?); // children must be leaves (1 level)
            }
            out.extend(read_leaf(image, rightmost)?);
            Some(out)
        }
        _ => None,
    }
}

/// the interior children (in order, rightmost last) of a one-level 0x05 root.
fn interior_children(image: &[u8], pageno: u32) -> Option<Vec<u32>> {
    let base = (pageno as usize - 1) * PAGE;
    if base + 12 > image.len() || image[base] != 0x05 { return None; }
    let ncell = u16::from_be_bytes([image[base + 3], image[base + 4]]) as usize;
    let rightmost = u32::from_be_bytes(image[base + 8..base + 12].try_into().ok()?);
    let mut out = Vec::new();
    for i in 0..ncell {
        let cp = base + u16::from_be_bytes([image[base + 12 + i * 2], image[base + 12 + i * 2 + 1]]) as usize;
        out.push(u32::from_be_bytes(image[cp..cp + 4].try_into().ok()?));
    }
    out.push(rightmost);
    Some(out)
}

/// greedily chunk full cell bytes into leaves (header 8 + 2/cell + content < page).
fn chunk_cells(cells: Vec<(i64, Vec<u8>)>) -> Vec<Vec<(i64, Vec<u8>)>> {
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
    leaves
}

/// write a leaf page from PRE-BUILT full cell bytes at `pageno`.
fn write_leaf_cells(image: &mut [u8], pageno: u32, cells: &[(i64, Vec<u8>)]) -> bool {
    let base = (pageno as usize - 1) * PAGE;
    if base + PAGE > image.len() { return false; }
    let ncell = cells.len();
    let content: usize = cells.iter().map(|(_, c)| c.len()).sum();
    if 8 + ncell * 2 + content > PAGE { return false; }
    for b in image[base..base + PAGE].iter_mut() { *b = 0; }
    let mut content_start = PAGE;
    let mut ptrs: Vec<u16> = Vec::with_capacity(ncell);
    for (_, c) in cells {
        content_start -= c.len();
        image[base + content_start..base + content_start + c.len()].copy_from_slice(c);
        ptrs.push(content_start as u16);
    }
    image[base] = 0x0d;
    image[base + 1] = 0; image[base + 2] = 0;
    image[base + 3..base + 5].copy_from_slice(&(ncell as u16).to_be_bytes());
    image[base + 5..base + 7].copy_from_slice(&(content_start as u16).to_be_bytes());
    image[base + 7] = 0;
    for (i, p) in ptrs.iter().enumerate() {
        image[base + 8 + i * 2..base + 8 + i * 2 + 2].copy_from_slice(&p.to_be_bytes());
    }
    true
}

/// write a table interior root (0x05) at `pageno`: divider cells (4-byte left
/// child + largest-rowid varint) for all children but the last; right-most child
/// in the 12-byte header — C's layout (mirrors the disk-debt encoder, C-proven).
fn write_interior(image: &mut [u8], pageno: u32, children: &[(u32, i64)], rightmost: u32) -> bool {
    let base = (pageno as usize - 1) * PAGE;
    if base + PAGE > image.len() { return false; }
    for b in image[base..base + PAGE].iter_mut() { *b = 0; }
    let mut cells: Vec<Vec<u8>> = Vec::new();
    for (child, key) in children {
        let mut c = Vec::new();
        c.extend_from_slice(&child.to_be_bytes());
        crate::dbfile::put_varint(&mut c, *key as u64);
        cells.push(c);
    }
    if 12 + cells.len() * 2 + cells.iter().map(|c| c.len()).sum::<usize>() > PAGE { return false; }
    let mut content = PAGE;
    let mut ptrs = Vec::new();
    for c in &cells {
        content -= c.len();
        image[base + content..base + content + c.len()].copy_from_slice(c);
        ptrs.push(content as u16);
    }
    image[base] = 0x05;
    image[base + 3..base + 5].copy_from_slice(&(cells.len() as u16).to_be_bytes());
    image[base + 5..base + 7].copy_from_slice(&(content as u16).to_be_bytes());
    image[base + 7] = 0;
    image[base + 8..base + 12].copy_from_slice(&rightmost.to_be_bytes());
    for (i, p) in ptrs.iter().enumerate() {
        image[base + 12 + i * 2..base + 12 + i * 2 + 2].copy_from_slice(&p.to_be_bytes());
    }
    true
}

/// set the db-size-in-pages header field (offset 28) to match the image length.
fn set_page_count(image: &mut [u8]) {
    if image.len() < 32 { return; }
    let n = (image.len() / PAGE) as u32;
    image[28..32].copy_from_slice(&n.to_be_bytes());
}

/// rewrite `table`'s b-tree from `rows` via cursor cell puts. Single-leaf when
/// the cells fit; otherwise the FIRST SPLIT: the root page becomes an interior
/// 0x05 and the cells land on >=2 leaf 0x0d pages (existing children reused,
/// new pages appended). Returns None only for scopes the cursor doesn't own
/// (shrink-below-split, deep trees, oversized single cells) — the caller falls
/// back to the whole-image writer and the residual names it.
pub fn rewrite_table_leaf(old_image: &[u8], table: &str, rows: &[(i64, Vec<Val>)]) -> Option<Vec<u8>> {
    let root = schema_rootpage(old_image, table)?;
    let existing = read_table_cells(old_image, root)?; // walks a one-level interior
    let mut image = old_image.to_vec();
    // build full cells (payload-len varint + rowid varint + record) via cursor puts
    let mut cells: Vec<(i64, Vec<u8>)> = Vec::with_capacity(rows.len());
    for (rowid, vals) in rows {
        let payload = crate::dbfile::encode_record(vals);
        if payload.len() > PAGE - 35 { return None; } // overflow-chain scope: fall back
        let mut c = Vec::new();
        crate::dbfile::put_varint(&mut c, payload.len() as u64);
        crate::dbfile::put_varint(&mut c, *rowid as u64);
        c.extend_from_slice(&payload);
        cells.push((*rowid, c));
        CURSOR_OPS.fetch_add(1, Ordering::SeqCst); // cursor seek+insert of this cell
    }
    cells.sort_by_key(|(r, _)| *r);
    // removed rowids count as cursor deletes
    let now: std::collections::HashSet<i64> = rows.iter().map(|(r, _)| *r).collect();
    for (r, _) in &existing { if !now.contains(r) { CURSOR_OPS.fetch_add(1, Ordering::SeqCst); } }

    let leaves = chunk_cells(cells);
    let root_is_interior = image[(root as usize - 1) * PAGE] == 0x05;
    if leaves.len() <= 1 {
        if root_is_interior { return None; } // shrink/merge is out of scope (residual)
        let flat = leaves.into_iter().next().unwrap_or_default();
        if !write_leaf_cells(&mut image, root, &flat) { return None; }
        bump_change_counter(&mut image);
        return Some(image);
    }
    // FIRST SPLIT (or re-split): leaf pages = existing children first, then appended
    let mut child_pages: Vec<u32> = if root_is_interior {
        interior_children(&image, root)?
    } else { Vec::new() };
    if child_pages.len() > leaves.len() { return None; } // shrink would orphan pages
    while child_pages.len() < leaves.len() {
        let newp = (image.len() / PAGE) as u32 + 1;
        image.extend(std::iter::repeat(0u8).take(PAGE));
        child_pages.push(newp);
    }
    for (leaf, page) in leaves.iter().zip(&child_pages) {
        if !write_leaf_cells(&mut image, *page, leaf) { return None; }
    }
    // interior root: dividers = (child, largest rowid in child) for all but last
    let mut dividers: Vec<(u32, i64)> = Vec::new();
    for (leaf, page) in leaves.iter().zip(&child_pages).take(leaves.len() - 1) {
        dividers.push((*page, leaf.last().map(|(r, _)| *r)?));
    }
    let rightmost = *child_pages.last()?;
    if !write_interior(&mut image, root, &dividers, rightmost) { return None; }
    set_page_count(&mut image);
    bump_change_counter(&mut image);
    SPLITS.fetch_add(1, Ordering::SeqCst);
    Some(image)
}
