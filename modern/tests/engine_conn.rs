//! Run-36 connection-lifecycle replay — mirrors /tmp/conn_harness.c, asserted
//! byte-identical against the frozen C goldens. Plain language: a connection
//! won't close while statements live; close_v2 waits; busy handlers sleep and
//! retry; hooks fire on commit / row change / trace events.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lock() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn open(cid: &'static str, path: &str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn new(cid: &'static str) -> H { H::open(cid, ":memory:") }
    fn ex(&mut self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn exr(&mut self, label: &str, s: &str) { unsafe {
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let e = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={}", self.cid, label, rc, e));
        sqlite3_free(em as *mut c_void);
    } }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn os(&mut self, n: &str, v: &str) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 {
            let e = CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy().into_owned();
            self.lines.push(format!("OBS {} {} prep.rc={} err={}", self.cid, label, rc, e));
            return;
        }
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
    fn finish(self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
    fn check(self) { unsafe { sqlite3_close(self.db); } self.finish() }
}
fn fresh(path: &str) { for sfx in ["", "-wal", "-shm", "-journal"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
    std::fs::create_dir_all("/tmp/eftest").unwrap(); }
unsafe fn errline(h: &mut H, label: &str, rc: c_int, db: *mut Sqlite3) {
    let e = CStr::from_ptr(sqlite3_errmsg(db)).to_string_lossy().into_owned();
    h.lines.push(format!("OBS {} {} rc={} err={}", h.cid, label, rc, e));
}

#[test] fn a001() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-001-C001");
    h.ex("CREATE TABLE t(a);");
    let db = h.db; h.db = ptr::null_mut();
    h.oi("close", sqlite3_close(db) as i64);
    h.oi("close_null", sqlite3_close(ptr::null_mut()) as i64);
    h.oi("close_v2_null", sqlite3_close_v2(ptr::null_mut()) as i64);
    h.finish();
} }

#[test] fn a002() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-001-C002");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(h.db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    let dbp = h.db;
    let rc = sqlite3_close(dbp);
    errline(&mut h, "close", rc, dbp);
    h.oi("finalize", sqlite3_finalize(st) as i64);
    let db = h.db; h.db = ptr::null_mut();
    h.oi("close2", sqlite3_close(db) as i64);
    h.finish();
} }

#[test] fn a003() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-001-C003");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(7);");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(h.db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    h.oi("step", sqlite3_step(st) as i64);
    let db = h.db; h.db = ptr::null_mut();
    h.oi("close_v2", sqlite3_close_v2(db) as i64);
    h.oi("col_after", sqlite3_column_int(st, 0) as i64);
    h.oi("finalize", sqlite3_finalize(st) as i64); // completes the zombie teardown
    h.finish();
} }

#[test] fn a004() { let _g = lock(); unsafe {
    let path = "/tmp/eftest/rw_conn_a4.db";
    fresh(path);
    let mut h = H::open("engine-conn-001-C004", path);
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    h.ex("BEGIN; INSERT INTO t VALUES(2);");
    let db = h.db;
    h.db = ptr::null_mut();
    h.oi("close", sqlite3_close(db) as i64);
    let mut db2: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db2);
    h.db = db2;
    h.rows("reopen", "SELECT count(*), max(a) FROM t");
    h.check();
} }

#[test] fn a005() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-001-C005");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2);");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(h.db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    let dbp = h.db;
    let rc = sqlite3_close(dbp);
    errline(&mut h, "close", rc, dbp);
    sqlite3_reset(st);
    let rc = sqlite3_close(dbp);
    errline(&mut h, "close_after_reset", rc, dbp);
    h.oi("finalize", sqlite3_finalize(st) as i64);
    let db = h.db; h.db = ptr::null_mut();
    h.oi("close2", sqlite3_close(db) as i64);
    h.finish();
} }

#[test] fn a006() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-001-C006");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'414141');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    sqlite3_blob_open(h.db, c"main".as_ptr(), c"t".as_ptr(), c"d".as_ptr(), 1, 0, &mut b);
    let dbp = h.db;
    let rc = sqlite3_close(dbp);
    errline(&mut h, "close", rc, dbp);
    h.oi("blob_close", sqlite3_blob_close(b) as i64);
    let db = h.db; h.db = ptr::null_mut();
    h.oi("close2", sqlite3_close(db) as i64);
    h.finish();
} }

// ---- busy ----
static BCALLS: Mutex<i64> = Mutex::new(0);
static BFIRST: Mutex<i64> = Mutex::new(-1);
unsafe extern "C" fn busy0(_a: *mut c_void, n: c_int) -> c_int {
    *BCALLS.lock().unwrap() += 1;
    let mut f = BFIRST.lock().unwrap();
    if *f < 0 { *f = n as i64; }
    0
}
unsafe extern "C" fn busy2(_a: *mut c_void, n: c_int) -> c_int {
    *BCALLS.lock().unwrap() += 1;
    let mut f = BFIRST.lock().unwrap();
    if *f < 0 { *f = n as i64; }
    (n < 2) as c_int
}
unsafe fn open2(path: &str) -> (*mut Sqlite3, *mut Sqlite3) {
    let (mut d1, mut d2): (*mut Sqlite3, *mut Sqlite3) = (ptr::null_mut(), ptr::null_mut());
    sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut d1);
    sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut d2);
    (d1, d2)
}
unsafe fn ex_on(db: *mut Sqlite3, s: &str) { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); }
unsafe fn exr_on(h: &mut H, db: *mut Sqlite3, label: &str, s: &str) {
    let mut em: *mut c_char = ptr::null_mut();
    let rc = sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
    let e = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
    h.lines.push(format!("OBS {} {} rc={} err={}", h.cid, label, rc, e));
    sqlite3_free(em as *mut c_void);
}

#[test] fn b001() { let _g = lock(); unsafe {
    let path = "/tmp/eftest/rw_conn_b1.db"; fresh(path);
    let (d1, d2) = open2(path);
    let mut h = H { cid: "engine-conn-002-C001", lines: Vec::new(), db: ptr::null_mut() };
    ex_on(d1, "CREATE TABLE t(a);");
    ex_on(d1, "BEGIN IMMEDIATE; INSERT INTO t VALUES(1);");
    exr_on(&mut h, d2, "blocked", "INSERT INTO t VALUES(2);");
    ex_on(d1, "COMMIT;");
    exr_on(&mut h, d2, "after_commit", "INSERT INTO t VALUES(3);");
    h.db = d2;
    h.rows("cnt", "SELECT count(*) FROM t");
    h.db = ptr::null_mut();
    sqlite3_close(d1); sqlite3_close(d2);
    h.finish();
} }

#[test] fn b002() { let _g = lock(); unsafe {
    let path = "/tmp/eftest/rw_conn_b2.db"; fresh(path);
    let (d1, d2) = open2(path);
    let mut h = H { cid: "engine-conn-002-C002", lines: Vec::new(), db: ptr::null_mut() };
    ex_on(d1, "CREATE TABLE t(a);");
    *BCALLS.lock().unwrap() = 0; *BFIRST.lock().unwrap() = -1;
    sqlite3_busy_handler(d2, Some(busy0), ptr::null_mut());
    ex_on(d1, "BEGIN IMMEDIATE; INSERT INTO t VALUES(1);");
    exr_on(&mut h, d2, "blocked", "INSERT INTO t VALUES(2);");
    h.oi("calls", *BCALLS.lock().unwrap());
    h.oi("first_count", *BFIRST.lock().unwrap());
    sqlite3_close(d1); sqlite3_close(d2);
    h.finish();
} }

#[test] fn b003() { let _g = lock(); unsafe {
    let path = "/tmp/eftest/rw_conn_b3.db"; fresh(path);
    let (d1, d2) = open2(path);
    let mut h = H { cid: "engine-conn-002-C003", lines: Vec::new(), db: ptr::null_mut() };
    ex_on(d1, "CREATE TABLE t(a);");
    *BCALLS.lock().unwrap() = 0; *BFIRST.lock().unwrap() = -1;
    sqlite3_busy_handler(d2, Some(busy2), ptr::null_mut());
    ex_on(d1, "BEGIN IMMEDIATE; INSERT INTO t VALUES(1);");
    exr_on(&mut h, d2, "blocked", "INSERT INTO t VALUES(2);");
    h.oi("calls", *BCALLS.lock().unwrap());
    h.oi("first_count", *BFIRST.lock().unwrap());
    sqlite3_close(d1); sqlite3_close(d2);
    h.finish();
} }

#[test] fn b004() { let _g = lock(); unsafe {
    let path = "/tmp/eftest/rw_conn_b4.db"; fresh(path);
    let (d1, d2) = open2(path);
    let mut h = H { cid: "engine-conn-002-C004", lines: Vec::new(), db: ptr::null_mut() };
    ex_on(d1, "CREATE TABLE t(a);");
    *BCALLS.lock().unwrap() = 0; *BFIRST.lock().unwrap() = -1;
    sqlite3_busy_handler(d2, Some(busy0), ptr::null_mut());
    h.oi("timeout_rc", sqlite3_busy_timeout(d2, 20) as i64);
    ex_on(d1, "BEGIN IMMEDIATE; INSERT INTO t VALUES(1);");
    exr_on(&mut h, d2, "blocked", "INSERT INTO t VALUES(2);");
    h.oi("handler_calls", *BCALLS.lock().unwrap());
    sqlite3_close(d1); sqlite3_close(d2);
    h.finish();
} }

#[test] fn b005() { let _g = lock(); unsafe {
    let path = "/tmp/eftest/rw_conn_b5.db"; fresh(path);
    let (d1, d2) = open2(path);
    let mut h = H { cid: "engine-conn-002-C005", lines: Vec::new(), db: ptr::null_mut() };
    ex_on(d1, "CREATE TABLE t(a);");
    sqlite3_busy_timeout(d2, 5000);
    h.oi("clear_rc", sqlite3_busy_timeout(d2, 0) as i64);
    ex_on(d1, "BEGIN IMMEDIATE; INSERT INTO t VALUES(1);");
    exr_on(&mut h, d2, "blocked", "INSERT INTO t VALUES(2);");
    sqlite3_close(d1); sqlite3_close(d2);
    h.finish();
} }

#[test] fn b006() { let _g = lock(); unsafe {
    let path = "/tmp/eftest/rw_conn_b6.db"; fresh(path);
    let mut d1: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut d1);
    let mut h = H { cid: "engine-conn-002-C006", lines: Vec::new(), db: ptr::null_mut() };
    *BCALLS.lock().unwrap() = 0;
    sqlite3_busy_handler(d1, Some(busy0), ptr::null_mut());
    exr_on(&mut h, d1, "ins", "CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    h.oi("calls", *BCALLS.lock().unwrap());
    sqlite3_close(d1);
    h.finish();
} }

// ---- hooks / trace ----
static HLOG: Mutex<String> = Mutex::new(String::new());
static HCOUNT: Mutex<i64> = Mutex::new(0);
unsafe extern "C" fn upd_cb(_a: *mut c_void, op: c_int, db: *const c_char, tbl: *const c_char, rowid: i64) {
    let mut l = HLOG.lock().unwrap();
    let sep = if l.is_empty() { "" } else { " " };
    l.push_str(&format!("{sep}[{} {} {} {}]", op,
        CStr::from_ptr(db).to_string_lossy(), CStr::from_ptr(tbl).to_string_lossy(), rowid));
    *HCOUNT.lock().unwrap() += 1;
}
static CHCALLS: Mutex<i64> = Mutex::new(0);
static CHRET: Mutex<i32> = Mutex::new(0);
unsafe extern "C" fn commit_cb(_a: *mut c_void) -> c_int { *CHCALLS.lock().unwrap() += 1; *CHRET.lock().unwrap() }
static CH2: Mutex<i64> = Mutex::new(0);
unsafe extern "C" fn commit_cb2(_a: *mut c_void) -> c_int { *CH2.lock().unwrap() += 1; 0 }
static TLOG: Mutex<String> = Mutex::new(String::new());
static TROW: Mutex<i64> = Mutex::new(0);
static TCLOSE: Mutex<i64> = Mutex::new(0);
static TPROFILE: Mutex<i64> = Mutex::new(0);
unsafe extern "C" fn trace_cb(ty: u32, _ctx: *mut c_void, _p: *mut c_void, x: *mut c_void) -> c_int {
    match ty {
        1 => { let mut l = TLOG.lock().unwrap();
               let sep = if l.is_empty() { "" } else { " " };
               l.push_str(&format!("{sep}{{{}}}", CStr::from_ptr(x as *const c_char).to_string_lossy())); }
        4 => *TROW.lock().unwrap() += 1,
        8 => *TCLOSE.lock().unwrap() += 1,
        2 => *TPROFILE.lock().unwrap() += 1,
        _ => {}
    }
    0
}

#[test] fn h001() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C001");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, v TEXT);");
    HLOG.lock().unwrap().clear(); *HCOUNT.lock().unwrap() = 0;
    sqlite3_update_hook(h.db, Some(upd_cb), ptr::null_mut());
    h.ex("INSERT INTO t VALUES(5,'a');");
    h.ex("UPDATE t SET v='b' WHERE id=5;");
    h.ex("DELETE FROM t WHERE id=5;");
    let l = HLOG.lock().unwrap().clone();
    h.os("log", &l);
    h.oi("count", *HCOUNT.lock().unwrap());
    h.check();
} }

#[test] fn h002() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C002");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, v TEXT);");
    HLOG.lock().unwrap().clear(); *HCOUNT.lock().unwrap() = 0;
    sqlite3_update_hook(h.db, Some(upd_cb), 0x1234usize as *mut c_void);
    let old = sqlite3_update_hook(h.db, None, ptr::null_mut());
    h.oi("old_is_arg", (old == 0x1234usize as *mut c_void) as i64);
    h.ex("INSERT INTO t VALUES(1,'x');");
    h.oi("count", *HCOUNT.lock().unwrap());
    h.check();
} }

#[test] fn h003() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C003");
    h.ex("CREATE TABLE t(a);");
    *CHCALLS.lock().unwrap() = 0; *CHRET.lock().unwrap() = 0;
    sqlite3_commit_hook(h.db, Some(commit_cb), ptr::null_mut());
    h.ex("INSERT INTO t VALUES(1);");
    h.ex("INSERT INTO t VALUES(2);");
    h.ex("BEGIN; INSERT INTO t VALUES(3); INSERT INTO t VALUES(4); COMMIT;");
    h.oi("calls", *CHCALLS.lock().unwrap());
    h.rows("cnt", "SELECT count(*) FROM t");
    h.check();
} }

#[test] fn h004() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C004");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    *CHCALLS.lock().unwrap() = 0; *CHRET.lock().unwrap() = 1;
    sqlite3_commit_hook(h.db, Some(commit_cb), ptr::null_mut());
    h.exr("begin", "BEGIN;");
    h.exr("ins", "INSERT INTO t VALUES(2);");
    h.exr("commit", "COMMIT;");
    h.oi("autocommit", sqlite3_get_autocommit(h.db) as i64);
    *CHRET.lock().unwrap() = 0;
    h.rows("cnt", "SELECT count(*) FROM t");
    h.check();
} }

#[test] fn h005() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C005");
    h.ex("CREATE TABLE t(a);");
    *CHCALLS.lock().unwrap() = 0; *CH2.lock().unwrap() = 0; *CHRET.lock().unwrap() = 0;
    sqlite3_commit_hook(h.db, Some(commit_cb), 0x77usize as *mut c_void);
    let old = sqlite3_commit_hook(h.db, Some(commit_cb2), ptr::null_mut());
    h.oi("old_is_arg", (old == 0x77usize as *mut c_void) as i64);
    h.ex("INSERT INTO t VALUES(1);");
    h.oi("first_calls", *CHCALLS.lock().unwrap());
    h.oi("second_calls", *CH2.lock().unwrap());
    h.check();
} }

#[test] fn h006() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C006");
    TLOG.lock().unwrap().clear();
    sqlite3_trace_v2(h.db, 1, Some(trace_cb), ptr::null_mut());
    h.ex("CREATE TABLE t(a);");
    h.ex("INSERT INTO t VALUES(42);");
    let l = TLOG.lock().unwrap().clone();
    h.os("log", &l);
    h.check();
} }

#[test] fn h007() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C007");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(3);");
    *TROW.lock().unwrap() = 0;
    sqlite3_trace_v2(h.db, 4, Some(trace_cb), ptr::null_mut());
    h.rows("q", "SELECT a FROM t");
    h.oi("row_events", *TROW.lock().unwrap());
    h.check();
} }

#[test] fn h008() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C008");
    *TCLOSE.lock().unwrap() = 0;
    sqlite3_trace_v2(h.db, 8, Some(trace_cb), ptr::null_mut());
    h.ex("CREATE TABLE t(a);");
    h.oi("pre_close", *TCLOSE.lock().unwrap());
    let db = h.db; h.db = ptr::null_mut();
    sqlite3_close(db);
    h.oi("post_close", *TCLOSE.lock().unwrap());
    h.finish();
} }

#[test] fn h009() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C009");
    TLOG.lock().unwrap().clear(); *TROW.lock().unwrap() = 0;
    sqlite3_trace_v2(h.db, 1, Some(trace_cb), ptr::null_mut());
    h.ex("CREATE TABLE t(a);");
    sqlite3_trace_v2(h.db, 0, None, ptr::null_mut());
    h.ex("INSERT INTO t VALUES(1);");
    let l = TLOG.lock().unwrap().clone();
    h.os("log", &l);
    h.check();
} }

#[test] fn h010() { let _g = lock(); unsafe {
    let mut h = H::new("engine-conn-003-C010");
    *TPROFILE.lock().unwrap() = 0;
    sqlite3_trace_v2(h.db, 2, Some(trace_cb), ptr::null_mut());
    h.ex("CREATE TABLE t(a);");
    h.ex("INSERT INTO t VALUES(1);");
    h.oi("profile_events", *TPROFILE.lock().unwrap());
    h.check();
} }

// ---- MANDATORY anti-cheat: live stmt tracking + hooks over runtime data ----
#[test] fn anti_cheat_conn_runtime() { let _g = lock(); unsafe {
    let seed = (std::process::id() % 661 + 17) as i64;
    let tname = format!("ct{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, &format!("CREATE TABLE {tname}(id INTEGER PRIMARY KEY, v INT);"));
    // close BUSY then finalize then close OK: proves live statement tracking
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT v FROM {tname}")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_close(db), 5);
    assert_eq!(sqlite3_finalize(st), 0);
    // update_hook observes the pid-seeded table + rowid
    HLOG.lock().unwrap().clear(); *HCOUNT.lock().unwrap() = 0;
    sqlite3_update_hook(db, Some(upd_cb), ptr::null_mut());
    exs(db, &format!("INSERT INTO {tname} VALUES({seed}, 1);"));
    let l = HLOG.lock().unwrap().clone();
    assert_eq!(l, format!("[18 main {tname} {seed}]"));
    // commit_hook abort leaves a runtime value out of the table
    *CHRET.lock().unwrap() = 1;
    sqlite3_commit_hook(db, Some(commit_cb), ptr::null_mut());
    exs(db, &format!("BEGIN; INSERT INTO {tname} VALUES({}, 2);", seed + 1));
    let mut em: *mut c_char = ptr::null_mut();
    let rc = sqlite3_exec(db, c"COMMIT;".as_ptr(), None, ptr::null_mut(), &mut em);
    assert_eq!(rc, 19);
    sqlite3_free(em as *mut c_void);
    *CHRET.lock().unwrap() = 0;
    sqlite3_commit_hook(db, None, ptr::null_mut());
    sqlite3_update_hook(db, None, ptr::null_mut());
    let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT count(*) FROM {tname}")).unwrap().as_ptr(), -1, &mut st2, ptr::null_mut());
    sqlite3_step(st2);
    assert_eq!(sqlite3_column_int(st2, 0), 1, "aborted commit must leave only the first row");
    sqlite3_finalize(st2);
    assert_eq!(sqlite3_close(db), 0);
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
