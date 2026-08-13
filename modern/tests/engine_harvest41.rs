//! Run-51 authorizer action-code replay — mirrors /tmp/h41.c.
//! FUNCTION 31 / SAVEPOINT 32 / ANALYZE 28 / ALTER 26 outer / DROP_TABLE 11 /
//! CREATE_VTABLE 29 / DROP_VTABLE 30 / view+trigger s4. Callback logs are
//! FILTERED per case to the whitelisted codes (no sqlite_master tails frozen).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }
static ALOG: Mutex<String> = Mutex::new(String::new());
static FILTER: Mutex<Vec<i32>> = Mutex::new(Vec::new());
static DENY_CODE: Mutex<i32> = Mutex::new(0);
static DENY_NAME: Mutex<String> = Mutex::new(String::new());
static IGN_CODE: Mutex<i32> = Mutex::new(0);
static IGN_NAME: Mutex<String> = Mutex::new(String::new());

fn fs(p: *const c_char) -> String {
    if p.is_null() { "~".into() }
    else {
        let s = unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() };
        if s.is_empty() { "{}".into() } else { s }
    }
}
unsafe extern "C" fn auth_cb(_c: *mut c_void, code: c_int, s1: *const c_char, s2: *const c_char,
        s3: *const c_char, s4: *const c_char) -> c_int {
    let filt = FILTER.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if filt.is_empty() || filt.contains(&code) {
        ALOG.lock().unwrap_or_else(|e| e.into_inner())
            .push_str(&format!("[{}|{}|{}|{}|{}]", code, fs(s1), fs(s2), fs(s3), fs(s4)));
    }
    let s2s = fs(s2);
    let dc = *DENY_CODE.lock().unwrap_or_else(|e| e.into_inner());
    let dn = DENY_NAME.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if code == dc && (dn.is_empty() || s2s == dn) { return 1; }
    let ic = *IGN_CODE.lock().unwrap_or_else(|e| e.into_inner());
    let inm = IGN_NAME.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if code == ic && (inm.is_empty() || s2s == inm) { return 2; }
    0
}
fn set_deny(code: i32, name: &str) {
    *DENY_CODE.lock().unwrap_or_else(|e| e.into_inner()) = code;
    *DENY_NAME.lock().unwrap_or_else(|e| e.into_inner()) = name.to_string();
}
fn set_ign(code: i32, name: &str) {
    *IGN_CODE.lock().unwrap_or_else(|e| e.into_inner()) = code;
    *IGN_NAME.lock().unwrap_or_else(|e| e.into_inner()) = name.to_string();
}
fn set_filter(codes: &[i32]) {
    *FILTER.lock().unwrap_or_else(|e| e.into_inner()) = codes.to_vec();
}
fn take_log() -> String { std::mem::take(&mut *ALOG.lock().unwrap_or_else(|e| e.into_inner())) }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn on(&self) { unsafe { sqlite3_set_authorizer(self.db, Some(auth_cb), ptr::null_mut()); } }
    fn off(&self) { unsafe { sqlite3_set_authorizer(self.db, None, ptr::null_mut()); }
        set_deny(0, ""); set_ign(0, ""); set_filter(&[]); }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn exr(&mut self, label: &str, s: &str) { unsafe {
        take_log();
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let err = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={} LOG={}", self.cid, label, rc, err, take_log()));
    } }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        take_log();
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 {
            self.lines.push(format!("OBS {} {} prep.rc={} err={} LOG={}", self.cid, label, rc,
                CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy(), take_log()));
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
        self.lines.push(format!("OBS {} {} rows={} LOG={}", self.cid, label, buf, take_log()));
    } }
    fn check_keep(&mut self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-harvest41/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
}

// ---- vtab module for 29/30 ----
#[repr(C)] struct TCur { base: Sqlite3VtabCursor, i: i64 }
unsafe extern "C" fn t_create(db: *mut Sqlite3, _a: *mut c_void, _argc: c_int,
        _argv: *const *const c_char, pp: *mut *mut Sqlite3Vtab, _pz: *mut *mut c_char) -> c_int {
    let rc = sqlite3_declare_vtab(db, c"CREATE TABLE x(v INTEGER)".as_ptr());
    if rc != SQLITE_OK { return rc; }
    let v = Box::new(Sqlite3Vtab { p_module: ptr::null(), n_ref: 0, z_err_msg: ptr::null_mut() });
    *pp = Box::into_raw(v);
    SQLITE_OK }
unsafe extern "C" fn t_disc(v: *mut Sqlite3Vtab) -> c_int { drop(Box::from_raw(v)); SQLITE_OK }
unsafe extern "C" fn t_best(_v: *mut Sqlite3Vtab, _i: *mut Sqlite3IndexInfo) -> c_int { SQLITE_OK }
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
unsafe extern "C" fn t_rowid(c: *mut Sqlite3VtabCursor, p: *mut i64) -> c_int { *p = (*(c as *mut TCur)).i; SQLITE_OK }
static M: Sqlite3Module = Sqlite3Module {
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

// ================= 001: SQLITE_FUNCTION =================

#[test] fn h001() { let _g = lockg(); {
    let mut h = H::new("engine-harvest41-001-C001");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(-3);");
    h.on();
    h.rows("f_abs", "SELECT abs(a) FROM t");
    h.rows("f_min", "SELECT min(a) FROM t");
    h.rows("f_count", "SELECT count(*) FROM t");
    h.rows("f_coal", "SELECT coalesce(min(a), 999) FROM t");
    h.off();
    h.check_keep();
    h.cid = "engine-harvest41-001-C002";
    h.on(); set_deny(31, "abs");
    h.rows("f_deny", "SELECT abs(a) FROM t");
    set_deny(31, "min");
    h.rows("f_deny_inner", "SELECT coalesce(min(a), 999) FROM t");
    h.off();
    h.check_keep();
    h.cid = "engine-harvest41-001-C003";
    h.on(); set_ign(31, "min");
    h.rows("f_ignore_min", "SELECT min(a) FROM t");
    h.rows("f_ignore_coal", "SELECT coalesce(min(a), 999) FROM t");
    h.off();
    h.check();
} }

// ================= 002: SQLITE_SAVEPOINT =================

#[test] fn h002() { let _g = lockg(); {
    let mut h = H::new("engine-harvest41-002-C001");
    h.ex("CREATE TABLE t(a);");
    h.on();
    h.exr("sp_open", "SAVEPOINT s1;");
    h.exr("sp_release", "RELEASE s1;");
    h.exr("sp_rbto", "SAVEPOINT s2; ROLLBACK TO s2; RELEASE s2;");
    h.exr("txn_rollback", "BEGIN; ROLLBACK;");
    h.off();
    h.check_keep();
    h.cid = "engine-harvest41-002-C002";
    h.on(); set_deny(32, "");
    h.exr("sp_deny", "SAVEPOINT s3;");
    h.exr("rel_deny_missing", "RELEASE s3;");
    h.off();
    h.check();
} }

// ================= 003: SQLITE_ANALYZE outer =================

#[test] fn h003() { let _g = lockg(); {
    let mut h = H::new("engine-harvest41-003-C001");
    h.ex("CREATE TABLE t(a); CREATE INDEX ti ON t(a); INSERT INTO t VALUES(1),(2);");
    h.ex("ANALYZE;");
    h.on(); set_filter(&[28]);
    h.exr("an_all", "ANALYZE;");
    h.exr("an_t", "ANALYZE t;");
    h.off();
    h.check_keep();
    h.cid = "engine-harvest41-003-C002";
    h.on(); set_filter(&[28]); set_deny(28, "");
    h.exr("an_deny", "ANALYZE;");
    h.off();
    h.check();
} }

// ================= 004: SQLITE_ALTER_TABLE outer =================

#[test] fn h004() { let _g = lockg(); {
    let mut h = H::new("engine-harvest41-004-C001");
    h.ex("CREATE TABLE t(a);");
    h.on(); set_filter(&[26]);
    h.exr("alter_rename", "ALTER TABLE t RENAME TO u;");
    h.off();
    h.rows("after_rename", "SELECT name FROM sqlite_master WHERE type='table'");
    h.on(); set_filter(&[26]);
    h.exr("alter_addcol", "ALTER TABLE u ADD COLUMN b;");
    h.off();
    h.check_keep();
    h.cid = "engine-harvest41-004-C002";
    h.on(); set_filter(&[26]); set_ign(26, "");
    h.exr("alter_ignore", "ALTER TABLE u RENAME TO w;");
    h.off();
    h.rows("after_ignore", "SELECT name FROM sqlite_master WHERE type='table'");
    h.on(); set_filter(&[26]); set_deny(26, "");
    h.exr("alter_deny", "ALTER TABLE u RENAME TO w;");
    h.off();
    h.check();
} }

// ================= 005: DROP_TABLE / vtab codes outer =================

#[test] fn h005() { let _g = lockg(); unsafe {
    let mut h = H::new("engine-harvest41-005-C001");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    h.on(); set_filter(&[11]);
    h.exr("drop_t", "DROP TABLE t;");
    h.off();
    h.ex("CREATE TABLE t2(a);");
    h.on(); set_filter(&[11]); set_deny(11, "");
    h.exr("drop_deny", "DROP TABLE t2;");
    h.off();
    h.rows("t2_survives", "SELECT name FROM sqlite_master WHERE type='table'");
    h.check_keep();
    h.cid = "engine-harvest41-005-C002";
    sqlite3_create_module(h.db, c"ser".as_ptr(), &M, ptr::null_mut());
    h.on(); set_filter(&[29, 30, 0]);
    h.exr("cv", "CREATE VIRTUAL TABLE vt USING ser;");
    h.exr("dv", "DROP TABLE vt;");
    h.off();
    h.check_keep();
    h.cid = "engine-harvest41-005-C003";
    h.on(); set_filter(&[29, 30, 0]); set_deny(29, "");
    h.exr("cv_deny", "CREATE VIRTUAL TABLE vt3 USING ser;");
    h.off();
    h.ex("CREATE VIRTUAL TABLE vt2 USING ser;");
    h.on(); set_filter(&[29, 30, 0]); set_deny(30, "");
    h.exr("dv_deny", "DROP TABLE vt2;");
    h.off();
    h.check();
} }

// ================= 006: view / trigger s4 =================

#[test] fn h006() { let _g = lockg(); {
    let mut h = H::new("engine-harvest41-006-C001");
    h.ex("CREATE TABLE t(a,b); INSERT INTO t VALUES(1,2);");
    h.ex("CREATE VIEW v AS SELECT a, b FROM t;");
    h.on();
    h.rows("view_sel", "SELECT * FROM v");
    h.rows("view_one", "SELECT a FROM v");
    h.off();
    h.check_keep();
    h.cid = "engine-harvest41-006-C002";
    h.ex("CREATE TABLE log(x); CREATE TRIGGER tr AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.on();
    h.exr("trig_ins", "INSERT INTO t VALUES(5,6);");
    h.off();
    h.rows("log_rows", "SELECT x FROM log");
    h.check();
} }

// ================= anti-cheat =================

unsafe extern "C" fn rt_udf(c: *mut Sqlite3Context, _n: c_int, v: *mut *mut Sqlite3Value) {
    let x = sqlite3_value_int64(*v);
    sqlite3_result_int64(c, x + 7);
}

#[test] fn anti_cheat_h41_runtime_function() { let _g = lockg(); unsafe {
    // a runtime-registered UDF name must reach s2, and denying exactly that
    // name must fail with C's per-name message
    let seed = std::process::id() % 100000;
    let fname = format!("rtfn{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_create_function(db, CString::new(fname.clone()).unwrap().as_ptr(), 1, 1,
        ptr::null_mut(), Some(rt_udf), None, None);
    sqlite3_exec(db, c"CREATE TABLE t(a); INSERT INTO t VALUES(1);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    sqlite3_set_authorizer(db, Some(auth_cb), ptr::null_mut());
    take_log();
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let sql = CString::new(format!("SELECT {fname}(a) FROM t")).unwrap();
    assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut()), 0);
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    let log = take_log();
    assert!(log.contains(&format!("[31|~|{fname}|~|~]")), "runtime name must reach s2: {log}");
    set_deny(31, &fname);
    let rc = sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(rc, 1);
    let em = CStr::from_ptr(sqlite3_errmsg(db)).to_string_lossy().into_owned();
    assert_eq!(em, format!("not authorized to use function: {fname}"));
    set_deny(0, "");
    sqlite3_set_authorizer(db, None, ptr::null_mut());
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h41_runtime_savepoint() { let _g = lockg(); unsafe {
    // a runtime savepoint name must reach s2 of code 32
    let seed = std::process::id() % 100000;
    let spname = format!("rtsp{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_set_authorizer(db, Some(auth_cb), ptr::null_mut());
    take_log();
    sqlite3_exec(db, CString::new(format!("SAVEPOINT {spname}; RELEASE {spname};")).unwrap().as_ptr(),
        None, ptr::null_mut(), ptr::null_mut());
    let log = take_log();
    assert_eq!(log, format!("[32|BEGIN|{spname}|~|~][32|RELEASE|{spname}|~|~]"));
    sqlite3_set_authorizer(db, None, ptr::null_mut());
    sqlite3_close(db);
} }
