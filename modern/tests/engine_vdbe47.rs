//! Run-58 VDBE first-slice replay — mirrors /tmp/h58.c.
//! EXPLAIN of constant SELECTs matches C's program row-for-row, and stepping the
//! same SQL runs the program through the dispatch loop (counter anti-cheat: it
//! moves on the VM statements and NOT on a kitchen-fallback statement).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
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
        p.push(format!("tests/characterization/engine-vdbe47/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
}

#[test] fn vdbe47_explain() {
    let mut h = H::new("engine-vdbe47-001-C001");
    h.rows("x_select_1", "EXPLAIN SELECT 1");
    h.rows("x_select_42", "EXPLAIN SELECT 42");
    h.check_keep();
    h.cid = "engine-vdbe47-001-C002";
    h.rows("x_where_1", "EXPLAIN SELECT 1 WHERE 1");
    h.rows("x_where_0", "EXPLAIN SELECT 1 WHERE 0");
    h.check_keep();
    h.cid = "engine-vdbe47-001-C003";
    h.rows("x_add", "EXPLAIN SELECT 1+2");
    h.rows("x_text", "EXPLAIN SELECT 'hi'");
    h.rows("x_two_cols", "EXPLAIN SELECT 1, 2");
    h.check();
}

#[test] fn vdbe47_execute() { let _g = lockg();
    let d0 = vdbe::dispatch_count();
    let mut h = H::new("engine-vdbe47-002-C001");
    h.rows("run_1", "SELECT 1");
    h.rows("run_42", "SELECT 42");
    h.rows("run_add", "SELECT 1+2");
    h.rows("run_text", "SELECT 'hi'");
    h.rows("run_two", "SELECT 1, 2");
    h.rows("run_where_0", "SELECT 1 WHERE 0");
    h.check_keep();
    // anti-cheat: those steps DISPATCHED opcodes (not the kitchen evaluator)
    let d1 = vdbe::dispatch_count();
    assert!(d1 > d0, "dispatch counter must move on VM statements: {d0} -> {d1}");

    h.cid = "engine-vdbe47-002-C002";
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"SELECT 1".as_ptr(), -1, &mut st, ptr::null_mut());
        let r1 = sqlite3_step(st); let v1 = sqlite3_column_int64(st, 0);
        let r2 = sqlite3_step(st);
        sqlite3_reset(st);
        let r3 = sqlite3_step(st); let v3 = sqlite3_column_int64(st, 0);
        h.lines.push(format!("OBS {} step_cycle {},{} {} {},{}", h.cid, r1, v1, r2, r3, v3));
        sqlite3_finalize(st);
    }
    h.check();
}

// ---- anti-cheat: kitchen-fallback SQL must NOT move the dispatch counter ----
#[test] fn anti_cheat_vdbe47_kitchen_fallback() { let _g = lockg(); unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_exec(db, c"CREATE TABLE a(x); CREATE TABLE b(y); INSERT INTO a VALUES(1); INSERT INTO b VALUES(1);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    let d0 = vdbe::dispatch_count();
    // a join — still owned by the kitchen evaluator
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT x, y FROM a, b WHERE x = y".as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    assert_eq!(vdbe::dispatch_count(), d0, "a kitchen-fallback join must not move the dispatch counter");
    // and a VM statement DOES move it (runtime literal derived from the pid)
    let n = (std::process::id() % 1000) as i64;
    let sql = CString::new(format!("SELECT {n}")).unwrap();
    sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), n, "runtime literal must come out of the VM");
    sqlite3_finalize(st);
    assert!(vdbe::dispatch_count() > d0, "the runtime constant SELECT must dispatch");
    sqlite3_close(db);
} }
