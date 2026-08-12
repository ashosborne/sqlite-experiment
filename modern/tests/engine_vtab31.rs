//! Run-41 vtab-core replay — mirrors /tmp/vtab_harness.c.
//! Plain language: register a module with sqlite3_create_module(_v2); CREATE VIRTUAL
//! TABLE runs xCreate which declares columns via sqlite3_declare_vtab; SELECT projects
//! rows through the module cursor (xOpen/xBestIndex/xFilter/xEof/xColumn/xNext/xClose).
//! The intseries/pairtab modules here are the same tiny in-process test modules the C
//! harness compiled against the bare pinned amalgamation — not ext/ extensions.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

// counters + argv capture are process-global (like the C harness statics); the tests
// that observe them run serialized behind this lock.
static LOCK: Mutex<()> = Mutex::new(());
static G_DESTROY: AtomicI32 = AtomicI32::new(0);
static G_MDEST: AtomicI32 = AtomicI32::new(0);
static LASTARGV: Mutex<String> = Mutex::new(String::new());

fn lockg() -> std::sync::MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// error string allocated so modern's sqlite3_free can reclaim it (C used sqlite3_mprintf)
unsafe fn errstr(s: &str) -> *mut c_char {
    let bytes = s.as_bytes();
    let p = sqlite3_malloc64(bytes.len() as u64 + 1) as *mut u8;
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
    *p.add(bytes.len()) = 0;
    p as *mut c_char
}

// ---- intseries: 1..N with a HIDDEN lim column (same shape as the C harness module) ----
#[repr(C)]
struct SeriesVtab { base: Sqlite3Vtab, n: i64 }
#[repr(C)]
struct SeriesCur { base: Sqlite3VtabCursor, i: i64 }

unsafe extern "C" fn series_create(db: *mut Sqlite3, _aux: *mut c_void, argc: c_int,
        argv: *const *const c_char, pp: *mut *mut Sqlite3Vtab, pz_err: *mut *mut c_char) -> c_int {
    let mut n: i64 = 3;
    let args: Vec<String> = (0..argc as usize)
        .map(|i| CStr::from_ptr(*argv.add(i)).to_string_lossy().into_owned()).collect();
    *LASTARGV.lock().unwrap_or_else(|e| e.into_inner()) = args.join("/");
    if argc > 3 {
        let a = &args[3];
        match a.parse::<i64>() {
            Ok(v) if v > 0 => n = v,
            _ => { *pz_err = errstr(&format!("intseries: bad limit '{a}'")); return SQLITE_ERROR; }
        }
    }
    let rc = sqlite3_declare_vtab(db, c"CREATE TABLE x(value INTEGER, lim HIDDEN INTEGER)".as_ptr());
    if rc != SQLITE_OK { return rc; }
    let v = Box::new(SeriesVtab {
        base: Sqlite3Vtab { p_module: ptr::null(), n_ref: 0, z_err_msg: ptr::null_mut() }, n });
    *pp = Box::into_raw(v) as *mut Sqlite3Vtab;
    SQLITE_OK
}
unsafe extern "C" fn series_disconnect(v: *mut Sqlite3Vtab) -> c_int {
    drop(Box::from_raw(v as *mut SeriesVtab)); SQLITE_OK }
unsafe extern "C" fn series_destroy(v: *mut Sqlite3Vtab) -> c_int {
    G_DESTROY.fetch_add(1, Ordering::SeqCst);
    drop(Box::from_raw(v as *mut SeriesVtab)); SQLITE_OK }
unsafe extern "C" fn series_best_index(_v: *mut Sqlite3Vtab, _i: *mut Sqlite3IndexInfo) -> c_int { SQLITE_OK }
unsafe extern "C" fn series_open(_v: *mut Sqlite3Vtab, pp: *mut *mut Sqlite3VtabCursor) -> c_int {
    let c = Box::new(SeriesCur { base: Sqlite3VtabCursor { p_vtab: ptr::null_mut() }, i: 0 });
    *pp = Box::into_raw(c) as *mut Sqlite3VtabCursor; SQLITE_OK }
unsafe extern "C" fn series_close(c: *mut Sqlite3VtabCursor) -> c_int {
    drop(Box::from_raw(c as *mut SeriesCur)); SQLITE_OK }
unsafe extern "C" fn series_filter(c: *mut Sqlite3VtabCursor, _n: c_int, _s: *const c_char,
        _ac: c_int, _av: *mut *mut Sqlite3Value) -> c_int { (*(c as *mut SeriesCur)).i = 1; SQLITE_OK }
unsafe extern "C" fn series_next(c: *mut Sqlite3VtabCursor) -> c_int {
    (*(c as *mut SeriesCur)).i += 1; SQLITE_OK }
unsafe extern "C" fn series_eof(c: *mut Sqlite3VtabCursor) -> c_int {
    let cur = &*(c as *mut SeriesCur);
    let vt = &*(cur.base.p_vtab as *mut SeriesVtab);
    (cur.i > vt.n) as c_int }
unsafe extern "C" fn series_column(c: *mut Sqlite3VtabCursor, ctx: *mut Sqlite3Context, i: c_int) -> c_int {
    let cur = &*(c as *mut SeriesCur);
    if i == 0 { sqlite3_result_int64(ctx, cur.i); }
    else { sqlite3_result_int64(ctx, (*(cur.base.p_vtab as *mut SeriesVtab)).n); }
    SQLITE_OK }
unsafe extern "C" fn series_rowid(c: *mut Sqlite3VtabCursor, p: *mut i64) -> c_int {
    *p = (*(c as *mut SeriesCur)).i; SQLITE_OK }
unsafe extern "C" fn module_destroy(_aux: *mut c_void) { G_MDEST.fetch_add(1, Ordering::SeqCst); }

static SERIES_MOD: Sqlite3Module = Sqlite3Module {
    i_version: 1,
    x_create: Some(series_create), x_connect: Some(series_create),
    x_best_index: Some(series_best_index),
    x_disconnect: Some(series_disconnect), x_destroy: Some(series_destroy),
    x_open: Some(series_open), x_close: Some(series_close),
    x_filter: Some(series_filter), x_next: Some(series_next), x_eof: Some(series_eof),
    x_column: Some(series_column), x_rowid: Some(series_rowid),
    x_update: None, x_begin: None, x_sync: None, x_commit: None, x_rollback: None,
    x_find_function: None, x_rename: None, x_savepoint: None, x_release: None,
    x_rollback_to: None, x_shadow_name: None, x_integrity: None,
};

// ---- pairtab: fixed two-row (a INTEGER, b TEXT) table through the cursor ----
#[repr(C)]
struct PairCur { base: Sqlite3VtabCursor, i: i32 }

unsafe extern "C" fn pair_create(db: *mut Sqlite3, _aux: *mut c_void, _argc: c_int,
        _argv: *const *const c_char, pp: *mut *mut Sqlite3Vtab, _pz: *mut *mut c_char) -> c_int {
    let rc = sqlite3_declare_vtab(db, c"CREATE TABLE x(a INTEGER, b TEXT)".as_ptr());
    if rc != SQLITE_OK { return rc; }
    let v = Box::new(Sqlite3Vtab { p_module: ptr::null(), n_ref: 0, z_err_msg: ptr::null_mut() });
    *pp = Box::into_raw(v);
    SQLITE_OK }
unsafe extern "C" fn pair_disconnect(v: *mut Sqlite3Vtab) -> c_int {
    drop(Box::from_raw(v)); SQLITE_OK }
unsafe extern "C" fn pair_best_index(_v: *mut Sqlite3Vtab, _i: *mut Sqlite3IndexInfo) -> c_int { SQLITE_OK }
unsafe extern "C" fn pair_open(_v: *mut Sqlite3Vtab, pp: *mut *mut Sqlite3VtabCursor) -> c_int {
    let c = Box::new(PairCur { base: Sqlite3VtabCursor { p_vtab: ptr::null_mut() }, i: 0 });
    *pp = Box::into_raw(c) as *mut Sqlite3VtabCursor; SQLITE_OK }
unsafe extern "C" fn pair_close(c: *mut Sqlite3VtabCursor) -> c_int {
    drop(Box::from_raw(c as *mut PairCur)); SQLITE_OK }
unsafe extern "C" fn pair_filter(c: *mut Sqlite3VtabCursor, _n: c_int, _s: *const c_char,
        _ac: c_int, _av: *mut *mut Sqlite3Value) -> c_int { (*(c as *mut PairCur)).i = 0; SQLITE_OK }
unsafe extern "C" fn pair_next(c: *mut Sqlite3VtabCursor) -> c_int {
    (*(c as *mut PairCur)).i += 1; SQLITE_OK }
unsafe extern "C" fn pair_eof(c: *mut Sqlite3VtabCursor) -> c_int {
    ((*(c as *mut PairCur)).i > 1) as c_int }
unsafe extern "C" fn pair_column(c: *mut Sqlite3VtabCursor, ctx: *mut Sqlite3Context, i: c_int) -> c_int {
    let cur = &*(c as *mut PairCur);
    if i == 0 { sqlite3_result_int(ctx, cur.i + 1); }
    else {
        let t = if cur.i == 0 { c"one" } else { c"two" };
        sqlite3_result_text(ctx, t.as_ptr(), -1, ptr::null_mut());
    }
    SQLITE_OK }
unsafe extern "C" fn pair_rowid(c: *mut Sqlite3VtabCursor, p: *mut i64) -> c_int {
    *p = ((*(c as *mut PairCur)).i + 1) as i64; SQLITE_OK }

static PAIR_MOD: Sqlite3Module = Sqlite3Module {
    i_version: 1,
    x_create: Some(pair_create), x_connect: Some(pair_create),
    x_best_index: Some(pair_best_index),
    x_disconnect: Some(pair_disconnect), x_destroy: Some(pair_disconnect),
    x_open: Some(pair_open), x_close: Some(pair_close),
    x_filter: Some(pair_filter), x_next: Some(pair_next), x_eof: Some(pair_eof),
    x_column: Some(pair_column), x_rowid: Some(pair_rowid),
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
    fn reg(&mut self) { unsafe { sqlite3_create_module(self.db, c"intseries".as_ptr(), &SERIES_MOD, ptr::null_mut()); } }
    fn reg_pair(&mut self) { unsafe { sqlite3_create_module(self.db, c"pairtab".as_ptr(), &PAIR_MOD, ptr::null_mut()); } }
    fn obs(&mut self, tail: String) { self.lines.push(format!("OBS {} {}", self.cid, tail)); }
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
    fn colnames(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={} err={}", self.cid, label, rc, CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy())); return; }
        let n = sqlite3_column_count(st);
        let mut buf = String::new();
        for i in 0..n {
            if i > 0 { buf.push(','); }
            buf.push_str(&CStr::from_ptr(sqlite3_column_name(st, i)).to_string_lossy());
        }
        sqlite3_finalize(st);
        self.lines.push(format!("OBS {} {} n={} names={}", self.cid, label, n, buf));
    } }
    fn close_db(&mut self) { unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } } }
    fn reopen(&mut self) { unsafe {
        self.close_db();
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        self.db = db;
    } }
    fn check(mut self) {
        self.close_db();
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-vtab31/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}

// ================= A — module lifecycle (engine-vtab31-001) =================

#[test] fn a001() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C001");
    let rc = unsafe { sqlite3_create_module(h.db, c"intseries".as_ptr(), &SERIES_MOD, ptr::null_mut()) };
    h.obs(format!("reg rc={rc}"));
    h.exr("cvt", "CREATE VIRTUAL TABLE nums USING intseries(5);");
    h.rows("master", "SELECT type, name, tbl_name, rootpage, sql FROM sqlite_master WHERE name='nums'");
    h.check(); }

#[test] fn a002() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C002");
    h.exr("unknown", "CREATE VIRTUAL TABLE t USING nosuch;");
    h.rows("master_count", "SELECT count(*) FROM sqlite_master");
    h.check(); }

#[test] fn a003() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C003"); h.reg();
    h.exr("badarg", "CREATE VIRTUAL TABLE bad USING intseries(bogus);");
    h.rows("master_count", "SELECT count(*) FROM sqlite_master WHERE name='bad'");
    h.check(); }

#[test] fn a004() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C004"); h.reg();
    G_DESTROY.store(0, Ordering::SeqCst);
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(4);");
    h.exr("drop", "DROP TABLE nums;");
    h.obs(format!("destroyed n={}", G_DESTROY.load(Ordering::SeqCst)));
    h.rows("gone", "SELECT value FROM nums");
    h.check(); }

#[test] fn a005() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C005"); h.reg();
    LASTARGV.lock().unwrap_or_else(|e| e.into_inner()).clear();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(7);");
    let argv = LASTARGV.lock().unwrap_or_else(|e| e.into_inner()).clone();
    h.obs(format!("argv {argv}"));
    h.check(); }

#[test] fn a006() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C006");
    G_MDEST.store(0, Ordering::SeqCst);
    unsafe { sqlite3_create_module_v2(h.db, c"modv".as_ptr(), &SERIES_MOD, ptr::null_mut(), Some(module_destroy)); }
    h.obs(format!("after_reg n={}", G_MDEST.load(Ordering::SeqCst)));
    unsafe { sqlite3_create_module_v2(h.db, c"modv".as_ptr(), &SERIES_MOD, ptr::null_mut(), Some(module_destroy)); }
    h.obs(format!("after_replace n={}", G_MDEST.load(Ordering::SeqCst)));
    h.close_db();
    h.obs(format!("after_close n={}", G_MDEST.load(Ordering::SeqCst)));
    h.check(); }

#[test] fn a007() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C007"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(2);");
    h.reopen(); // fresh connection: no module registered
    h.exr("fresh_cvt", "CREATE VIRTUAL TABLE nums USING intseries(2);");
    h.check(); }

#[test] fn a008() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C008"); h.reg();
    h.ex("CREATE VIRTUAL TABLE n3 USING intseries(3); CREATE VIRTUAL TABLE n5 USING intseries(5);");
    h.rows("n3", "SELECT value FROM n3");
    h.rows("n5", "SELECT value FROM n5");
    h.check(); }

#[test] fn a009() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C009"); h.reg();
    h.exr("noargs", "CREATE VIRTUAL TABLE d USING intseries;");
    h.rows("dflt", "SELECT value FROM d");
    h.check(); }

#[test] fn a010() { let _g = lockg(); let mut h = H::new("engine-vtab31-001-C010"); h.reg();
    h.ex("CREATE VIRTUAL TABLE r USING intseries(2);");
    h.rows("before", "SELECT value FROM r");
    h.ex("DROP TABLE r; CREATE VIRTUAL TABLE r USING intseries(6);");
    h.rows("after", "SELECT value FROM r");
    h.check(); }

// ================= B — declare_vtab shape (engine-vtab31-002) =================

#[test] fn b001() { let _g = lockg(); let mut h = H::new("engine-vtab31-002-C001"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(4);");
    h.colnames("star", "SELECT * FROM nums");
    h.check(); }

#[test] fn b002() { let _g = lockg(); let mut h = H::new("engine-vtab31-002-C002"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(4);");
    h.rows("byname", "SELECT value FROM nums");
    h.rows("hidden_byname", "SELECT lim FROM nums");
    h.check(); }

#[test] fn b003() { let _g = lockg(); let mut h = H::new("engine-vtab31-002-C003"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(4);");
    h.rows("tinfo", "SELECT name, type FROM pragma_table_info('nums')");
    h.check(); }

#[test] fn b004() { let _g = lockg(); let mut h = H::new("engine-vtab31-002-C004"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(3);");
    h.rows("star_rows", "SELECT * FROM nums");
    h.check(); }

#[test] fn b005() { let _g = lockg(); let mut h = H::new("engine-vtab31-002-C005"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(2);");
    h.rows("typeof", "SELECT typeof(value) FROM nums");
    h.check(); }

#[test] fn b006() { let _g = lockg(); let mut h = H::new("engine-vtab31-002-C006");
    let rc = unsafe { sqlite3_declare_vtab(h.db, c"CREATE TABLE x(a)".as_ptr()) };
    h.obs(format!("misuse rc={rc}"));
    h.check(); }

#[test] fn b007() { let _g = lockg(); let mut h = H::new("engine-vtab31-002-C007"); h.reg(); h.reg_pair();
    h.ex("CREATE VIRTUAL TABLE p USING pairtab;");
    h.colnames("shape", "SELECT * FROM p");
    h.rows("both", "SELECT a, b FROM p");
    h.check(); }

#[test] fn b008() { let _g = lockg(); let mut h = H::new("engine-vtab31-002-C008"); h.reg_pair();
    h.ex("CREATE VIRTUAL TABLE p USING pairtab;");
    h.rows("types", "SELECT typeof(a), typeof(b) FROM p");
    h.rows("tinfo", "SELECT name, type FROM pragma_table_info('p')");
    h.check(); }

// ================= C — SELECT through the vtab cursor (engine-vtab31-003) =================

#[test] fn c001() { let _g = lockg(); let mut h = H::new("engine-vtab31-003-C001"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(5);");
    h.rows("scan", "SELECT * FROM nums");
    h.check(); }

#[test] fn c002() { let _g = lockg(); let mut h = H::new("engine-vtab31-003-C002"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(5);");
    h.rows("where", "SELECT value FROM nums WHERE value > 2");
    h.check(); }

#[test] fn c003() { let _g = lockg(); let mut h = H::new("engine-vtab31-003-C003"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(5);");
    h.rows("agg", "SELECT count(*), sum(value) FROM nums");
    h.check(); }

#[test] fn c004() { let _g = lockg(); let mut h = H::new("engine-vtab31-003-C004"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(5);");
    h.ex("CREATE TABLE t(k INTEGER, s TEXT); INSERT INTO t VALUES(2,'two'),(4,'four');");
    h.rows("join", "SELECT value, s FROM nums JOIN t ON t.k = value ORDER BY value");
    h.check(); }

#[test] fn c005() { let _g = lockg(); let mut h = H::new("engine-vtab31-003-C005"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(5);");
    h.rows("desc", "SELECT value FROM nums ORDER BY value DESC");
    h.check(); }

#[test] fn c006() { let _g = lockg(); let mut h = H::new("engine-vtab31-003-C006"); h.reg();
    h.ex("CREATE VIRTUAL TABLE n3 USING intseries(3); CREATE VIRTUAL TABLE n2 USING intseries(2);");
    h.rows("two", "SELECT (SELECT sum(value) FROM n3), (SELECT sum(value) FROM n2)");
    h.check(); }

#[test] fn c007() { let _g = lockg(); let mut h = H::new("engine-vtab31-003-C007"); h.reg();
    h.ex("CREATE VIRTUAL TABLE nums USING intseries(5);");
    h.rows("hidden_where", "SELECT value FROM nums WHERE lim = 5");
    h.rows("hidden_where_miss", "SELECT value FROM nums WHERE lim = 4");
    h.check(); }

// ================= D — anti-cheat: runtime names / payloads / reshape =================

#[test] fn anti_cheat_vtab31_runtime_names() { let _g = lockg(); unsafe {
    // module + table names and the row payload are computed at runtime — a canned
    // answer table cannot know them.
    let seed = (std::process::id() % 1000) as i64;
    let n = seed % 7 + 2;
    let (mname, tname) = (format!("m{seed}"), format!("vt{seed}"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_create_module(db, CString::new(mname.clone()).unwrap().as_ptr(), &SERIES_MOD, ptr::null_mut());
    let rc = sqlite3_exec(db, CString::new(format!("CREATE VIRTUAL TABLE {tname} USING {mname}({n});")).unwrap().as_ptr(),
        None, ptr::null_mut(), ptr::null_mut());
    assert_eq!(rc, 0, "runtime-named module must create");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT count(*), sum(value) FROM {tname}")).unwrap().as_ptr(),
        -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), n, "row count must equal the runtime arg");
    assert_eq!(sqlite3_column_int64(st, 1), n * (n + 1) / 2, "payload must come from the cursor");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_vtab31_reshape() { let _g = lockg(); unsafe {
    // drop + recreate the same table name with a different module: the shape and the
    // payload must both change (schema comes from declare_vtab, rows from the cursor).
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_create_module(db, c"intseries".as_ptr(), &SERIES_MOD, ptr::null_mut());
    sqlite3_create_module(db, c"pairtab".as_ptr(), &PAIR_MOD, ptr::null_mut());
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, "CREATE VIRTUAL TABLE shp USING intseries(3);");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT * FROM shp".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_column_count(st), 1);
    assert_eq!(CStr::from_ptr(sqlite3_column_name(st, 0)).to_string_lossy(), "value");
    sqlite3_finalize(st);
    exs(db, "DROP TABLE shp; CREATE VIRTUAL TABLE shp USING pairtab;");
    let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT * FROM shp".as_ptr(), -1, &mut st2, ptr::null_mut());
    assert_eq!(sqlite3_column_count(st2), 2, "recreated vtab must carry the new shape");
    assert_eq!(CStr::from_ptr(sqlite3_column_name(st2, 0)).to_string_lossy(), "a");
    assert_eq!(CStr::from_ptr(sqlite3_column_name(st2, 1)).to_string_lossy(), "b");
    assert_eq!(sqlite3_step(st2), 100);
    assert_eq!(CStr::from_ptr(sqlite3_column_text(st2, 1) as *const c_char).to_string_lossy(), "one");
    sqlite3_finalize(st2);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_vtab31_unknown_module_pin() { let _g = lockg(); unsafe {
    // runtime-random unknown module name -> exact C error text
    let seed = std::process::id() % 1000;
    let bad = format!("zz{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let mut em: *mut c_char = ptr::null_mut();
    let rc = sqlite3_exec(db, CString::new(format!("CREATE VIRTUAL TABLE q USING {bad};")).unwrap().as_ptr(),
        None, ptr::null_mut(), &mut em);
    assert_eq!(rc, 1);
    assert_eq!(CStr::from_ptr(em).to_string_lossy(), format!("no such module: {bad}"));
    sqlite3_free(em as *mut c_void);
    sqlite3_close(db);
} }
