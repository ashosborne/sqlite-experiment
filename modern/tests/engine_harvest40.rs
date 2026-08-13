//! Run-50 partial-to-full replay — mirrors /tmp/h40.c.
//! vtab xRename + xSavepoint/xRelease/xRollbackTo with C's txn-savepoint-excluded
//! numbering; legacy sqlite3_trace/sqlite3_profile (shared slot, param expansion);
//! WITHOUT ROWID update_hook suppression and the truncate fast-path.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

// ---- module: logging + snapshot storage (twin of the C harness `ser`) ----
static MLOG: Mutex<String> = Mutex::new(String::new());
static SROWS: Mutex<Vec<i64>> = Mutex::new(Vec::new());
static SNAP: Mutex<[i64; 16]> = Mutex::new([0; 16]);
static BASE: Mutex<i64> = Mutex::new(0);
fn mlog(s: String) { MLOG.lock().unwrap_or_else(|e| e.into_inner()).push_str(&s); }
fn mlog_take() -> String { std::mem::take(&mut *MLOG.lock().unwrap_or_else(|e| e.into_inner())) }
fn srows() -> std::sync::MutexGuard<'static, Vec<i64>> { SROWS.lock().unwrap_or_else(|e| e.into_inner()) }

#[repr(C)] struct MCur { base: Sqlite3VtabCursor, i: i64 }
unsafe extern "C" fn m_create(db: *mut Sqlite3, _a: *mut c_void, _argc: c_int,
        _argv: *const *const c_char, pp: *mut *mut Sqlite3Vtab, _pz: *mut *mut c_char) -> c_int {
    let rc = sqlite3_declare_vtab(db, c"CREATE TABLE x(v INTEGER)".as_ptr());
    if rc != SQLITE_OK { return rc; }
    let v = Box::new(Sqlite3Vtab { p_module: ptr::null(), n_ref: 0, z_err_msg: ptr::null_mut() });
    *pp = Box::into_raw(v);
    SQLITE_OK }
unsafe extern "C" fn m_disc(v: *mut Sqlite3Vtab) -> c_int { drop(Box::from_raw(v)); SQLITE_OK }
unsafe extern "C" fn m_best(_v: *mut Sqlite3Vtab, _i: *mut Sqlite3IndexInfo) -> c_int { SQLITE_OK }
unsafe extern "C" fn m_open(_v: *mut Sqlite3Vtab, pp: *mut *mut Sqlite3VtabCursor) -> c_int {
    let c = Box::new(MCur { base: Sqlite3VtabCursor { p_vtab: ptr::null_mut() }, i: 0 });
    *pp = Box::into_raw(c) as *mut Sqlite3VtabCursor; SQLITE_OK }
unsafe extern "C" fn m_close(c: *mut Sqlite3VtabCursor) -> c_int { drop(Box::from_raw(c as *mut MCur)); SQLITE_OK }
unsafe extern "C" fn m_filter(c: *mut Sqlite3VtabCursor, _n: c_int, _s: *const c_char,
        _ac: c_int, _av: *mut *mut Sqlite3Value) -> c_int { (*(c as *mut MCur)).i = 0; SQLITE_OK }
unsafe extern "C" fn m_next(c: *mut Sqlite3VtabCursor) -> c_int { (*(c as *mut MCur)).i += 1; SQLITE_OK }
unsafe extern "C" fn m_eof(c: *mut Sqlite3VtabCursor) -> c_int {
    ((*(c as *mut MCur)).i >= srows().len() as i64) as c_int }
unsafe extern "C" fn m_col(c: *mut Sqlite3VtabCursor, x: *mut Sqlite3Context, _i: c_int) -> c_int {
    let v = srows()[(*(c as *mut MCur)).i as usize];
    sqlite3_result_int64(x, v); SQLITE_OK }
unsafe extern "C" fn m_rowid(c: *mut Sqlite3VtabCursor, p: *mut i64) -> c_int {
    *p = (*(c as *mut MCur)).i + 1; SQLITE_OK }
unsafe extern "C" fn m_update(_v: *mut Sqlite3Vtab, argc: c_int, argv: *mut *mut Sqlite3Value, p_rowid: *mut i64) -> c_int {
    if argc > 1 && sqlite3_value_type(*argv) == 5 /* NULL */ {
        let val = sqlite3_value_int64(*argv.add(2));
        let mut r = srows(); r.push(val);
        *p_rowid = r.len() as i64;
        mlog(format!("[ins:{val}]"));
    } else if argc == 1 {
        let rid = sqlite3_value_int64(*argv);
        srows().pop();
        mlog(format!("[del:{rid}]"));
    }
    SQLITE_OK }
unsafe extern "C" fn m_begin(_v: *mut Sqlite3Vtab) -> c_int {
    mlog("[begin]".into()); *BASE.lock().unwrap_or_else(|e| e.into_inner()) = srows().len() as i64; SQLITE_OK }
unsafe extern "C" fn m_sync(_v: *mut Sqlite3Vtab) -> c_int { mlog("[sync]".into()); SQLITE_OK }
unsafe extern "C" fn m_commit(_v: *mut Sqlite3Vtab) -> c_int { mlog("[commit]".into()); SQLITE_OK }
unsafe extern "C" fn m_rollback(_v: *mut Sqlite3Vtab) -> c_int {
    mlog("[rollback]".into());
    let b = *BASE.lock().unwrap_or_else(|e| e.into_inner());
    srows().truncate(b as usize); SQLITE_OK }
unsafe extern "C" fn m_rename(_v: *mut Sqlite3Vtab, z: *const c_char) -> c_int {
    mlog(format!("[rename:{}]", CStr::from_ptr(z).to_string_lossy())); SQLITE_OK }
unsafe extern "C" fn m_savepoint(_v: *mut Sqlite3Vtab, n: c_int) -> c_int {
    mlog(format!("[svpt:{n}]"));
    if (0..16).contains(&n) { SNAP.lock().unwrap_or_else(|e| e.into_inner())[n as usize] = srows().len() as i64; }
    SQLITE_OK }
unsafe extern "C" fn m_release(_v: *mut Sqlite3Vtab, n: c_int) -> c_int {
    mlog(format!("[release:{n}]")); SQLITE_OK }
unsafe extern "C" fn m_rollback_to(_v: *mut Sqlite3Vtab, n: c_int) -> c_int {
    mlog(format!("[rbto:{n}]"));
    let target = if (0..16).contains(&n) {
        SNAP.lock().unwrap_or_else(|e| e.into_inner())[n as usize]
    } else {
        *BASE.lock().unwrap_or_else(|e| e.into_inner())
    };
    srows().truncate(target as usize);
    SQLITE_OK }
static M: Sqlite3Module = Sqlite3Module {
    i_version: 2,
    x_create: Some(m_create), x_connect: Some(m_create),
    x_best_index: Some(m_best), x_disconnect: Some(m_disc), x_destroy: Some(m_disc),
    x_open: Some(m_open), x_close: Some(m_close),
    x_filter: Some(m_filter), x_next: Some(m_next), x_eof: Some(m_eof),
    x_column: Some(m_col), x_rowid: Some(m_rowid),
    x_update: Some(m_update), x_begin: Some(m_begin), x_sync: Some(m_sync),
    x_commit: Some(m_commit), x_rollback: Some(m_rollback),
    x_find_function: None, x_rename: Some(m_rename),
    x_savepoint: Some(m_savepoint), x_release: Some(m_release), x_rollback_to: Some(m_rollback_to),
    x_shadow_name: None, x_integrity: None,
};
static M1: Sqlite3Module = Sqlite3Module {
    i_version: 1,
    x_create: Some(m_create), x_connect: Some(m_create),
    x_best_index: Some(m_best), x_disconnect: Some(m_disc), x_destroy: Some(m_disc),
    x_open: Some(m_open), x_close: Some(m_close),
    x_filter: Some(m_filter), x_next: Some(m_next), x_eof: Some(m_eof),
    x_column: Some(m_col), x_rowid: Some(m_rowid),
    x_update: Some(m_update), x_begin: None, x_sync: None,
    x_commit: None, x_rollback: None,
    x_find_function: None, x_rename: None,
    x_savepoint: None, x_release: None, x_rollback_to: None,
    x_shadow_name: None, x_integrity: None,
};

// ---- legacy trace / profile / hook plumbing ----
static LTLOG: Mutex<String> = Mutex::new(String::new());
static NLT: Mutex<i64> = Mutex::new(0);
static NLP: Mutex<i64> = Mutex::new(0);
unsafe extern "C" fn legacy_trace(_c: *mut c_void, z: *const c_char) {
    *NLT.lock().unwrap_or_else(|e| e.into_inner()) += 1;
    LTLOG.lock().unwrap_or_else(|e| e.into_inner())
        .push_str(&format!("[T:{}]", CStr::from_ptr(z).to_string_lossy()));
}
unsafe extern "C" fn legacy_profile(_c: *mut c_void, z: *const c_char, _ns: u64) {
    *NLP.lock().unwrap_or_else(|e| e.into_inner()) += 1;
    LTLOG.lock().unwrap_or_else(|e| e.into_inner())
        .push_str(&format!("[P:{}]", CStr::from_ptr(z).to_string_lossy()));
}
unsafe extern "C" fn trace_v2cb(t: u32, _c: *mut c_void, _p: *mut c_void, x: *mut c_void) -> c_int {
    if t == 1 {
        LTLOG.lock().unwrap_or_else(|e| e.into_inner())
            .push_str(&format!("[V2:{}]", CStr::from_ptr(x as *const c_char).to_string_lossy()));
    }
    0
}
static NHOOK: Mutex<i64> = Mutex::new(0);
static HLOG: Mutex<String> = Mutex::new(String::new());
unsafe extern "C" fn uhook(_c: *mut c_void, op: c_int, _d: *const c_char, t: *const c_char, r: i64) {
    *NHOOK.lock().unwrap_or_else(|e| e.into_inner()) += 1;
    HLOG.lock().unwrap_or_else(|e| e.into_inner())
        .push_str(&format!("[{op}:{}:{r}]", CStr::from_ptr(t).to_string_lossy()));
}
fn clear_lt() {
    LTLOG.lock().unwrap_or_else(|e| e.into_inner()).clear();
    *NLT.lock().unwrap_or_else(|e| e.into_inner()) = 0;
    *NLP.lock().unwrap_or_else(|e| e.into_inner()) = 0;
}
fn ltlog() -> String { LTLOG.lock().unwrap_or_else(|e| e.into_inner()).clone() }
fn nlt() -> i64 { *NLT.lock().unwrap_or_else(|e| e.into_inner()) }
fn nlp() -> i64 { *NLP.lock().unwrap_or_else(|e| e.into_inner()) }
fn clear_hook() {
    HLOG.lock().unwrap_or_else(|e| e.into_inner()).clear();
    *NHOOK.lock().unwrap_or_else(|e| e.into_inner()) = 0;
}
fn hlog() -> String { HLOG.lock().unwrap_or_else(|e| e.into_inner()).clone() }
fn nhook() -> i64 { *NHOOK.lock().unwrap_or_else(|e| e.into_inner()) }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn reset_mod(&self) {
        srows().clear();
        *BASE.lock().unwrap_or_else(|e| e.into_inner()) = 0;
        mlog_take();
    }
    fn exm(&mut self, label: &str, s: &str) { unsafe {
        mlog_take();
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let err = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={} mlog={}", self.cid, label, rc, err, mlog_take()));
    } }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
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
    fn check_keep(&mut self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-harvest40/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
}

// ================= 001: vtab xRename =================

#[test] fn h001() { let _g = lockg(); unsafe {
    let mut h = H::new("engine-harvest40-001-C001");
    sqlite3_create_module(h.db, c"ser".as_ptr(), &M, ptr::null_mut());
    h.reset_mod();
    h.exm("create", "CREATE VIRTUAL TABLE vt USING ser;");
    h.exm("ins", "INSERT INTO vt VALUES(5);");
    h.exm("rename", "ALTER TABLE vt RENAME TO wt;");
    h.rows("master", "SELECT name, sql FROM sqlite_master");
    h.rows("sel_new", "SELECT v FROM wt");
    h.exm("sel_old", "SELECT v FROM vt;");
    h.check_keep();
    h.cid = "engine-harvest40-001-C002";
    sqlite3_create_module(h.db, c"nor".as_ptr(), &M1, ptr::null_mut());
    h.exm("create2", "CREATE VIRTUAL TABLE nv USING nor;");
    h.exm("rename2", "ALTER TABLE nv RENAME TO nw;");
    h.rows("master2", "SELECT name FROM sqlite_master ORDER BY name");
    h.check();
} }

// ================= 002: xSavepoint / xRelease / xRollbackTo =================

#[test] fn h002() { let _g = lockg(); unsafe {
    let mut h = H::new("engine-harvest40-002-C001");
    sqlite3_create_module(h.db, c"ser".as_ptr(), &M, ptr::null_mut());
    h.reset_mod();
    h.exm("create", "CREATE VIRTUAL TABLE vt USING ser;");
    h.exm("begin", "BEGIN;");
    h.exm("ins1", "INSERT INTO vt VALUES(11);");
    h.exm("sp1", "SAVEPOINT s1;");
    h.exm("ins2", "INSERT INTO vt VALUES(22);");
    h.exm("sp2", "SAVEPOINT s2;");
    h.exm("ins3", "INSERT INTO vt VALUES(33);");
    h.rows("pre_rb", "SELECT v FROM vt");
    h.exm("rbto1", "ROLLBACK TO s1;");
    h.rows("post_rb", "SELECT v FROM vt");
    h.exm("rel1", "RELEASE s1;");
    h.exm("commit", "COMMIT;");
    h.rows("final", "SELECT v FROM vt");
    h.check();

    let mut h = H::new("engine-harvest40-002-C002");
    sqlite3_create_module(h.db, c"ser".as_ptr(), &M, ptr::null_mut());
    h.reset_mod();
    h.exm("create", "CREATE VIRTUAL TABLE vt USING ser;");
    h.exm("sp_a", "SAVEPOINT a;");
    h.exm("sp_b", "SAVEPOINT b;");
    h.exm("ins", "INSERT INTO vt VALUES(7);");
    h.exm("sp_c", "SAVEPOINT c;");
    h.exm("ins2", "INSERT INTO vt VALUES(8);");
    h.exm("rb_c", "ROLLBACK TO c;");
    h.rows("after_rb_c", "SELECT v FROM vt");
    h.exm("rb_a", "ROLLBACK TO a;");
    h.rows("after_rb_a", "SELECT v FROM vt");
    h.exm("rel_a", "RELEASE a;");
    h.rows("final", "SELECT v FROM vt");
    h.check();

    let mut h = H::new("engine-harvest40-002-C003");
    sqlite3_create_module(h.db, c"ser".as_ptr(), &M, ptr::null_mut());
    h.reset_mod();
    h.exm("create", "CREATE VIRTUAL TABLE vt USING ser;");
    h.exm("auto_ins", "INSERT INTO vt VALUES(1);");
    h.exm("begin", "BEGIN;");
    h.exm("ins", "INSERT INTO vt VALUES(2);");
    h.exm("rollback", "ROLLBACK;");
    h.rows("after_rollback", "SELECT v FROM vt");
    h.check();
} }

// ================= 003: legacy sqlite3_trace / sqlite3_profile =================

#[test] fn h003() { let _g = lockg(); unsafe {
    let mut h = H::new("engine-harvest40-003-C001");
    h.ex("CREATE TABLE t(a);");
    sqlite3_trace(h.db, Some(legacy_trace), ptr::null_mut());
    clear_lt();
    h.ex("INSERT INTO t VALUES(1);");
    {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"SELECT a FROM t WHERE a = ?".as_ptr(), -1, &mut st, ptr::null_mut());
        sqlite3_bind_int(st, 1, 1);
        while sqlite3_step(st) == 100 {}
        sqlite3_finalize(st);
    }
    h.oi("nlt", nlt()); let l = ltlog(); h.os("log", &l);
    h.check_keep();
    h.cid = "engine-harvest40-003-C002";
    sqlite3_profile(h.db, Some(legacy_profile), ptr::null_mut());
    clear_lt();
    h.ex("SELECT count(*) FROM t;");
    h.oi("nlt", nlt()); h.oi("nlp", nlp()); let l = ltlog(); h.os("log", &l);
    h.check_keep();
    h.cid = "engine-harvest40-003-C003";
    sqlite3_trace_v2(h.db, 1, Some(trace_v2cb), ptr::null_mut());
    clear_lt();
    h.ex("SELECT 1;");
    h.oi("nlt", nlt()); h.oi("nlp", nlp()); let l = ltlog(); h.os("log", &l);
    sqlite3_trace_v2(h.db, 0, None, ptr::null_mut());
    clear_lt();
    h.ex("SELECT 2;");
    h.oi("nlt2", nlt()); h.oi("nlp2", nlp()); let l = ltlog(); h.os("log2", &l);
    h.check();
} }

// ================= 004: WITHOUT ROWID hooks + truncate fast-path =================

#[test] fn h004() { let _g = lockg(); unsafe {
    let mut h = H::new("engine-harvest40-004-C001");
    h.ex("CREATE TABLE r(a INTEGER PRIMARY KEY, b); CREATE TABLE w(a INT PRIMARY KEY, b) WITHOUT ROWID;");
    sqlite3_update_hook(h.db, Some(uhook), ptr::null_mut());
    clear_hook();
    h.ex("INSERT INTO r VALUES(1,'x'); UPDATE r SET b='y'; DELETE FROM r WHERE a=1;");
    h.oi("rowid_events", nhook()); let l = hlog(); h.os("rowid_log", &l);
    clear_hook();
    h.ex("INSERT INTO w VALUES(1,'x'); UPDATE w SET b='y'; DELETE FROM w WHERE a=1;");
    h.oi("wr_events", nhook()); let l = hlog(); h.os("wr_log", &l);
    h.check_keep();
    h.cid = "engine-harvest40-004-C002";
    h.ex("INSERT INTO r VALUES(1,'x'),(2,'y'),(3,'z');");
    clear_hook();
    h.ex("DELETE FROM r;");
    h.oi("trunc_events", nhook()); h.oi("trunc_changes", sqlite3_changes(h.db) as i64);
    h.ex("INSERT INTO r VALUES(4,'a'),(5,'b');");
    clear_hook();
    h.ex("DELETE FROM r WHERE a >= 4;");
    h.oi("where_events", nhook()); h.oi("where_changes", sqlite3_changes(h.db) as i64);
    let l = hlog(); h.os("where_log", &l);
    h.check();
} }

// ================= anti-cheat =================

#[test] fn anti_cheat_h40_rename_runtime() { let _g = lockg(); unsafe {
    // runtime-chosen rename target must reach xRename and the schema
    let seed = std::process::id() % 100000;
    let newname = format!("rt{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_create_module(db, c"ser".as_ptr(), &M, ptr::null_mut());
    srows().clear();
    *BASE.lock().unwrap_or_else(|e| e.into_inner()) = 0;
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, "CREATE VIRTUAL TABLE vt USING ser;");
    mlog_take();
    exs(db, &format!("ALTER TABLE vt RENAME TO {newname};"));
    assert_eq!(mlog_take(), format!("[rename:{newname}]"), "xRename must receive the runtime name");
    exs(db, &format!("INSERT INTO {newname} VALUES(9);"));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT v FROM {newname}")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), 9);
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h40_savepoint_rollback_runtime() { let _g = lockg(); unsafe {
    // a runtime-length batch inserted after a savepoint must vanish on ROLLBACK TO
    let n = (std::process::id() % 5) as i64 + 2;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_create_module(db, c"ser".as_ptr(), &M, ptr::null_mut());
    srows().clear();
    *BASE.lock().unwrap_or_else(|e| e.into_inner()) = 0;
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, "CREATE VIRTUAL TABLE vt USING ser;");
    exs(db, "BEGIN; INSERT INTO vt VALUES(1); SAVEPOINT k;");
    for i in 0..n { exs(db, &format!("INSERT INTO vt VALUES({});", 100 + i)); }
    let count = |db: *mut Sqlite3| -> i64 {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db, c"SELECT count(*) FROM vt".as_ptr(), -1, &mut st, ptr::null_mut());
        sqlite3_step(st);
        let v = sqlite3_column_int64(st, 0);
        sqlite3_finalize(st);
        v };
    assert_eq!(count(db), n + 1, "runtime batch visible before rollback");
    exs(db, "ROLLBACK TO k;");
    assert_eq!(count(db), 1, "runtime batch must vanish on ROLLBACK TO");
    exs(db, "COMMIT;");
    sqlite3_close(db);
} }
