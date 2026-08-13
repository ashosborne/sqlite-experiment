//! Run-48 vtab lifecycle replay — mirrors /tmp/v38h.c.
//! Plain language: reopening a file db with a stored CREATE VIRTUAL TABLE row runs
//! xConnect (never xCreate) once the module is re-registered; sqlite3_drop_modules
//! keeps the named survivors; the _v2 destructor defers while instances hold the
//! module; xUpdate makes vtabs writable through the module's own storage.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }
static N_CREATE: AtomicI64 = AtomicI64::new(0);
static N_CONNECT: AtomicI64 = AtomicI64::new(0);
static N_MDEST: AtomicI64 = AtomicI64::new(0);
static N_DESTROY: AtomicI64 = AtomicI64::new(0);
static N_DISC: AtomicI64 = AtomicI64::new(0);
static UPLOG: Mutex<String> = Mutex::new(String::new());
struct MemRow { rowid: i64, a: i64, b: String }
static MEM: Mutex<Vec<MemRow>> = Mutex::new(Vec::new());
fn mem() -> std::sync::MutexGuard<'static, Vec<MemRow>> { MEM.lock().unwrap_or_else(|e| e.into_inner()) }
fn uplog(s: String) { UPLOG.lock().unwrap_or_else(|e| e.into_inner()).push_str(&s); }

// ---- ser: separate create/connect counters ----
#[repr(C)] struct SerVtab { base: Sqlite3Vtab, n: i64 }
#[repr(C)] struct SerCur { base: Sqlite3VtabCursor, i: i64 }
unsafe fn ser_mk(db: *mut Sqlite3, argc: c_int, argv: *const *const c_char,
        pp: *mut *mut Sqlite3Vtab, is_create: bool) -> c_int {
    if is_create { N_CREATE.fetch_add(1, Ordering::SeqCst); } else { N_CONNECT.fetch_add(1, Ordering::SeqCst); }
    let mut n: i64 = 3;
    if argc > 3 {
        let a = CStr::from_ptr(*argv.add(3)).to_string_lossy().into_owned();
        n = a.trim().parse().unwrap_or(3);
    }
    let rc = sqlite3_declare_vtab(db, c"CREATE TABLE x(value INTEGER)".as_ptr());
    if rc != SQLITE_OK { return rc; }
    let v = Box::new(SerVtab { base: Sqlite3Vtab { p_module: ptr::null(), n_ref: 0, z_err_msg: ptr::null_mut() }, n });
    *pp = Box::into_raw(v) as *mut Sqlite3Vtab;
    SQLITE_OK
}
unsafe extern "C" fn ser_create(db: *mut Sqlite3, _a: *mut c_void, argc: c_int,
        argv: *const *const c_char, pp: *mut *mut Sqlite3Vtab, _pz: *mut *mut c_char) -> c_int {
    ser_mk(db, argc, argv, pp, true) }
unsafe extern "C" fn ser_connect(db: *mut Sqlite3, _a: *mut c_void, argc: c_int,
        argv: *const *const c_char, pp: *mut *mut Sqlite3Vtab, _pz: *mut *mut c_char) -> c_int {
    ser_mk(db, argc, argv, pp, false) }
unsafe extern "C" fn ser_disc(v: *mut Sqlite3Vtab) -> c_int {
    N_DISC.fetch_add(1, Ordering::SeqCst); drop(Box::from_raw(v as *mut SerVtab)); SQLITE_OK }
unsafe extern "C" fn ser_dest(v: *mut Sqlite3Vtab) -> c_int {
    N_DESTROY.fetch_add(1, Ordering::SeqCst); drop(Box::from_raw(v as *mut SerVtab)); SQLITE_OK }
unsafe extern "C" fn ser_best(_v: *mut Sqlite3Vtab, _i: *mut Sqlite3IndexInfo) -> c_int { SQLITE_OK }
unsafe extern "C" fn ser_open(_v: *mut Sqlite3Vtab, pp: *mut *mut Sqlite3VtabCursor) -> c_int {
    let c = Box::new(SerCur { base: Sqlite3VtabCursor { p_vtab: ptr::null_mut() }, i: 0 });
    *pp = Box::into_raw(c) as *mut Sqlite3VtabCursor; SQLITE_OK }
unsafe extern "C" fn ser_close(c: *mut Sqlite3VtabCursor) -> c_int { drop(Box::from_raw(c as *mut SerCur)); SQLITE_OK }
unsafe extern "C" fn ser_filter(c: *mut Sqlite3VtabCursor, _n: c_int, _s: *const c_char,
        _ac: c_int, _av: *mut *mut Sqlite3Value) -> c_int { (*(c as *mut SerCur)).i = 1; SQLITE_OK }
unsafe extern "C" fn ser_next(c: *mut Sqlite3VtabCursor) -> c_int { (*(c as *mut SerCur)).i += 1; SQLITE_OK }
unsafe extern "C" fn ser_eof(c: *mut Sqlite3VtabCursor) -> c_int {
    let cur = &*(c as *mut SerCur);
    (cur.i > (*(cur.base.p_vtab as *mut SerVtab)).n) as c_int }
unsafe extern "C" fn ser_col(c: *mut Sqlite3VtabCursor, x: *mut Sqlite3Context, _i: c_int) -> c_int {
    sqlite3_result_int64(x, (*(c as *mut SerCur)).i); SQLITE_OK }
unsafe extern "C" fn ser_rowid(c: *mut Sqlite3VtabCursor, p: *mut i64) -> c_int {
    *p = (*(c as *mut SerCur)).i; SQLITE_OK }
static SER_MOD: Sqlite3Module = Sqlite3Module {
    i_version: 1,
    x_create: Some(ser_create), x_connect: Some(ser_connect),
    x_best_index: Some(ser_best), x_disconnect: Some(ser_disc), x_destroy: Some(ser_dest),
    x_open: Some(ser_open), x_close: Some(ser_close),
    x_filter: Some(ser_filter), x_next: Some(ser_next), x_eof: Some(ser_eof),
    x_column: Some(ser_col), x_rowid: Some(ser_rowid),
    x_update: None, x_begin: None, x_sync: None, x_commit: None, x_rollback: None,
    x_find_function: None, x_rename: None, x_savepoint: None, x_release: None,
    x_rollback_to: None, x_shadow_name: None, x_integrity: None,
};
static EPO_MOD: Sqlite3Module = Sqlite3Module {
    i_version: 1,
    x_create: None, x_connect: Some(ser_connect),
    x_best_index: Some(ser_best), x_disconnect: Some(ser_disc), x_destroy: Some(ser_disc),
    x_open: Some(ser_open), x_close: Some(ser_close),
    x_filter: Some(ser_filter), x_next: Some(ser_next), x_eof: Some(ser_eof),
    x_column: Some(ser_col), x_rowid: Some(ser_rowid),
    x_update: None, x_begin: None, x_sync: None, x_commit: None, x_rollback: None,
    x_find_function: None, x_rename: None, x_savepoint: None, x_release: None,
    x_rollback_to: None, x_shadow_name: None, x_integrity: None,
};
unsafe extern "C" fn mdest(_p: *mut c_void) { N_MDEST.fetch_add(1, Ordering::SeqCst); }

// ---- memvt: writable, storage inside the module ----
#[repr(C)] struct MvCur { base: Sqlite3VtabCursor, i: usize }
unsafe extern "C" fn mv_create(db: *mut Sqlite3, _a: *mut c_void, _argc: c_int,
        _argv: *const *const c_char, pp: *mut *mut Sqlite3Vtab, _pz: *mut *mut c_char) -> c_int {
    let rc = sqlite3_declare_vtab(db, c"CREATE TABLE x(a INTEGER, b TEXT)".as_ptr());
    if rc != SQLITE_OK { return rc; }
    let v = Box::new(Sqlite3Vtab { p_module: ptr::null(), n_ref: 0, z_err_msg: ptr::null_mut() });
    *pp = Box::into_raw(v);
    SQLITE_OK }
unsafe extern "C" fn mv_disc(v: *mut Sqlite3Vtab) -> c_int { drop(Box::from_raw(v)); SQLITE_OK }
unsafe extern "C" fn mv_best(_v: *mut Sqlite3Vtab, _i: *mut Sqlite3IndexInfo) -> c_int { SQLITE_OK }
unsafe extern "C" fn mv_open(_v: *mut Sqlite3Vtab, pp: *mut *mut Sqlite3VtabCursor) -> c_int {
    let c = Box::new(MvCur { base: Sqlite3VtabCursor { p_vtab: ptr::null_mut() }, i: 0 });
    *pp = Box::into_raw(c) as *mut Sqlite3VtabCursor; SQLITE_OK }
unsafe extern "C" fn mv_close(c: *mut Sqlite3VtabCursor) -> c_int { drop(Box::from_raw(c as *mut MvCur)); SQLITE_OK }
unsafe extern "C" fn mv_filter(c: *mut Sqlite3VtabCursor, _n: c_int, _s: *const c_char,
        _ac: c_int, _av: *mut *mut Sqlite3Value) -> c_int { (*(c as *mut MvCur)).i = 0; SQLITE_OK }
unsafe extern "C" fn mv_next(c: *mut Sqlite3VtabCursor) -> c_int { (*(c as *mut MvCur)).i += 1; SQLITE_OK }
unsafe extern "C" fn mv_eof(c: *mut Sqlite3VtabCursor) -> c_int {
    ((*(c as *mut MvCur)).i >= mem().len()) as c_int }
unsafe extern "C" fn mv_col(c: *mut Sqlite3VtabCursor, x: *mut Sqlite3Context, i: c_int) -> c_int {
    let idx = (*(c as *mut MvCur)).i;
    let m = mem();
    if let Some(r) = m.get(idx) {
        if i == 0 { sqlite3_result_int64(x, r.a); }
        else {
            let t = CString::new(r.b.clone()).unwrap_or_default();
            sqlite3_result_text(x, t.as_ptr(), -1, ptr::null_mut());
        }
    }
    SQLITE_OK }
unsafe extern "C" fn mv_rowid(c: *mut Sqlite3VtabCursor, p: *mut i64) -> c_int {
    let idx = (*(c as *mut MvCur)).i;
    *p = mem().get(idx).map(|r| r.rowid).unwrap_or(0); SQLITE_OK }
fn vtn(v: *mut Sqlite3Value) -> &'static str {
    match unsafe { sqlite3_value_type(v) } { 1 => "I", 2 => "F", 3 => "T", 4 => "B", _ => "N" }
}
unsafe extern "C" fn mv_update(_v: *mut Sqlite3Vtab, argc: c_int, argv: *mut *mut Sqlite3Value,
        p_rowid: *mut i64) -> c_int {
    if argc == 1 {
        let rid = sqlite3_value_int64(*argv);
        uplog(format!("[DEL argc=1 rid={rid}]"));
        mem().retain(|r| r.rowid != rid);
        return SQLITE_OK;
    }
    let a = sqlite3_value_int64(*argv.add(2));
    let bp = sqlite3_value_text(*argv.add(3));
    let b = if bp.is_null() { String::new() } else { CStr::from_ptr(bp as *const c_char).to_string_lossy().into_owned() };
    if a < 0 { return 19; } // module-side constraint
    if sqlite3_value_type(*argv) == 5 /* NULL */ {
        let rid = if sqlite3_value_type(*argv.add(1)) == 5 {
            mem().iter().map(|r| r.rowid).max().unwrap_or(0) + 1
        } else { sqlite3_value_int64(*argv.add(1)) };
        uplog(format!("[INS argc={argc} a0={} a1={} rid={rid}]", vtn(*argv), vtn(*argv.add(1))));
        mem().push(MemRow { rowid: rid, a, b });
        *p_rowid = rid;
        return SQLITE_OK;
    }
    let old = sqlite3_value_int64(*argv);
    let nw = sqlite3_value_int64(*argv.add(1));
    uplog(format!("[UPD argc={argc} old={old} new={nw}]"));
    for r in mem().iter_mut() { if r.rowid == old { r.rowid = nw; r.a = a; r.b = b.clone(); break; } }
    SQLITE_OK }
static MV_MOD: Sqlite3Module = Sqlite3Module {
    i_version: 1,
    x_create: Some(mv_create), x_connect: Some(mv_create),
    x_best_index: Some(mv_best), x_disconnect: Some(mv_disc), x_destroy: Some(mv_disc),
    x_open: Some(mv_open), x_close: Some(mv_close),
    x_filter: Some(mv_filter), x_next: Some(mv_next), x_eof: Some(mv_eof),
    x_column: Some(mv_col), x_rowid: Some(mv_rowid),
    x_update: Some(mv_update), x_begin: None, x_sync: None, x_commit: None, x_rollback: None,
    x_find_function: None, x_rename: None, x_savepoint: None, x_release: None,
    x_rollback_to: None, x_shadow_name: None, x_integrity: None,
};

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn bare(cid: &'static str) -> H { H { cid, lines: Vec::new(), db: ptr::null_mut() } }
    fn open_mem(&mut self) { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        self.db = db; } }
    fn open_path(&mut self, path: &str) { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        self.db = db; } }
    fn close(&mut self) { unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } } }
    fn ex(&mut self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn exr(&mut self, label: &str, s: &str) { unsafe {
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let e = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={}", self.cid, label, rc, e));
        sqlite3_free(em as *mut c_void);
    } }
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
    fn check(mut self) {
        self.close();
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-vtab38/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}

// ================= 001: xConnect on file reopen =================

#[test] fn v001() { let _g = lockg(); unsafe {
    let path = format!("/tmp/v38a-rs-{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{path}-journal"));
    let mut h = H::bare("engine-vtab38-001-C001");
    h.open_path(&path);
    sqlite3_create_module(h.db, c"ser".as_ptr(), &SER_MOD, ptr::null_mut());
    N_CREATE.store(0, Ordering::SeqCst); N_CONNECT.store(0, Ordering::SeqCst);
    N_DISC.store(0, Ordering::SeqCst); N_DESTROY.store(0, Ordering::SeqCst);
    h.exr("cvt", "CREATE VIRTUAL TABLE vt USING ser(4);");
    h.oi("create_calls", N_CREATE.load(Ordering::SeqCst));
    h.oi("connect_calls", N_CONNECT.load(Ordering::SeqCst));
    h.rows("scan", "SELECT value FROM vt");
    h.close();
    h.oi("disc_at_close", N_DISC.load(Ordering::SeqCst));
    h.oi("destroy_at_close", N_DESTROY.load(Ordering::SeqCst));
    h.check();

    let mut h = H::bare("engine-vtab38-001-C002");
    h.open_path(&path);
    h.rows("noreg_select", "SELECT value FROM vt");
    h.rows("master", "SELECT type,name,rootpage,sql FROM sqlite_master WHERE name='vt'");
    sqlite3_create_module(h.db, c"ser".as_ptr(), &SER_MOD, ptr::null_mut());
    N_CREATE.store(0, Ordering::SeqCst); N_CONNECT.store(0, Ordering::SeqCst);
    h.rows("rereg_select", "SELECT value FROM vt");
    h.oi("reopen_create_calls", N_CREATE.load(Ordering::SeqCst));
    h.oi("reopen_connect_calls", N_CONNECT.load(Ordering::SeqCst));
    N_DESTROY.store(0, Ordering::SeqCst);
    h.exr("drop", "DROP TABLE vt;");
    h.oi("destroy_after_drop", N_DESTROY.load(Ordering::SeqCst));
    h.check();
} }

// ================= 002: drop_modules =================

#[test] fn v002_c001() { let _g = lockg(); unsafe {
    let mut h = H::bare("engine-vtab38-002-C001");
    h.open_mem();
    sqlite3_create_module(h.db, c"ser".as_ptr(), &SER_MOD, ptr::null_mut());
    sqlite3_create_module(h.db, c"memvt".as_ptr(), &MV_MOD, ptr::null_mut());
    let keep = [c"memvt".as_ptr(), ptr::null()];
    h.oi("drop_keep_memvt_rc", sqlite3_drop_modules(h.db, keep.as_ptr()) as i64);
    h.exr("cvt_ser_dropped", "CREATE VIRTUAL TABLE a USING ser;");
    h.exr("cvt_memvt_kept", "CREATE VIRTUAL TABLE b USING memvt;");
    h.check();
} }

#[test] fn v002_c002() { let _g = lockg(); unsafe {
    let mut h = H::bare("engine-vtab38-002-C002");
    h.open_mem();
    sqlite3_create_module(h.db, c"ser".as_ptr(), &SER_MOD, ptr::null_mut());
    h.ex("CREATE VIRTUAL TABLE vt USING ser(3);");
    h.oi("drop_all_with_live_rc", sqlite3_drop_modules(h.db, ptr::null()) as i64);
    h.rows("live_select_after_drop", "SELECT value FROM vt");
    h.exr("cvt_after_dropall", "CREATE VIRTUAL TABLE v2 USING ser;");
    h.exr("drop_table_after_dropall", "DROP TABLE vt;");
    h.check();
} }

// ================= 003: destructor deferral =================

#[test] fn v003() { let _g = lockg(); unsafe {
    let mut h = H::bare("engine-vtab38-003-C001");
    h.open_mem();
    N_MDEST.store(0, Ordering::SeqCst);
    sqlite3_create_module_v2(h.db, c"ser".as_ptr(), &SER_MOD, ptr::null_mut(), Some(mdest));
    h.ex("CREATE VIRTUAL TABLE vt USING ser(2);");
    sqlite3_create_module_v2(h.db, c"ser".as_ptr(), &SER_MOD, ptr::null_mut(), Some(mdest));
    h.oi("mdest_after_replace_with_live", N_MDEST.load(Ordering::SeqCst));
    h.ex("DROP TABLE vt;");
    h.oi("mdest_after_drop", N_MDEST.load(Ordering::SeqCst));
    h.close();
    h.oi("mdest_after_close", N_MDEST.load(Ordering::SeqCst));
    h.check();
} }

// ================= 004: xUpdate =================

#[test] fn v004() { let _g = lockg(); unsafe {
    let mut h = H::bare("engine-vtab38-004-C001");
    h.open_mem();
    sqlite3_create_module(h.db, c"memvt".as_ptr(), &MV_MOD, ptr::null_mut());
    mem().clear(); UPLOG.lock().unwrap_or_else(|e| e.into_inner()).clear();
    h.ex("CREATE VIRTUAL TABLE m USING memvt;");
    h.exr("ins_auto", "INSERT INTO m VALUES(10,'x');");
    h.oi("last_rowid_auto", sqlite3_last_insert_rowid(h.db));
    h.exr("ins_explicit", "INSERT INTO m(rowid,a,b) VALUES(7,20,'y');");
    h.oi("last_rowid_explicit", sqlite3_last_insert_rowid(h.db));
    h.rows("scan", "SELECT rowid, a, b FROM m ORDER BY rowid");
    let db = h.db; h.db = ptr::null_mut(); // keep the connection across cases
    h.check();

    let mut h = H { cid: "engine-vtab38-004-C002", lines: Vec::new(), db };
    h.exr("upd", "UPDATE m SET a = 99 WHERE b = 'x';");
    h.rows("scan2", "SELECT rowid, a, b FROM m ORDER BY rowid");
    h.exr("del", "DELETE FROM m WHERE a = 20;");
    h.rows("scan3", "SELECT rowid, a, b FROM m ORDER BY rowid");
    let db = h.db; h.db = ptr::null_mut();
    h.check();

    let mut h = H { cid: "engine-vtab38-004-C003", lines: Vec::new(), db };
    h.exr("constraint_ins", "INSERT INTO m VALUES(-5,'bad');");
    h.rows("count_after_constraint", "SELECT count(*) FROM m");
    let l = UPLOG.lock().unwrap_or_else(|e| e.into_inner()).clone();
    h.lines.push(format!("OBS {} uplog {}", h.cid, l));
    h.check();
} }

// ================= 005: eponymous-only =================

#[test] fn v005() { let _g = lockg(); unsafe {
    let mut h = H::bare("engine-vtab38-005-C001");
    h.open_mem();
    sqlite3_create_module(h.db, c"epo".as_ptr(), &EPO_MOD, ptr::null_mut());
    N_CREATE.store(0, Ordering::SeqCst); N_CONNECT.store(0, Ordering::SeqCst);
    h.rows("epo_select", "SELECT value FROM epo");
    h.oi("epo_create_calls", N_CREATE.load(Ordering::SeqCst));
    h.oi("epo_connect_calls", N_CONNECT.load(Ordering::SeqCst));
    h.exr("epo_cvt", "CREATE VIRTUAL TABLE e USING epo;");
    h.rows("epo_master", "SELECT count(*) FROM sqlite_master");
    h.check();
} }

// ================= anti-cheat =================

#[test] fn anti_cheat_v38_reconnect_runtime() { let _g = lockg(); unsafe {
    // runtime series bound survives the file round-trip and reconnects via xConnect
    let seed = (std::process::id() % 7) as i64 + 2;
    let path = format!("/tmp/v38ac-{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path.clone()).unwrap().as_ptr(), &mut db);
    sqlite3_create_module(db, c"ser".as_ptr(), &SER_MOD, ptr::null_mut());
    sqlite3_exec(db, CString::new(format!("CREATE VIRTUAL TABLE vt USING ser({seed});")).unwrap().as_ptr(),
        None, ptr::null_mut(), ptr::null_mut());
    sqlite3_close(db);
    let mut db2: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db2);
    sqlite3_create_module(db2, c"ser".as_ptr(), &SER_MOD, ptr::null_mut());
    N_CREATE.store(0, Ordering::SeqCst); N_CONNECT.store(0, Ordering::SeqCst);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db2, c"SELECT count(*) FROM vt".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed, "runtime bound survives the reload through xConnect argv");
    sqlite3_finalize(st);
    assert_eq!(N_CREATE.load(Ordering::SeqCst), 0, "reopen must NOT call xCreate");
    assert!(N_CONNECT.load(Ordering::SeqCst) > 0, "reopen must call xConnect");
    sqlite3_close(db2);
} }

#[test] fn anti_cheat_v38_xupdate_runtime() { let _g = lockg(); unsafe {
    // runtime payload written through xUpdate lands in the module's own storage
    let seed = (std::process::id() % 100000) as i64;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_create_module(db, c"memvt".as_ptr(), &MV_MOD, ptr::null_mut());
    mem().clear();
    sqlite3_exec(db, c"CREATE VIRTUAL TABLE m USING memvt;".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    sqlite3_exec(db, CString::new(format!("INSERT INTO m VALUES({seed},'p{seed}');")).unwrap().as_ptr(),
        None, ptr::null_mut(), ptr::null_mut());
    assert_eq!(mem().len(), 1, "module storage received the row");
    assert_eq!(mem()[0].a, seed, "runtime payload reached xUpdate argv");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT a, b FROM m".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed);
    assert_eq!(CStr::from_ptr(sqlite3_column_text(st, 1) as *const c_char).to_string_lossy(),
        format!("p{seed}"), "scan reads the module's storage back");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }
