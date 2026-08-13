//! Run-55 rollback-journal pager replay — mirrors /tmp/h55.c.
//! File-backed DELETE-mode: <db>-journal present during a write txn and gone
//! after COMMIT/ROLLBACK; ROLLBACK replays the journal to restore pre-images;
//! COMMIT persists a valid db file. Plus pcache methods2 coupling (xFetch/xUnpin
//! move under real page traffic) and runtime anti-cheats.
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
    p.push(format!("pager44_{}_{}.db", std::process::id(), name));
    p
}
fn cleanup(p: &std::path::Path) {
    let _ = std::fs::remove_file(p);
    let mut j = p.as_os_str().to_os_string(); j.push("-journal");
    let _ = std::fs::remove_file(PathBuf::from(j));
    let mut w = p.as_os_str().to_os_string(); w.push("-wal");
    let _ = std::fs::remove_file(PathBuf::from(w));
}
fn journal_of(p: &std::path::Path) -> PathBuf {
    let mut j = p.as_os_str().to_os_string(); j.push("-journal"); PathBuf::from(j)
}

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3, path: PathBuf }
impl H {
    fn open(cid: &'static str, path: PathBuf) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path.to_str().unwrap()).unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db, path } } }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn oi(&mut self, label: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, label, v)); }
    fn journal_here(&mut self, label: &str) { let e = journal_of(&self.path).exists() as i64; self.oi(label, e); }
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
        p.push(format!("tests/characterization/engine-pager44/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
}

#[test] fn pager44_lifecycle() { let _g = lockg();
    let path = tmp("life"); cleanup(&path);
    let mut h = H::open("engine-pager44-001-C001", path.clone());
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(a INTEGER PRIMARY KEY, b);");
    h.ex("INSERT INTO t VALUES(1,'one'),(2,'two'),(3,'three');");
    h.ex("BEGIN IMMEDIATE;");
    h.ex("INSERT INTO t VALUES(4,'four');");
    h.journal_here("journal_during_txn");
    h.ex("COMMIT;");
    h.journal_here("journal_after_commit");
    h.rows("after_commit_rows", "SELECT b FROM t ORDER BY a");
    h.ex("BEGIN IMMEDIATE;");
    h.ex("INSERT INTO t VALUES(5,'five');");
    h.journal_here("journal_during_txn2");
    h.ex("ROLLBACK;");
    h.journal_here("journal_after_rollback");
    h.rows("after_rollback_rows", "SELECT b FROM t ORDER BY a");
    h.close();
    h.check_keep();

    let mut h = H::open("engine-pager44-001-C002", path.clone());
    h.rows("reopen_rows", "SELECT a,b FROM t ORDER BY a");
    h.close();
    h.check_keep();
    cleanup(&path);
}

#[test] fn pager44_restore_persist() { let _g = lockg();
    let path = tmp("rp"); cleanup(&path);
    let mut h = H::open("engine-pager44-002-C001", path.clone());
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(a INTEGER PRIMARY KEY, b);");
    h.ex("INSERT INTO t VALUES(1,'one'),(2,'two'),(3,'three'),(4,'four');");
    h.ex("BEGIN IMMEDIATE;"); h.ex("UPDATE t SET b='ONE' WHERE a=1;"); h.ex("ROLLBACK;");
    h.rows("upd_rollback", "SELECT b FROM t WHERE a=1");
    h.ex("BEGIN IMMEDIATE;"); h.ex("DELETE FROM t WHERE a=2;"); h.ex("ROLLBACK;");
    h.rows("del_rollback", "SELECT b FROM t WHERE a=2");
    h.ex("BEGIN IMMEDIATE;"); h.ex("UPDATE t SET b='X';"); h.ex("ROLLBACK;");
    h.rows("multi_rollback", "SELECT b FROM t ORDER BY a");
    h.close();
    h.check_keep();

    let mut h = H::open("engine-pager44-002-C002", path.clone());
    h.ex("BEGIN IMMEDIATE;"); h.ex("UPDATE t SET b='cc';"); h.ex("COMMIT;");
    h.rows("multi_commit", "SELECT b FROM t ORDER BY a");
    h.close();
    let mut h2 = H::open("engine-pager44-002-C002", path.clone());
    h2.rows("reopen_after_commit", "SELECT a,b FROM t ORDER BY a");
    h2.close();
    // fold the reopen observation into the same case buffer, then check
    h.lines.push(h2.lines.remove(0));
    h.check_keep();
    cleanup(&path);
}

// ---- pcache-001 coupling: page get/write goes through methods2 (counters move) ----
#[test] fn pager44_pcache_coupling() { let _g = lockg();
    let path = tmp("pc"); cleanup(&path);
    let (f0, u0, w0) = pager::pcache_counters();
    let mut h = H::open("pcache", path.clone());
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(a INTEGER PRIMARY KEY, b);");
    h.ex("INSERT INTO t VALUES(1,'one'),(2,'two');");
    h.ex("BEGIN IMMEDIATE;");
    h.ex("UPDATE t SET b='z' WHERE a=1;");
    h.ex("COMMIT;");
    h.close();
    let (f1, u1, w1) = pager::pcache_counters();
    assert!(f1 > f0, "xFetch must move under file-txn page traffic: {f0} -> {f1}");
    assert!(u1 > u0, "xUnpin must move: {u0} -> {u1}");
    assert!(w1 > w0, "page writes must move: {w0} -> {w1}");

    // control: a :memory: db (no pager file) must NOT move the file-pager counters
    let (f2, u2, _) = pager::pcache_counters();
    unsafe {
        let mut mdb: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut mdb);
        sqlite3_exec(mdb, c"CREATE TABLE m(x); INSERT INTO m VALUES(1); UPDATE m SET x=2;".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
        sqlite3_close(mdb);
    }
    let (f3, u3, _) = pager::pcache_counters();
    assert_eq!((f3, u3), (f2, u2), ":memory: traffic must not move the file-pager pcache counters");
    cleanup(&path);
}

// ---- anti-cheat: runtime payload survives COMMIT + reopen; runtime UPDATE rolls back ----
#[test] fn anti_cheat_pager44_runtime() { let _g = lockg();
    let path = tmp("ac"); cleanup(&path);
    let payload = format!("p{}", std::process::id());
    let mut h = H::open("ac", path.clone());
    h.ex("PRAGMA journal_mode=DELETE;");
    h.ex("CREATE TABLE t(a INTEGER PRIMARY KEY, b);");
    h.ex(&format!("INSERT INTO t VALUES(1,'{payload}');"));
    // COMMIT then reopen returns the runtime payload
    h.ex("BEGIN IMMEDIATE;"); h.ex("INSERT INTO t VALUES(2,'seed');"); h.ex("COMMIT;");
    h.close();
    let mut h = H::open("ac", path.clone());
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"SELECT b FROM t WHERE a=1".as_ptr(), -1, &mut st, ptr::null_mut());
        assert_eq!(sqlite3_step(st), 100);
        let got = CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy().into_owned();
        assert_eq!(got, payload, "runtime payload must survive COMMIT + reopen");
        sqlite3_finalize(st);
    }
    // runtime UPDATE then ROLLBACK restores the pre-update (runtime) value, not the new one
    h.ex("BEGIN IMMEDIATE;");
    h.ex("UPDATE t SET b='CLOBBERED' WHERE a=1;");
    h.ex("ROLLBACK;");
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"SELECT b FROM t WHERE a=1".as_ptr(), -1, &mut st, ptr::null_mut());
        assert_eq!(sqlite3_step(st), 100);
        let got = CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy().into_owned();
        assert_eq!(got, payload, "ROLLBACK must restore the pre-update runtime value");
        sqlite3_finalize(st);
    }
    // and the journal is gone after the rollback
    assert!(!journal_of(&path).exists(), "journal must be gone after ROLLBACK");
    h.close();
    cleanup(&path);
}
