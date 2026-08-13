//! Run-57 leaf-split replay — mirrors /tmp/h57.c.
//! Overflow INSERT takes the CURSOR split path (interior 0x05 root + >=2 0x0d
//! leaves; split counter moves; no dbfile whole-image fallback). The 002 case is
//! the pinned C amalgamation's read of the modern-written split file: geometry +
//! rows are re-derived from the actual file bytes, and the integrity line comes
//! from EXEC'ING the pinned C amalgamation (never the kitchen integrity walk).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

fn tmp(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("btree46_{}_{}.db", std::process::id(), name));
    p
}
fn cleanup(p: &std::path::Path) {
    let _ = std::fs::remove_file(p);
    let mut j = p.as_os_str().to_os_string(); j.push("-journal");
    let _ = std::fs::remove_file(PathBuf::from(j));
}
fn page_type(path: &std::path::Path, pgno: u64) -> i64 {
    match std::fs::read(path) {
        Ok(b) => { let off = ((pgno - 1) * 4096) as usize;
            if off < b.len() { b[off] as i64 } else { -1 } }
        Err(_) => -1,
    }
}

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3, path: PathBuf }
impl H {
    fn open(cid: &'static str, path: &std::path::Path) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path.to_str().unwrap()).unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db, path: path.to_path_buf() } } }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn oi(&mut self, label: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, label, v)); }
    fn ptype(&mut self, label: &str, pgno: u64) { let v = page_type(&self.path, pgno); self.oi(label, v); }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={}", self.cid, label, rc)); return; }
        let mut buf = String::new();
        while sqlite3_step(st) == 100 {
            if !buf.is_empty() { buf.push('|'); }
            for i in 0..sqlite3_column_count(st) {
                if i > 0 { buf.push(','); }
                let p = sqlite3_column_text(st, i as c_int);
                if p.is_null() { buf.push('~'); } else { buf.push_str(&CStr::from_ptr(p as *const c_char).to_string_lossy()); }
            }
        }
        sqlite3_finalize(st);
        self.lines.push(format!("OBS {} {} {}", self.cid, label, buf));
    } }
    fn close(&mut self) { unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } } }
    fn check_keep(&mut self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-btree46/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
}
fn insert_overflow(h: &mut H) {
    for i in 10..22 {
        let pay: String = std::iter::repeat(char::from(b'a' + (i % 26) as u8)).take(500).collect();
        h.ex(&format!("INSERT INTO t VALUES({i},'{pay}');"));
    }
}

#[test] fn btree46_split() { let _g = lockg();
    let path = tmp("split"); cleanup(&path);
    let s0 = pager::split_count();
    let mut h = H::open("engine-btree46-001-C001", &path);
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(k INTEGER PRIMARY KEY, v);");
    h.ex("INSERT INTO t VALUES(1,'aaaa'),(2,'bbbb'),(3,'cccc'),(4,'dddd');");
    h.rows("pages_small", "PRAGMA page_count;");
    h.ptype("root_type_small", 2);
    insert_overflow(&mut h);
    h.rows("pages_after_overflow", "PRAGMA page_count;");
    h.ptype("root_type_after", 2);
    h.ptype("p3_type", 3);
    h.ptype("p4_type", 4);
    h.rows("count_all", "SELECT count(*) FROM t;");
    h.rows("min_max", "SELECT min(k), max(k) FROM t;");
    h.close();
    h.check_keep();
    // anti-cheat: the overflow took the CURSOR split path, not the dbfile fallback
    assert!(pager::split_count() > s0, "split counter must move on the overflow INSERT");

    let mut h = H::open("engine-btree46-001-C002", &path);
    h.rows("reopen_count", "SELECT count(*) FROM t;");
    h.rows("reopen_rowids", "SELECT k FROM t ORDER BY k;");
    h.rows("reopen_hi", "SELECT substr(v,1,4) FROM t WHERE k=21;");
    h.rows("reopen_low", "SELECT v FROM t WHERE k=3;");
    h.close();
    h.check_keep();

    let s1 = pager::split_count();
    let mut h = H::open("engine-btree46-001-C003", &path);
    h.ex("INSERT INTO t VALUES(30,'post-split');");
    h.rows("post_split_row", "SELECT v FROM t WHERE k=30;");
    h.rows("post_split_count", "SELECT count(*) FROM t;");
    h.close();
    let mut h2 = H::open("engine-btree46-001-C003", &path);
    h2.rows("post_split_reopen", "SELECT v FROM t WHERE k=30;");
    h2.ptype("root_still_interior", 2);
    h2.close();
    h.lines.append(&mut h2.lines);
    h.check_keep();
    // the post-split write stayed on the cursor path (a re-split of the interior)
    assert!(pager::split_count() > s1, "the second write must stay on the cursor split path");
    cleanup(&path);
}

// ---- 002: the pinned C amalgamation reads the modern-written split file ----
// Geometry + rows come from parsing the real file; the integrity line comes from
// EXEC'ING the pinned C amalgamation (kitchen integrity is never used as proof).
#[test] fn btree46_c_reads_modern_file() { let _g = lockg();
    let out = std::env::temp_dir().join(format!("btree46_cproof_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&out);
    // 1. write the deterministic split db through the modern cursor path
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(out.to_str().unwrap()).unwrap().as_ptr(), &mut db);
        let run = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
        run(db, "PRAGMA journal_mode=DELETE;");
        run(db, "CREATE TABLE t(k INTEGER PRIMARY KEY, v);");
        run(db, "INSERT INTO t VALUES(1,'aaaa'),(2,'bbbb'),(3,'cccc'),(4,'dddd');");
        for i in 10..22 {
            let pay: String = std::iter::repeat(char::from(b'a' + (i % 26) as u8)).take(500).collect();
            run(db, &format!("INSERT INTO t VALUES({i},'{pay}');"));
        }
        sqlite3_close(db);
    }
    // 2. derive geometry + rows from the ACTUAL file bytes
    let img = dbfile::read_db_bytes(&std::fs::read(&out).unwrap());
    let t = img.tables.iter().find(|t| t.name == "t").expect("table t in modern file");
    let mut rowids: Vec<i64> = t.rows.iter().map(|(r, _)| *r).collect();
    rowids.sort_unstable();
    let rowid_str = rowids.iter().map(|r| r.to_string()).collect::<Vec<_>>().join("|");
    let probe = t.rows.iter().find(|(r, _)| *r == 21)
        .and_then(|(_, v)| v.get(1).cloned())
        .map(|v| match v { store::Val::Text(s) => s[..4].to_string(), _ => "?".into() })
        .expect("k=21 present");
    // 3. EXEC the pinned C amalgamation on the modern file (integrity + count)
    let (c_count, c_integrity) = exec_pinned_c(&out);
    let mut lines = Vec::new();
    let cid = "engine-btree46-002-C001";
    lines.push(format!("OBS {cid} modern_count {c_count}"));
    lines.push(format!("OBS {cid} modern_rowids {rowid_str}"));
    lines.push(format!("OBS {cid} modern_probe {probe}"));
    lines.push(format!("OBS {cid} modern_integrity {c_integrity}"));
    lines.push(format!("OBS {cid} modern_root_type {}", page_type(&out, 2)));
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
    p.push("tests/characterization/engine-btree46/cases/engine-btree46-002/C001.approved.txt");
    assert_eq!(lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "C-reads-modern golden");
    let _ = std::fs::remove_file(&out);
}

/// compile (cached) and run the pinned C amalgamation reader on `db`, returning
/// (count, integrity). Panics with a clear message if the pin is unavailable —
/// this test IS the C proof and must not silently degrade.
fn exec_pinned_c(db: &std::path::Path) -> (String, String) {
    let amal = std::path::Path::new("/tmp/sqlite-build/sqlite3.c");
    assert!(amal.exists(),
        "pinned C amalgamation not found at /tmp/sqlite-build — the C proof cannot run (see ADR 0044)");
    let bin = std::env::temp_dir().join("btree46_creader");
    if !bin.exists() {
        let src = std::env::temp_dir().join("btree46_creader.c");
        std::fs::write(&src, r#"
#include <stdio.h>
#include "sqlite3.h"
int main(int argc, char**argv){ sqlite3*db;
  if(sqlite3_open(argv[1],&db)){ printf("OPENFAIL\n"); return 1; }
  sqlite3_stmt*st; char cnt[64]="?", integ[128]="?";
  if(!sqlite3_prepare_v2(db,"SELECT count(*) FROM t",-1,&st,0)){
    if(sqlite3_step(st)==SQLITE_ROW) snprintf(cnt,sizeof cnt,"%s",sqlite3_column_text(st,0));
    sqlite3_finalize(st); }
  if(!sqlite3_prepare_v2(db,"PRAGMA integrity_check",-1,&st,0)){
    if(sqlite3_step(st)==SQLITE_ROW) snprintf(integ,sizeof integ,"%s",sqlite3_column_text(st,0));
    sqlite3_finalize(st); }
  printf("%s %s\n",cnt,integ); sqlite3_close(db); return 0; }
"#).unwrap();
        let ok = std::process::Command::new("cc")
            .args(["-O0", "-I/tmp/sqlite-build", src.to_str().unwrap(),
                   "/tmp/sqlite-build/sqlite3.c", "-o", bin.to_str().unwrap(),
                   "-lm", "-lpthread", "-ldl"])
            .status().map(|s| s.success()).unwrap_or(false);
        assert!(ok, "failed to compile the pinned C reader");
    }
    let outp = std::process::Command::new(&bin).arg(db).output().expect("run C reader");
    let text = String::from_utf8_lossy(&outp.stdout);
    let mut it = text.split_whitespace();
    (it.next().unwrap_or("?").to_string(), it.next().unwrap_or("?").to_string())
}

// ---- control: :memory: DML must not move the split counter ----
#[test] fn btree46_memory_control() { let _g = lockg();
    let s0 = pager::split_count();
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        let run = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
        run(db, "CREATE TABLE m(k INTEGER PRIMARY KEY, v);");
        for i in 0..30 {
            let pay: String = std::iter::repeat('z').take(500).collect();
            run(db, &format!("INSERT INTO m VALUES({i},'{pay}');"));
        }
        sqlite3_close(db);
    }
    assert_eq!(pager::split_count(), s0, ":memory: DML must not move the split counter");
}

// ---- anti-cheat: runtime payload in a high rowid survives the split + C reads it ----
#[test] fn anti_cheat_btree46_runtime() { let _g = lockg();
    let path = tmp("rt"); cleanup(&path);
    let payload = format!("rt{}", std::process::id());
    let s0 = pager::split_count();
    let mut h = H::open("rt", &path);
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(k INTEGER PRIMARY KEY, v);");
    insert_overflow(&mut h);
    h.ex(&format!("INSERT INTO t VALUES(99,'{payload}');"));
    h.close();
    assert!(pager::split_count() > s0, "runtime overflow must take the cursor split path");
    // reopen through modern: the runtime payload is on the split tree
    let mut h = H::open("rt", &path);
    h.rows("chk", "SELECT v FROM t WHERE k=99");
    assert_eq!(h.lines[0], format!("OBS rt chk {payload}"));
    h.close();
    // and the PINNED C amalgamation reads it from the modern-written split file
    let (_cnt, integ) = exec_pinned_c(&path);
    assert_eq!(integ, "ok", "pinned C integrity_check must pass on the runtime split file");
    cleanup(&path);
}
