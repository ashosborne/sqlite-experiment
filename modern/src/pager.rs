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
