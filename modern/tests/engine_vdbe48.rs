//! Run-59 table-scan bytecode replay — mirrors /tmp/h59.c.
//! EXPLAIN of SELECT col(s) FROM t matches C's program (real root page, schema
//! cookie, column-count hint) and stepping the same SQL walks btree cells
//! through OpenRead/Rewind/Column/Next in the dispatch loop. Anti-cheat: the
//! dispatch AND cursor-read counters move on the scan; neither the join
//! (kitchen) nor SELECT 1 (v47 constant path, no cursor) moves the cursor-read
//! counter; Column output round-trips a runtime payload that is not in any SQL
//! literal frozen here.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

const DDL: &str = "CREATE TABLE t(a INTEGER, b TEXT);\
    INSERT INTO t VALUES(10,'ten');INSERT INTO t VALUES(20,'twenty');\
    INSERT INTO t VALUES(30,'thirty');";

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn fresh(cid: &'static str, path: &str, ddl: &str) -> H { unsafe {
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(format!("{path}-journal"));
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        if !ddl.is_empty() {
            sqlite3_exec(db, CString::new(ddl).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
        }
        H { cid, lines: Vec::new(), db } } }
    fn reopen(&mut self, path: &str) { unsafe {
        sqlite3_close(self.db);
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        self.db = db; } }
    fn ex(&mut self, sql: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(sql).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={}", self.cid, label, rc)); return; }
        let mut buf = String::new();
        while sqlite3_step(st) == 100 {
            if !buf.is_empty() { buf.push('/'); }
            for i in 0..sqlite3_column_count(st) {
                if i > 0 { buf.push('|'); }
                let p = sqlite3_column_text(st, i as c_int);
                if p.is_null() { buf.push('~'); } else { buf.push_str(&CStr::from_ptr(p as *const c_char).to_string_lossy()); }
            }
        }
        sqlite3_finalize(st);
        self.lines.push(format!("OBS {} {} {}", self.cid, label, buf));
    } }
    fn check_keep(&mut self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-vdbe48/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
}

#[test] fn vdbe48_explain() { let _g = lockg();
    let mut h = H::fresh("engine-vdbe48-001-C001", "/tmp/v48a.db", DDL);
    h.rows("rootpage", "SELECT rootpage FROM sqlite_master WHERE name='t'");
    h.rows("x_a", "EXPLAIN SELECT a FROM t");
    h.check_keep();
    h.cid = "engine-vdbe48-001-C002";
    h.rows("x_ab", "EXPLAIN SELECT a, b FROM t");
    h.rows("x_b", "EXPLAIN SELECT b FROM t");
    h.check_keep();
    h.cid = "engine-vdbe48-001-C003";
    let mut he = H::fresh("engine-vdbe48-001-C003", "/tmp/v48e.db", "CREATE TABLE e(a INTEGER, b TEXT);");
    he.rows("rootpage_e", "SELECT rootpage FROM sqlite_master WHERE name='e'");
    he.rows("x_e", "EXPLAIN SELECT a FROM e");
    he.check();
    h.cid = "engine-vdbe48-001-C004";
    h.reopen("/tmp/v48a.db");
    h.rows("x_a_reopen", "EXPLAIN SELECT a FROM t");
    h.check();
}

#[test] fn vdbe48_execute() { let _g = lockg();
    let d0 = vdbe::dispatch_count();
    let c0 = vdbe::cursor_read_count();
    let mut h = H::fresh("engine-vdbe48-002-C001", "/tmp/v48b.db", DDL);
    h.rows("run_a", "SELECT a FROM t");
    h.rows("run_ab", "SELECT a, b FROM t");
    h.rows("run_b", "SELECT b FROM t");
    h.check_keep();
    assert!(vdbe::dispatch_count() > d0, "the scan must dispatch opcodes");
    assert!(vdbe::cursor_read_count() > c0, "the scan must position on btree cells");

    h.cid = "engine-vdbe48-002-C002";
    let mut he = H::fresh("engine-vdbe48-002-C002", "/tmp/v48f.db", "CREATE TABLE e(a INTEGER, b TEXT);");
    he.rows("run_e", "SELECT a FROM e");
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(he.db, c"SELECT a FROM e".as_ptr(), -1, &mut st, ptr::null_mut());
        he.lines.push(format!("OBS {} empty_step_rc {}", he.cid, sqlite3_step(st)));
        sqlite3_finalize(st);
    }
    he.check();

    h.cid = "engine-vdbe48-002-C003";
    let mut hc = H::fresh("engine-vdbe48-002-C003", "/tmp/v48c.db", DDL);
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(hc.db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
        let mut buf = String::new();
        for _ in 0..4 {
            let rc = sqlite3_step(st);
            if !buf.is_empty() { buf.push('/'); }
            if rc == 100 { buf.push_str(&format!("{rc}:{}", sqlite3_column_int64(st, 0))); }
            else { buf.push_str(&rc.to_string()); }
        }
        sqlite3_reset(st);
        let rc = sqlite3_step(st);
        hc.lines.push(format!("OBS {} cycle {} reset_step {}:{}", hc.cid, buf, rc, sqlite3_column_int64(st, 0)));
        sqlite3_finalize(st);
    }
    hc.check();

    h.cid = "engine-vdbe48-002-C004";
    let mut hd = H::fresh("engine-vdbe48-002-C004", "/tmp/v48d.db", DDL);
    hd.rows("pre", "SELECT a FROM t");
    hd.ex("INSERT INTO t VALUES(40,'forty');");
    hd.rows("post", "SELECT a FROM t");
    hd.rows("post_ab", "SELECT a, b FROM t");
    hd.check();
    drop(h);
}

// ---- anti-cheat: counters, kitchen fallback, runtime payload through Column ----

#[test] fn anti_cheat_vdbe48_counters() { let _g = lockg(); unsafe {
    // scan db (single table, file-backed)
    let path = "/tmp/v48ac.db";
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(format!("{path}-journal"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
    sqlite3_exec(db, CString::new(DDL).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    // join db (two tables => kitchen)
    let mut jdb: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut jdb);
    sqlite3_exec(jdb, c"CREATE TABLE a(x); CREATE TABLE b(y); INSERT INTO a VALUES(1); INSERT INTO b VALUES(1);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());

    // 1) kitchen join: NEITHER counter moves
    let (d0, c0) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(jdb, c"SELECT x, y FROM a, b WHERE x = y".as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    assert_eq!(vdbe::dispatch_count(), d0, "a kitchen join must not dispatch");
    assert_eq!(vdbe::cursor_read_count(), c0, "a kitchen join must not touch the cursor");

    // 2) constant SELECT 1 (v47 path): dispatch moves, cursor-read does NOT
    let (d1, c1) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    sqlite3_prepare_v2(db, c"SELECT 1".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    sqlite3_finalize(st);
    assert!(vdbe::dispatch_count() > d1, "SELECT 1 dispatches (v47)");
    assert_eq!(vdbe::cursor_read_count(), c1, "SELECT 1 has no cursor — the read counter must not move");

    // 3) the scan: BOTH counters move
    let (d2, c2) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    sqlite3_prepare_v2(db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    let mut n = 0;
    while sqlite3_step(st) == 100 { n += 1; }
    sqlite3_finalize(st);
    assert_eq!(n, 3);
    assert!(vdbe::dispatch_count() > d2, "the scan must dispatch");
    assert!(vdbe::cursor_read_count() >= c2 + 3, "the scan must position on each of the 3 cells");

    sqlite3_close(db);
    sqlite3_close(jdb);
} }

#[test] fn anti_cheat_vdbe48_runtime_payload() { let _g = lockg(); unsafe {
    // a value that is in NO SQL literal of any frozen case must round-trip
    // through OP_Column (cell payload decode), proving the result is not canned.
    let seed = (std::process::id() % 90000) as i64 + 100_000;
    let path = format!("/tmp/v48rt-{seed}.db");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{path}-journal"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path.as_str()).unwrap().as_ptr(), &mut db);
    sqlite3_exec(db, CString::new(format!(
        "CREATE TABLE t(a INTEGER, b TEXT); INSERT INTO t VALUES({seed},'p{seed}');"
    )).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());

    let (d0, c0) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT a, b FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed, "runtime integer must come from the cell");
    let b = CStr::from_ptr(sqlite3_column_text(st, 1) as *const c_char).to_string_lossy().to_string();
    assert_eq!(b, format!("p{seed}"), "runtime text must come from the cell");
    assert_eq!(sqlite3_step(st), 101);
    sqlite3_finalize(st);
    assert!(vdbe::dispatch_count() > d0 && vdbe::cursor_read_count() > c0,
        "the runtime round-trip must go through the dispatch loop and the cursor");
    sqlite3_close(db);
    let _ = std::fs::remove_file(&path);
} }
