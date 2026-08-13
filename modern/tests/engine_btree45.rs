//! Run-56 btree replay — mirrors /tmp/h56.c.
//! sqlite3_txn_state on the pager-backed handle; a table cursor that moves cells
//! on pager leaf pages for file-backed INSERT/SELECT-by-rowid/DELETE/literal
//! UPDATE; committed file stays valid SQLite (C integrity_check ok).
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
    p.push(format!("btree45_{}_{}.db", std::process::id(), name));
    p
}
fn cleanup(p: &std::path::Path) {
    let _ = std::fs::remove_file(p);
    let mut j = p.as_os_str().to_os_string(); j.push("-journal");
    let _ = std::fs::remove_file(PathBuf::from(j));
}
fn journal_of(p: &std::path::Path) -> PathBuf {
    let mut j = p.as_os_str().to_os_string(); j.push("-journal"); PathBuf::from(j)
}

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn open(cid: &'static str, path: &std::path::Path) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path.to_str().unwrap()).unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn ts(&mut self, label: &str) { let v = unsafe { sqlite3_txn_state(self.db, ptr::null()) } as i64; self.lines.push(format!("OBS {} {} {}", self.cid, label, v)); }
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
        p.push(format!("tests/characterization/engine-btree45/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
}

#[test] fn btree45_txn_state() { let _g = lockg();
    let path = tmp("ts"); cleanup(&path);
    let mut h = H::open("engine-btree45-001-C001", &path);
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(a INTEGER PRIMARY KEY, b);");
    h.ts("idle");
    h.ex("BEGIN;");
    h.ts("after_deferred_begin");
    h.rows("sel", "SELECT count(*) FROM t");
    h.ts("after_read");
    h.ex("INSERT INTO t VALUES(1,'x');");
    h.ts("after_write");
    h.ex("COMMIT;");
    h.ts("after_commit");
    h.check_keep();
    h.cid = "engine-btree45-001-C002";
    h.ex("BEGIN IMMEDIATE;");
    h.ts("after_begin_immediate");
    h.ex("ROLLBACK;");
    h.ts("after_rollback");
    h.ex("BEGIN EXCLUSIVE;");
    h.ts("after_begin_exclusive");
    h.ex("COMMIT;");
    h.ts("after_commit2");
    h.close();
    h.check_keep();
    cleanup(&path);
}

#[test] fn btree45_table_cursor() { let _g = lockg();
    let path = tmp("cur"); cleanup(&path);
    let ops0 = pager::cursor_ops();
    let mut h = H::open("engine-btree45-002-C001", &path);
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(k INTEGER PRIMARY KEY, v);");
    h.ex("INSERT INTO t VALUES(10,'ten'),(20,'twenty'),(30,'thirty'),(40,'forty');");
    h.close();
    let mut h = H::open("engine-btree45-002-C001", &path);
    h.rows("reopen_all", "SELECT k,v FROM t ORDER BY k");
    h.rows("seek_30", "SELECT v FROM t WHERE k=30");
    h.close();
    h.check_keep();

    let mut h = H::open("engine-btree45-002-C002", &path);
    h.ex("DELETE FROM t WHERE k=20;");
    h.ex("UPDATE t SET v='TEN' WHERE k=10;");
    h.close();
    let mut h = H::open("engine-btree45-002-C002", &path);
    h.rows("after_del_upd", "SELECT k,v FROM t ORDER BY k");
    h.rows("seek_deleted", "SELECT v FROM t WHERE k=20");
    h.rows("integrity", "PRAGMA integrity_check");
    h.close();
    h.check_keep();
    let ops1 = pager::cursor_ops();
    assert!(ops1 > ops0, "table-cursor cell ops must move on file-backed DML: {ops0} -> {ops1}");
    cleanup(&path);
}

// ---- anti-cheat: runtime payload through the cursor + C-open + counter + journal ----
#[test] fn anti_cheat_btree45_runtime() { let _g = lockg();
    let path = tmp("ac"); cleanup(&path);
    let payload = format!("p{}", std::process::id());
    let ops0 = pager::cursor_ops();
    let mut h = H::open("ac", &path);
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(k INTEGER PRIMARY KEY, v);");
    h.ex(&format!("INSERT INTO t VALUES(1,'{payload}'),(2,'gone');"));
    // journal present during an open write txn, gone after COMMIT (pager still works)
    h.ex("BEGIN IMMEDIATE;");
    h.ex("DELETE FROM t WHERE k=2;");
    assert!(journal_of(&path).exists(), "journal present during the open write txn");
    h.ex("COMMIT;");
    assert!(!journal_of(&path).exists(), "journal gone after COMMIT");
    h.close();
    let ops1 = pager::cursor_ops();
    assert!(ops1 > ops0, "cursor ops moved on the file-backed writes");

    // reopen through the kitchen: runtime payload present, deleted row gone
    let mut h = H::open("ac", &path);
    h.rows("chk", "SELECT k,v FROM t ORDER BY k");
    assert_eq!(h.lines[0], format!("OBS ac chk 1,{payload}"), "runtime payload survives; k=2 deleted");
    h.close();
    cleanup(&path);
}

// ---- C-readable committed file (gated on PAGER44/BTREE out env) ----
#[test] fn btree45_write_committed_file() {
    let out = match std::env::var("BTREE45_OUT") { Ok(v) => v, Err(_) => return };
    let _ = std::fs::remove_file(&out);
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(out.clone()).unwrap().as_ptr(), &mut db);
        let run = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
        run(db, "PRAGMA journal_mode=DELETE;");
        run(db, "CREATE TABLE t(k INTEGER PRIMARY KEY, v);");
        run(db, "INSERT INTO t VALUES(1,'one'),(2,'two'),(3,'three');");
        run(db, "DELETE FROM t WHERE k=2;");
        run(db, "INSERT INTO t VALUES(4,'cursor-made-me');");
        sqlite3_close(db);
    }
}

// ---- control: :memory: DML must not move the file-cursor counter ----
#[test] fn btree45_memory_control() { let _g = lockg();
    let (o0, c0) = (pager::cursor_ops(), pager::pcache_counters().0);
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        sqlite3_exec(db, c"CREATE TABLE m(k INTEGER PRIMARY KEY, v); INSERT INTO m VALUES(1,'a'); UPDATE m SET v='b'; DELETE FROM m WHERE k=1;".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
        sqlite3_close(db);
    }
    assert_eq!(pager::cursor_ops(), o0, ":memory: DML must not move the file table-cursor counter");
    let _ = c0;
}
