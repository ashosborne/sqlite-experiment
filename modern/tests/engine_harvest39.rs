//! Run-49 mixed one-hole replay — mirrors /tmp/h39.c.
//! Per-statement STMT/PROFILE, pragma_module_list lazy contract, per-row blob expiry
//! (+ TEXT-cell writes), randomness/test_control PRNG predicates.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }
static TLOG: Mutex<String> = Mutex::new(String::new());
static NSTMT: Mutex<i64> = Mutex::new(0);
static NPROF: Mutex<i64> = Mutex::new(0);

unsafe extern "C" fn trace_cb(t: u32, _c: *mut c_void, p: *mut c_void, x: *mut c_void) -> c_int {
    if t == 1 {
        *NSTMT.lock().unwrap_or_else(|e| e.into_inner()) += 1;
        let s = CStr::from_ptr(x as *const c_char).to_string_lossy().into_owned();
        TLOG.lock().unwrap_or_else(|e| e.into_inner()).push_str(&format!("[STMT:{s}]"));
    } else if t == 2 {
        *NPROF.lock().unwrap_or_else(|e| e.into_inner()) += 1;
        let sqlp = sqlite3_sql(p as *mut Sqlite3Stmt);
        let s = if sqlp.is_null() { String::new() } else { CStr::from_ptr(sqlp).to_string_lossy().into_owned() };
        TLOG.lock().unwrap_or_else(|e| e.into_inner()).push_str(&format!("[PROF:{s}]"));
    }
    0
}

// trivial module for module_list probing
unsafe extern "C" fn t_create(db: *mut Sqlite3, _a: *mut c_void, _argc: c_int,
        _argv: *const *const c_char, pp: *mut *mut Sqlite3Vtab, _pz: *mut *mut c_char) -> c_int {
    let rc = sqlite3_declare_vtab(db, c"CREATE TABLE x(v INTEGER)".as_ptr());
    if rc != SQLITE_OK { return rc; }
    let v = Box::new(Sqlite3Vtab { p_module: ptr::null(), n_ref: 0, z_err_msg: ptr::null_mut() });
    *pp = Box::into_raw(v);
    SQLITE_OK }
unsafe extern "C" fn t_disc(v: *mut Sqlite3Vtab) -> c_int { drop(Box::from_raw(v)); SQLITE_OK }
unsafe extern "C" fn t_best(_v: *mut Sqlite3Vtab, _i: *mut Sqlite3IndexInfo) -> c_int { SQLITE_OK }
#[repr(C)] struct TCur { base: Sqlite3VtabCursor, i: i32 }
unsafe extern "C" fn t_open(_v: *mut Sqlite3Vtab, pp: *mut *mut Sqlite3VtabCursor) -> c_int {
    let c = Box::new(TCur { base: Sqlite3VtabCursor { p_vtab: ptr::null_mut() }, i: 0 });
    *pp = Box::into_raw(c) as *mut Sqlite3VtabCursor; SQLITE_OK }
unsafe extern "C" fn t_close(c: *mut Sqlite3VtabCursor) -> c_int { drop(Box::from_raw(c as *mut TCur)); SQLITE_OK }
unsafe extern "C" fn t_filter(c: *mut Sqlite3VtabCursor, _n: c_int, _s: *const c_char,
        _ac: c_int, _av: *mut *mut Sqlite3Value) -> c_int { (*(c as *mut TCur)).i = 0; SQLITE_OK }
unsafe extern "C" fn t_next(c: *mut Sqlite3VtabCursor) -> c_int { (*(c as *mut TCur)).i += 1; SQLITE_OK }
unsafe extern "C" fn t_eof(c: *mut Sqlite3VtabCursor) -> c_int { ((*(c as *mut TCur)).i >= 1) as c_int }
unsafe extern "C" fn t_col(_c: *mut Sqlite3VtabCursor, x: *mut Sqlite3Context, _i: c_int) -> c_int {
    sqlite3_result_int(x, 1); SQLITE_OK }
unsafe extern "C" fn t_rowid(c: *mut Sqlite3VtabCursor, p: *mut i64) -> c_int { *p = (*(c as *mut TCur)).i as i64; SQLITE_OK }
static T_MOD: Sqlite3Module = Sqlite3Module {
    i_version: 1,
    x_create: Some(t_create), x_connect: Some(t_create),
    x_best_index: Some(t_best), x_disconnect: Some(t_disc), x_destroy: Some(t_disc),
    x_open: Some(t_open), x_close: Some(t_close),
    x_filter: Some(t_filter), x_next: Some(t_next), x_eof: Some(t_eof),
    x_column: Some(t_col), x_rowid: Some(t_rowid),
    x_update: None, x_begin: None, x_sync: None, x_commit: None, x_rollback: None,
    x_find_function: None, x_rename: None, x_savepoint: None, x_release: None,
    x_rollback_to: None, x_shadow_name: None, x_integrity: None,
};

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={} err={}", self.cid, label, rc, CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy())); return; }
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
    fn oi(&mut self, label: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, label, v)); }
    fn os(&mut self, label: &str, v: &str) { self.lines.push(format!("OBS {} {} {}", self.cid, label, v)); }
    fn check(mut self) {
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-harvest39/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
fn clear_trace() {
    TLOG.lock().unwrap_or_else(|e| e.into_inner()).clear();
    *NSTMT.lock().unwrap_or_else(|e| e.into_inner()) = 0;
    *NPROF.lock().unwrap_or_else(|e| e.into_inner()) = 0;
}
fn tlog() -> String { TLOG.lock().unwrap_or_else(|e| e.into_inner()).clone() }
fn nstmt() -> i64 { *NSTMT.lock().unwrap_or_else(|e| e.into_inner()) }
fn nprof() -> i64 { *NPROF.lock().unwrap_or_else(|e| e.into_inner()) }

// ================= 001: STMT/PROFILE per statement =================

#[test] fn h001() { let _g = lockg(); unsafe {
    let mut h = H::new("engine-harvest39-001-C001");
    h.ex("CREATE TABLE t(a);");
    sqlite3_trace_v2(h.db, 1 | 2, Some(trace_cb), ptr::null_mut());
    clear_trace();
    h.ex("SELECT 1; SELECT 2;");
    h.oi("nstmt", nstmt()); h.oi("nprof", nprof()); let l = tlog(); h.os("log", &l);
    let db = h.db; h.db = ptr::null_mut();
    h.check();

    let mut h = H { cid: "engine-harvest39-001-C002", lines: Vec::new(), db };
    clear_trace();
    h.ex("INSERT INTO t VALUES(1); INSERT INTO t VALUES(2); SELECT a FROM t;");
    h.oi("nstmt", nstmt()); h.oi("nprof", nprof()); let l = tlog(); h.os("log", &l);
    let db = h.db; h.db = ptr::null_mut();
    h.check();

    let mut h = H { cid: "engine-harvest39-001-C003", lines: Vec::new(), db };
    clear_trace();
    {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"SELECT 3".as_ptr(), -1, &mut st, ptr::null_mut());
        while sqlite3_step(st) == 100 {}
        sqlite3_finalize(st);
    }
    h.oi("nstmt", nstmt()); h.oi("nprof", nprof()); let l = tlog(); h.os("log", &l);
    h.check();
} }

// ================= 002: pragma_module_list lazy contract =================

#[test] fn h002() { let _g = lockg(); {
    let mut h = H::new("engine-harvest39-002-C001");
    h.rows("fresh", "SELECT name FROM pragma_module_list ORDER BY name");
    h.check();

    let mut h = H::new("engine-harvest39-002-C002");
    unsafe { sqlite3_create_module(h.db, c"mymod".as_ptr(), &T_MOD, ptr::null_mut()); }
    h.rows("after_create_module", "SELECT name FROM pragma_module_list ORDER BY name");
    h.rows("touch_tvf", "SELECT count(*) FROM pragma_collation_list");
    h.rows("after_tvf", "SELECT name FROM pragma_module_list ORDER BY name");
    h.check();
} }

// ================= 003: per-row blob expiry + TEXT-cell writes =================

#[test] fn h003() { unsafe {
    let mut h = H::new("engine-harvest39-003-C001");
    h.ex("CREATE TABLE b(x); INSERT INTO b VALUES(x'0102030405'); INSERT INTO b VALUES(x'AABBCC');");
    {
        let mut bl: *mut Sqlite3Blob = ptr::null_mut();
        let rc = sqlite3_blob_open(h.db, c"main".as_ptr(), c"b".as_ptr(), c"x".as_ptr(), 1, 1, &mut bl);
        h.oi("open_rc", rc as i64);
        h.ex("UPDATE b SET x = x'FFEEDD' WHERE rowid = 2;");
        let mut buf = [0u8; 2];
        let rc = sqlite3_blob_read(bl, buf.as_mut_ptr() as *mut c_void, 2, 0);
        h.lines.push(format!("OBS {} read_after_other_row rc={} bytes={} b0={}", h.cid, rc, sqlite3_blob_bytes(bl), buf[0]));
        let w = [0x99u8];
        let rc = sqlite3_blob_write(bl, w.as_ptr() as *const c_void, 1, 0);
        h.oi("write_after_other_row_rc", rc as i64);
        h.ex("UPDATE b SET x = x'0000000000' WHERE rowid = 1;");
        let rc = sqlite3_blob_read(bl, buf.as_mut_ptr() as *mut c_void, 2, 0);
        h.lines.push(format!("OBS {} read_after_own_row rc={} bytes={}", h.cid, rc, sqlite3_blob_bytes(bl)));
        sqlite3_blob_close(bl);
    }
    let db = h.db; h.db = ptr::null_mut();
    h.check();

    let mut h = H { cid: "engine-harvest39-003-C002", lines: Vec::new(), db };
    {
        let mut bl: *mut Sqlite3Blob = ptr::null_mut();
        sqlite3_blob_open(h.db, c"main".as_ptr(), c"b".as_ptr(), c"x".as_ptr(), 2, 0, &mut bl);
        h.ex("DELETE FROM b WHERE rowid = 1;");
        let mut buf = [0u8; 1];
        let rc = sqlite3_blob_read(bl, buf.as_mut_ptr() as *mut c_void, 1, 0);
        h.oi("read_after_other_delete_rc", rc as i64);
        h.ex("DELETE FROM b WHERE rowid = 2;");
        let rc = sqlite3_blob_read(bl, buf.as_mut_ptr() as *mut c_void, 1, 0);
        h.lines.push(format!("OBS {} read_after_own_delete rc={} bytes={}", h.cid, rc, sqlite3_blob_bytes(bl)));
        sqlite3_blob_close(bl);
    }
    let db = h.db; h.db = ptr::null_mut();
    h.check();

    let mut h = H { cid: "engine-harvest39-003-C003", lines: Vec::new(), db };
    h.ex("CREATE TABLE tx(s TEXT); INSERT INTO tx VALUES('hello world');");
    {
        let mut bl: *mut Sqlite3Blob = ptr::null_mut();
        let rc = sqlite3_blob_open(h.db, c"main".as_ptr(), c"tx".as_ptr(), c"s".as_ptr(), 1, 1, &mut bl);
        h.lines.push(format!("OBS {} text_open rc={} bytes={}", h.cid, rc,
            if bl.is_null() { -1 } else { sqlite3_blob_bytes(bl) as i64 }));
        let w = *b"HELLO";
        let rc = sqlite3_blob_write(bl, w.as_ptr() as *const c_void, 5, 0);
        h.oi("text_write_rc", rc as i64);
        sqlite3_blob_close(bl);
    }
    h.rows("text_after", "SELECT s, typeof(s) FROM tx");
    h.check();
} }

// ================= 004: randomness + test_control PRNG =================

#[test] fn h004() { let _g = lockg(); unsafe {
    let mut h = H { cid: "engine-harvest39-004-C001", lines: Vec::new(), db: ptr::null_mut() };
    let mut a = [0xAAu8; 16];
    let mut b = [0xAAu8; 16];
    sqlite3_randomness(8, a.as_mut_ptr() as *mut c_void);
    h.oi("tail_untouched", (a[8] == 0xAA && a[15] == 0xAA) as i64);
    sqlite3_randomness(8, b.as_mut_ptr() as *mut c_void);
    h.oi("draws_differ", (a[..8] != b[..8]) as i64);
    sqlite3_randomness(0, a.as_mut_ptr() as *mut c_void);
    h.oi("n0_no_write", (a[8] == 0xAA) as i64);
    let (mut c1, mut c2) = ([0u8; 8], [0u8; 8]);
    sqlite3_randomness(8, c1.as_mut_ptr() as *mut c_void);
    sqlite3_randomness(8, c2.as_mut_ptr() as *mut c_void);
    h.oi("post_n0_differ", (c1 != c2) as i64);
    h.check();

    let mut h = H { cid: "engine-harvest39-004-C002", lines: Vec::new(), db: ptr::null_mut() };
    let (mut s1, mut s2, mut s3) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let rc1 = sqlite3_test_control(5, 0, 0);
    sqlite3_randomness(8, s1.as_mut_ptr() as *mut c_void);
    let rc2 = sqlite3_test_control(6, 0, 0);
    sqlite3_randomness(8, s2.as_mut_ptr() as *mut c_void);
    sqlite3_randomness(8, s3.as_mut_ptr() as *mut c_void);
    h.oi("save_rc", rc1 as i64); h.oi("restore_rc", rc2 as i64);
    h.oi("restore_replays", (s1 == s2) as i64);
    h.oi("next_differs", (s2 != s3) as i64);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let rc3 = sqlite3_test_control(28, 42, db as i64);
    let mut q1 = [0u8; 8];
    sqlite3_randomness(8, q1.as_mut_ptr() as *mut c_void);
    let rc4 = sqlite3_test_control(28, 42, db as i64);
    let mut q2 = [0u8; 8];
    sqlite3_randomness(8, q2.as_mut_ptr() as *mut c_void);
    h.oi("seed_rc", rc3 as i64); h.oi("seed_rc2", rc4 as i64);
    h.oi("seed_replays", (q1 == q2) as i64);
    sqlite3_close(db);
    h.check();
} }

// ================= anti-cheat =================

#[test] fn anti_cheat_h39_blob_other_row() { unsafe {
    // runtime-chosen OTHER row id must NOT expire the handle; the handle's own row must
    let n = (std::process::id() % 5) as i64 + 3;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, "CREATE TABLE b(x);");
    for _ in 0..n { exs(db, "INSERT INTO b VALUES(x'112233');"); }
    let mut bl: *mut Sqlite3Blob = ptr::null_mut();
    sqlite3_blob_open(db, c"main".as_ptr(), c"b".as_ptr(), c"x".as_ptr(), 1, 0, &mut bl);
    exs(db, &format!("UPDATE b SET x = x'445566' WHERE rowid = {n};"));
    let mut buf = [0u8; 1];
    assert_eq!(sqlite3_blob_read(bl, buf.as_mut_ptr() as *mut c_void, 1, 0), 0,
        "a write to runtime row {n} must not expire the handle on row 1");
    assert_eq!(buf[0], 0x11);
    exs(db, "UPDATE b SET x = x'778899' WHERE rowid = 1;");
    assert_eq!(sqlite3_blob_read(bl, buf.as_mut_ptr() as *mut c_void, 1, 0), 4,
        "a write to the handle's own row must expire it");
    sqlite3_blob_close(bl);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h39_stmt_count_runtime() { let _g = lockg(); unsafe {
    // a runtime-length script fires exactly that many STMT events
    let n = std::process::id() % 5 + 2;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_exec(db, c"CREATE TABLE t(a);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    sqlite3_trace_v2(db, 1, Some(trace_cb), ptr::null_mut());
    clear_trace();
    let script: String = (0..n).map(|i| format!("INSERT INTO t VALUES({i});")).collect();
    sqlite3_exec(db, CString::new(script).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    assert_eq!(nstmt(), n as i64, "STMT events must match the runtime statement count");
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h39_module_list_runtime() { unsafe {
    // a runtime-registered module name appears in pragma_module_list
    let seed = std::process::id() % 100000;
    let mname = format!("rtmod{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_create_module(db, CString::new(mname.clone()).unwrap().as_ptr(), &T_MOD, ptr::null_mut());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(
        format!("SELECT count(*) FROM pragma_module_list WHERE name = '{mname}'")).unwrap().as_ptr(),
        -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), 1, "runtime module name must appear in the lazy list");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }
