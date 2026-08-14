//! Run-60 WHERE-on-the-cursor replay — mirrors /tmp/h60.c.
//! EXPLAIN of SELECT ... WHERE col <op> lit matches C's program (the inverted
//! jump-to-Next compare: = Ne / <> Eq / > Le / < Ge / >= Lt / <= Gt, BINARY-8
//! p5=84, init-section Integer literal; Variable for ?, SeekRowid for rowid=N)
//! and stepping the same SQL runs the compare AS AN OPCODE over btree cells.
//! Anti-cheat: dispatch + cursor-read move on the WHERE scan (the cursor
//! positions on REJECTED rows too), and neither moves on a kitchen join or on
//! SELECT length(b) FROM t.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

const DDL: &str = "CREATE TABLE t(a INTEGER, b TEXT);\
    INSERT INTO t VALUES(1,'one');INSERT INTO t VALUES(2,'two');\
    INSERT INTO t VALUES(1,'uno');INSERT INTO t VALUES(3,'three');";

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
        p.push(format!("tests/characterization/engine-vdbe49/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
}

#[test] fn vdbe49_explain() { let _g = lockg();
    let mut h = H::fresh("engine-vdbe49-001-C001", "/tmp/v49a.db", DDL);
    h.rows("x_eq", "EXPLAIN SELECT b FROM t WHERE a=1");
    h.check_keep();
    h.cid = "engine-vdbe49-001-C002";
    h.rows("x_eq_ab", "EXPLAIN SELECT a, b FROM t WHERE a=1");
    h.rows("x_eq_nomatch", "EXPLAIN SELECT b FROM t WHERE a=999");
    h.check_keep();
    h.cid = "engine-vdbe49-001-C003";
    h.rows("x_ne", "EXPLAIN SELECT b FROM t WHERE a<>1");
    h.rows("x_gt", "EXPLAIN SELECT b FROM t WHERE a>1");
    h.rows("x_lt", "EXPLAIN SELECT b FROM t WHERE a<2");
    h.rows("x_ge", "EXPLAIN SELECT b FROM t WHERE a>=2");
    h.rows("x_le", "EXPLAIN SELECT b FROM t WHERE a<=1");
    h.check_keep();
    h.cid = "engine-vdbe49-001-C004";
    h.rows("x_bind", "EXPLAIN SELECT b FROM t WHERE a=?");
    h.rows("x_rowid", "EXPLAIN SELECT b FROM t WHERE rowid=2");
    h.check();
}

#[test] fn vdbe49_execute() { let _g = lockg();
    let (d0, c0) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    let mut h = H::fresh("engine-vdbe49-002-C001", "/tmp/v49b.db", DDL);
    h.rows("run_eq", "SELECT b FROM t WHERE a=1");
    h.rows("run_eq_ab", "SELECT a, b FROM t WHERE a=1");
    h.check_keep();
    assert!(vdbe::dispatch_count() > d0, "the WHERE scan must dispatch");
    assert!(vdbe::cursor_read_count() >= c0 + 8, "each WHERE scan must position on all 4 cells (2 scans)");

    h.cid = "engine-vdbe49-002-C002";
    let c1 = vdbe::cursor_read_count();
    h.rows("run_nomatch", "SELECT b FROM t WHERE a=999");
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"SELECT b FROM t WHERE a=999".as_ptr(), -1, &mut st, ptr::null_mut());
        h.lines.push(format!("OBS {} nomatch_first_step_rc {}", h.cid, sqlite3_step(st)));
        sqlite3_finalize(st);
    }
    h.check_keep();
    // the compare is not a Rust filter: a no-match WHERE still walked every cell
    assert!(vdbe::cursor_read_count() >= c1 + 8, "the no-match scans must still position on all 4 cells each");

    h.cid = "engine-vdbe49-002-C003";
    h.rows("run_ne", "SELECT b FROM t WHERE a<>1");
    h.rows("run_gt", "SELECT b FROM t WHERE a>1");
    h.rows("run_lt", "SELECT b FROM t WHERE a<2");
    h.rows("run_ge", "SELECT b FROM t WHERE a>=2");
    h.rows("run_le", "SELECT b FROM t WHERE a<=1");
    h.check_keep();

    h.cid = "engine-vdbe49-002-C004";
    let c2 = vdbe::cursor_read_count();
    h.rows("run_rowid", "SELECT b FROM t WHERE rowid=2");
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"SELECT b FROM t WHERE a=?".as_ptr(), -1, &mut st, ptr::null_mut());
        sqlite3_bind_int64(st, 1, 3);
        let mut buf = String::new();
        while sqlite3_step(st) == 100 {
            if !buf.is_empty() { buf.push('/'); }
            buf.push_str(&CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy());
        }
        sqlite3_finalize(st);
        h.lines.push(format!("OBS {} bound_eq_3 {}", h.cid, buf));
    }
    h.check_keep();
    assert!(vdbe::cursor_read_count() > c2, "seek + bound scan must touch cells");

    h.cid = "engine-vdbe49-002-C005";
    let mut hc = H::fresh("engine-vdbe49-002-C005", "/tmp/v49c.db", DDL);
    hc.rows("pre", "SELECT b FROM t WHERE a=1");
    hc.ex("INSERT INTO t VALUES(1,'eins');");
    hc.rows("post", "SELECT b FROM t WHERE a=1");
    hc.check();
    drop(h);
}

// ---- anti-cheat: kitchen boundaries hold; v48 unfiltered scan unchanged ----

#[test] fn anti_cheat_vdbe49_boundaries() { let _g = lockg(); unsafe {
    let path = "/tmp/v49ac.db";
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(format!("{path}-journal"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
    sqlite3_exec(db, CString::new(DDL).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    let mut jdb: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut jdb);
    sqlite3_exec(jdb, c"CREATE TABLE a(x); CREATE TABLE b(y); INSERT INTO a VALUES(1); INSERT INTO b VALUES(1);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());

    // kitchen join: neither counter moves
    let (d0, c0) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(jdb, c"SELECT x, y FROM a, b WHERE x = y".as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    assert_eq!(vdbe::dispatch_count(), d0, "a kitchen join must not dispatch");
    assert_eq!(vdbe::cursor_read_count(), c0, "a kitchen join must not touch the cursor");

    // select-list expression: still kitchen (v48 boundary pin)
    let (d1, c1) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    sqlite3_prepare_v2(db, c"SELECT length(b) FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    assert_eq!(vdbe::dispatch_count(), d1, "length(b) must stay kitchen");
    assert_eq!(vdbe::cursor_read_count(), c1, "length(b) must not touch the cursor");

    // v48 unfiltered scan still rides the VM
    let (d2, c2) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    sqlite3_prepare_v2(db, c"SELECT b FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    let mut n = 0;
    while sqlite3_step(st) == 100 { n += 1; }
    sqlite3_finalize(st);
    assert_eq!(n, 4);
    assert!(vdbe::dispatch_count() > d2 && vdbe::cursor_read_count() >= c2 + 4, "the v48 scan must still ride the VM");

    sqlite3_close(db);
    sqlite3_close(jdb);
} }

#[test] fn anti_cheat_vdbe49_runtime_payload() { let _g = lockg(); unsafe {
    // a (key, payload) pair in NO frozen SQL literal must round-trip through
    // Column + the Ne compare — the result cannot be canned.
    let seed = (std::process::id() % 90000) as i64 + 200_000;
    let path = format!("/tmp/v49rt-{seed}.db");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{path}-journal"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path.as_str()).unwrap().as_ptr(), &mut db);
    sqlite3_exec(db, CString::new(format!(
        "CREATE TABLE t(a INTEGER, b TEXT); INSERT INTO t VALUES(7,'decoy'); INSERT INTO t VALUES({seed},'p{seed}');"
    )).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());

    let (d0, c0) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let sql = CString::new(format!("SELECT b FROM t WHERE a={seed}")).unwrap();
    sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    let b = CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy().to_string();
    assert_eq!(b, format!("p{seed}"), "runtime payload must come from the matched cell");
    assert_eq!(sqlite3_step(st), 101);
    sqlite3_finalize(st);
    assert!(vdbe::dispatch_count() > d0, "runtime WHERE must dispatch");
    assert!(vdbe::cursor_read_count() >= c0 + 2, "the scan must position on the decoy cell too");
    sqlite3_close(db);
    let _ = std::fs::remove_file(&path);
} }
