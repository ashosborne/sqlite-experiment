//! Run-47 one-hole replay — mirrors /tmp/h37.c. Each batch closes a named residual:
//! URI/open_v2 flags, authorizer argument strings + IGNORE, multi-page backup,
//! window EXCLUDE / offset RANGE, config + db_config matrices, stmt_isexplain.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static ALOG: Mutex<String> = Mutex::new(String::new());
static ALOCK: Mutex<()> = Mutex::new(());
fn alock() -> std::sync::MutexGuard<'static, ()> { ALOCK.lock().unwrap_or_else(|e| e.into_inner()) }
static IGN_CODE: Mutex<i32> = Mutex::new(0);

unsafe extern "C" fn auth_log(_p: *mut c_void, code: c_int, s1: *const c_char, s2: *const c_char,
        s3: *const c_char, s4: *const c_char) -> c_int {
    let g = |p: *const c_char| if p.is_null() { "~".to_string() } else { CStr::from_ptr(p).to_string_lossy().into_owned() };
    let line = format!("[{}|{}|{}|{}|{}]", code, g(s1), g(s2), g(s3), g(s4));
    ALOG.lock().unwrap_or_else(|e| e.into_inner()).push_str(&line);
    0
}
unsafe extern "C" fn auth_ign(_p: *mut c_void, code: c_int, _a: *const c_char, _b: *const c_char,
        _c: *const c_char, _d: *const c_char) -> c_int {
    if code == *IGN_CODE.lock().unwrap_or_else(|e| e.into_inner()) { 2 } else { 0 }
}

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn bare(cid: &'static str) -> H { H { cid, lines: Vec::new(), db: ptr::null_mut() } }
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
    fn olog(&mut self, label: &str) {
        let l = ALOG.lock().unwrap_or_else(|e| e.into_inner()).clone();
        self.lines.push(format!("OBS {} {} {}", self.cid, label, l));
    }
    fn check(mut self) {
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-harvest37/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
fn clear_log() { ALOG.lock().unwrap_or_else(|e| e.into_inner()).clear(); }

// ================= 001: URI + open_v2 flags =================

#[test] fn h001() { let mut h = H::bare("engine-harvest37-001-C001"); unsafe {
    let pid = std::process::id();
    let real = format!("/tmp/h37u-rs-{pid}.db");
    let _ = std::fs::remove_file(&real);
    let uri = CString::new(format!("file:{real}")).unwrap();
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open(uri.as_ptr(), &mut db);
    h.oi("plain_open_literal_rc", rc as i64);
    sqlite3_close(db);
    let mut db2: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(uri.as_ptr(), &mut db2, 0x2 | 0x4 | 0x40, ptr::null());
    h.oi("v2_uri_open_rc", rc as i64);
    sqlite3_exec(db2, c"CREATE TABLE t(a); INSERT INTO t VALUES(7);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    sqlite3_close(db2);
    h.oi("real_path_created", std::path::Path::new(&real).exists() as i64);
    h.check();
} }

#[test] fn h001_c002() { let mut h = H::bare("engine-harvest37-001-C002"); unsafe {
    let pid = std::process::id();
    let real = format!("/tmp/h37u-rs2-{pid}.db");
    let _ = std::fs::remove_file(&real);
    { let mut db: *mut Sqlite3 = ptr::null_mut();
      sqlite3_open_v2(CString::new(format!("file:{real}")).unwrap().as_ptr(), &mut db, 0x2 | 0x4 | 0x40, ptr::null());
      sqlite3_exec(db, c"CREATE TABLE t(a); INSERT INTO t VALUES(7);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
      sqlite3_close(db); }
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(CString::new(format!("file:{real}?mode=ro")).unwrap().as_ptr(), &mut db, 0x2 | 0x4 | 0x40, ptr::null());
    h.oi("mode_ro_open_rc", rc as i64);
    let mut em: *mut c_char = ptr::null_mut();
    let rc2 = sqlite3_exec(db, c"INSERT INTO t VALUES(9);".as_ptr(), None, ptr::null_mut(), &mut em);
    let e = if em.is_null() { "-".into() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
    h.lines.push(format!("OBS {} mode_ro_write rc={} err={}", h.cid, rc2, e));
    sqlite3_free(em as *mut c_void);
    sqlite3_close(db);
    let mut db3: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(CString::new(format!("file:{real}?vfs=nosuchvfs")).unwrap().as_ptr(), &mut db3, 0x2 | 0x40, ptr::null());
    let m = if db3.is_null() { "(null)".into() } else { CStr::from_ptr(sqlite3_errmsg(db3)).to_string_lossy().into_owned() };
    h.lines.push(format!("OBS {} bad_vfs rc={} msg={}", h.cid, rc, m));
    sqlite3_close(db3);
    h.check();
} }

#[test] fn h001_c003() { let mut h = H::bare("engine-harvest37-001-C003"); unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(c"file:h37nodisk-rs?mode=memory".as_ptr(), &mut db, 0x2 | 0x4 | 0x40, ptr::null());
    h.oi("mode_memory_rc", rc as i64);
    sqlite3_exec(db, c"CREATE TABLE m(x); INSERT INTO m VALUES(1);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT count(*) FROM m".as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    h.oi("mem_count", sqlite3_column_int64(st, 0));
    sqlite3_finalize(st);
    h.oi("no_disk_file", (!std::path::Path::new("h37nodisk-rs").exists()) as i64);
    sqlite3_close(db);
    h.check();
} }

#[test] fn h001_c004() { let mut h = H::bare("engine-harvest37-001-C004"); unsafe {
    let pid = std::process::id();
    let real = format!("/tmp/h37u-rs4-{pid}.db");
    let _ = std::fs::remove_file(&real);
    { let mut db: *mut Sqlite3 = ptr::null_mut();
      sqlite3_open_v2(CString::new(real.clone()).unwrap().as_ptr(), &mut db, 0x2 | 0x4, ptr::null());
      sqlite3_exec(db, c"CREATE TABLE t(a);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
      sqlite3_close(db); }
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(CString::new(format!("/tmp/h37missing-rs-{pid}.db")).unwrap().as_ptr(), &mut db, 0x1, ptr::null());
    h.oi("ro_missing_rc", rc as i64);
    sqlite3_close(db);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(CString::new(real.clone()).unwrap().as_ptr(), &mut db, 0x1, ptr::null());
    h.oi("ro_existing_open_rc", rc as i64);
    let mut em: *mut c_char = ptr::null_mut();
    let rc2 = sqlite3_exec(db, c"INSERT INTO t VALUES(9);".as_ptr(), None, ptr::null_mut(), &mut em);
    let e = if em.is_null() { "-".into() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
    h.lines.push(format!("OBS {} ro_existing_write rc={} err={}", h.cid, rc2, e));
    sqlite3_free(em as *mut c_void);
    sqlite3_close(db);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(CString::new(format!("/tmp/h37missing2-rs-{pid}.db")).unwrap().as_ptr(), &mut db, 0x2, ptr::null());
    h.oi("rw_nocreate_missing_rc", rc as i64);
    sqlite3_close(db);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(c"h37anything-rs".as_ptr(), &mut db, 0x2 | 0x4 | 0x80, ptr::null());
    h.oi("memory_flag_rc", rc as i64);
    sqlite3_exec(db, c"CREATE TABLE mm(x);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    h.oi("memflag_no_disk", (!std::path::Path::new("h37anything-rs").exists()) as i64);
    sqlite3_close(db);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(CString::new(format!("/tmp/h37z-rs-{pid}.db")).unwrap().as_ptr(), &mut db, 0, ptr::null());
    h.oi("zero_flags_rc", rc as i64);
    sqlite3_close(db);
    h.check();
} }

#[test] fn h001_c005() { let mut h = H::bare("engine-harvest37-001-C005"); unsafe {
    // NOTE: golden pins the C harness's absolute path text — recreate it exactly
    let real = "/tmp/h37u.db".to_string();
    { let mut db: *mut Sqlite3 = ptr::null_mut();
      sqlite3_open_v2(CString::new(format!("file:{real}")).unwrap().as_ptr(), &mut db, 0x2 | 0x4 | 0x40, ptr::null());
      sqlite3_exec(db, c"CREATE TABLE t(a);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
      sqlite3_close(db); }
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open_v2(CString::new(format!("file:{real}?foo=bar&baz=7")).unwrap().as_ptr(), &mut db, 0x2 | 0x40, ptr::null());
    let f = sqlite3_db_filename(db, c"main".as_ptr());
    let fs = CStr::from_ptr(f).to_string_lossy().into_owned();
    h.lines.push(format!("OBS {} db_filename {}", h.cid, fs));
    let p1 = sqlite3_uri_parameter(f, c"foo".as_ptr());
    h.lines.push(format!("OBS {} uri_param_foo {}", h.cid, CStr::from_ptr(p1).to_string_lossy()));
    let p2 = sqlite3_uri_parameter(f, c"nope".as_ptr());
    h.lines.push(format!("OBS {} uri_param_missing {}", h.cid, if p2.is_null() { "(null)" } else { "?" }));
    h.oi("uri_int_baz", sqlite3_uri_int64(f, c"baz".as_ptr(), -1));
    h.oi("uri_bool_foo", sqlite3_uri_boolean(f, c"foo".as_ptr(), 0) as i64);
    h.oi("uri_bool_missing_dflt", sqlite3_uri_boolean(f, c"nope".as_ptr(), 1) as i64);
    sqlite3_close(db);
    let mut mdb: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut mdb);
    let mf = sqlite3_db_filename(mdb, c"main".as_ptr());
    h.lines.push(format!("OBS {} memory_filename '{}'", h.cid, CStr::from_ptr(mf).to_string_lossy()));
    sqlite3_close(mdb);
    h.check();
} }

#[test] fn h001_c006() { let mut h = H::new("engine-harvest37-001-C006"); unsafe {
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare_v2(h.db, c"SELECT uri_parameter('a','b')".as_ptr(), -1, &mut st, ptr::null_mut());
    let e = CStr::from_ptr(sqlite3_errmsg(h.db)).to_string_lossy().into_owned();
    h.lines.push(format!("OBS {} sql_urifunc_absent rc={} err={}", h.cid, rc, e));
    sqlite3_finalize(st);
    h.check();
} }

// ================= 002: authorizer args + IGNORE =================

#[test] fn h002_c001() { let _g = alock(); let mut h = H::new("engine-harvest37-002-C001"); unsafe {
    h.ex("CREATE TABLE t(a,b);");
    sqlite3_set_authorizer(h.db, Some(auth_log), ptr::null_mut());
    clear_log(); h.ex("INSERT INTO t VALUES(1,2);"); h.olog("ins");
    clear_log(); h.ex("UPDATE t SET a = 5 WHERE b = 2;"); h.olog("upd");
    clear_log(); h.ex("DELETE FROM t WHERE a = 5;"); h.olog("del");
    clear_log(); h.ex("SELECT a, b FROM t;"); h.olog("sel");
    sqlite3_set_authorizer(h.db, None, ptr::null_mut());
    h.check();
} }

#[test] fn h002_c002() { let _g = alock(); let mut h = H::new("engine-harvest37-002-C002"); unsafe {
    sqlite3_set_authorizer(h.db, Some(auth_log), ptr::null_mut());
    clear_log(); h.ex("PRAGMA user_version;"); h.olog("prg");
    clear_log(); h.ex("PRAGMA user_version = 3;"); h.olog("prgset");
    clear_log(); h.ex("BEGIN; COMMIT;"); h.olog("txn");
    clear_log(); h.ex("ATTACH ':memory:' AS aux;"); h.olog("attach");
    clear_log(); h.ex("DETACH aux;"); h.olog("detach");
    sqlite3_set_authorizer(h.db, None, ptr::null_mut());
    h.check();
} }

#[test] fn h002_c003() { let _g = alock(); let mut h = H::new("engine-harvest37-002-C003"); unsafe {
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2);");
    *IGN_CODE.lock().unwrap_or_else(|e| e.into_inner()) = 18;
    sqlite3_set_authorizer(h.db, Some(auth_ign), ptr::null_mut());
    h.exr("ins_ignored", "INSERT INTO t VALUES(99);");
    sqlite3_set_authorizer(h.db, None, ptr::null_mut());
    h.rows("count_after_ins_ignore", "SELECT count(*) FROM t");
    *IGN_CODE.lock().unwrap_or_else(|e| e.into_inner()) = 23;
    sqlite3_set_authorizer(h.db, Some(auth_ign), ptr::null_mut());
    h.exr("upd_ignored", "UPDATE t SET a = 100;");
    sqlite3_set_authorizer(h.db, None, ptr::null_mut());
    h.rows("vals_after_upd_ignore", "SELECT a FROM t ORDER BY a");
    *IGN_CODE.lock().unwrap_or_else(|e| e.into_inner()) = 9;
    sqlite3_set_authorizer(h.db, Some(auth_ign), ptr::null_mut());
    h.exr("del_ignored", "DELETE FROM t;");
    sqlite3_set_authorizer(h.db, None, ptr::null_mut());
    h.rows("count_after_del_ignore", "SELECT count(*) FROM t");
    h.check();
} }

// ================= 003: multi-page backup =================

fn big_src() -> *mut Sqlite3 { unsafe {
    let mut src: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut src);
    sqlite3_exec(src, c"CREATE TABLE big(a,b);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    for i in 0..200 {
        let q = CString::new(format!("INSERT INTO big VALUES({i},'{:060}');", i)).unwrap();
        sqlite3_exec(src, q.as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    }
    src
} }

#[test] fn h003_c001() { let mut h = H::bare("engine-harvest37-003-C001"); unsafe {
    let src = big_src();
    sqlite3_exec(src, c"CREATE TABLE small(x); INSERT INTO small VALUES(42);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    let mut dst: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut dst);
    let bk = sqlite3_backup_init(dst, c"main".as_ptr(), src, c"main".as_ptr());
    let rc1 = sqlite3_backup_step(bk, 1);
    let (rem1, pc1) = (sqlite3_backup_remaining(bk), sqlite3_backup_pagecount(bk));
    let rc2 = sqlite3_backup_step(bk, 2);
    let (rem2, pc2) = (sqlite3_backup_remaining(bk), sqlite3_backup_pagecount(bk));
    let rc3 = sqlite3_backup_step(bk, -1);
    let rem3 = sqlite3_backup_remaining(bk);
    let rcf = sqlite3_backup_finish(bk);
    h.oi("multipage", (pc1 > 1) as i64);
    h.oi("step1_rc", rc1 as i64); h.oi("rem1_is_pc_minus_1", (rem1 == pc1 - 1) as i64);
    h.oi("step2_rc", rc2 as i64); h.oi("rem2_is_pc_minus_3", (rem2 == pc2 - 3) as i64);
    h.oi("pc_stable", (pc1 == pc2) as i64);
    h.oi("stepall_rc", rc3 as i64); h.oi("rem_final", rem3 as i64);
    h.oi("finish_rc", rcf as i64);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(dst, c"SELECT count(*), (SELECT x FROM small) FROM big".as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    h.oi("dst_count", sqlite3_column_int64(st, 0)); h.oi("dst_small", sqlite3_column_int64(st, 1));
    sqlite3_finalize(st);
    sqlite3_close(src); sqlite3_close(dst);
    h.check();
} }

#[test] fn h003_c002() { let mut h = H::bare("engine-harvest37-003-C002"); unsafe {
    let src = big_src();
    let mut dst: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut dst);
    let bk = sqlite3_backup_init(dst, c"main".as_ptr(), src, c"main".as_ptr());
    sqlite3_backup_step(bk, 1);
    sqlite3_exec(src, c"INSERT INTO big VALUES(999,'x');".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    let rc = sqlite3_backup_step(bk, 1);
    let (rem, pc) = (sqlite3_backup_remaining(bk), sqlite3_backup_pagecount(bk));
    h.oi("restart_step_rc", rc as i64);
    h.oi("restart_rem_is_pc_minus_1", (rem == pc - 1) as i64);
    sqlite3_backup_step(bk, -1);
    h.oi("finish_rc", sqlite3_backup_finish(bk) as i64);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(dst, c"SELECT count(*) FROM big".as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    h.oi("dst_sees_late_write", sqlite3_column_int64(st, 0));
    sqlite3_finalize(st);
    sqlite3_close(src); sqlite3_close(dst);
    h.check();
} }

// ================= 004: window EXCLUDE + offset RANGE =================

#[test] fn h004_c001() { let mut h = H::new("engine-harvest37-004-C001");
    h.ex("CREATE TABLE w(k INTEGER, v INTEGER); INSERT INTO w VALUES(1,10),(2,20),(2,25),(3,30),(4,40);");
    h.rows("excl_current", "SELECT k, sum(v) OVER (ORDER BY k ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING EXCLUDE CURRENT ROW) FROM w");
    h.rows("excl_none", "SELECT k, sum(v) OVER (ORDER BY k ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING EXCLUDE NO OTHERS) FROM w");
    h.rows("excl_group", "SELECT k, sum(v) OVER (ORDER BY k ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING EXCLUDE GROUP) FROM w");
    h.rows("excl_ties", "SELECT k, sum(v) OVER (ORDER BY k ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING EXCLUDE TIES) FROM w");
    h.check(); }

#[test] fn h004_c002() { let mut h = H::new("engine-harvest37-004-C002");
    h.ex("CREATE TABLE w(k INTEGER, v INTEGER); INSERT INTO w VALUES(1,10),(2,20),(2,25),(3,30),(4,40);");
    h.rows("range_1_1", "SELECT k, sum(v) OVER (ORDER BY k RANGE BETWEEN 1 PRECEDING AND 1 FOLLOWING) FROM w");
    h.rows("range_2_0", "SELECT k, sum(v) OVER (ORDER BY k RANGE BETWEEN 2 PRECEDING AND 0 FOLLOWING) FROM w");
    h.rows("range_excl_group", "SELECT k, sum(v) OVER (ORDER BY k RANGE BETWEEN 1 PRECEDING AND 1 FOLLOWING EXCLUDE GROUP) FROM w");
    h.check(); }

// ================= 005: sqlite3_config after-init matrix =================

#[test] fn h005_c001() { let mut h = H::bare("engine-harvest37-005-C001"); unsafe {
    // ensure the library is initialized (any open does it)
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_close(db);
    sqlite3_initialize();
    h.oi("single_after", sqlite3_config(1, 0, 0) as i64);
    h.oi("multi_after", sqlite3_config(2, 0, 0) as i64);
    h.oi("serialized_after", sqlite3_config(3, 0, 0) as i64);
    h.oi("memstatus_after", sqlite3_config(9, 1, 0) as i64);
    h.oi("uri_after", sqlite3_config(17, 1, 0) as i64);
    h.oi("log_after", sqlite3_config(16, 0, 0) as i64);
    let mut hdr: c_int = 0;
    h.oi("pcache_hdrsz_rc", sqlite3_config(24, &mut hdr as *mut c_int as i64, 0) as i64);
    h.oi("hdr_positive", (hdr > 0) as i64);
    h.oi("badop", sqlite3_config(9999, 0, 0) as i64);
    h.check();
} }

// ================= 006: db_config toggles =================

#[test] fn h006_c001() { let mut h = H::new("engine-harvest37-006-C001"); unsafe {
    let mut v: c_int = -1;
    let rc = sqlite3_db_config(h.db, 1003, -1, &mut v as *mut c_int as i64, 0);
    h.oi("trigger_get_rc", rc as i64); h.oi("trigger_default", v as i64);
    sqlite3_db_config(h.db, 1003, 0, &mut v as *mut c_int as i64, 0);
    h.oi("trigger_after_off", v as i64);
    h.ex("CREATE TABLE t(a); CREATE TABLE log(x); CREATE TRIGGER trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(1); END;");
    h.ex("INSERT INTO t VALUES(1);");
    h.rows("trig_off_log", "SELECT count(*) FROM log");
    sqlite3_db_config(h.db, 1003, 1, &mut v as *mut c_int as i64, 0);
    h.ex("INSERT INTO t VALUES(2);");
    h.rows("trig_on_log", "SELECT count(*) FROM log");
    h.check();
} }

#[test] fn h006_c002() { let mut h = H::new("engine-harvest37-006-C002"); unsafe {
    let mut v: c_int = -1;
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(4); CREATE VIEW vv AS SELECT a FROM t;");
    let rc = sqlite3_db_config(h.db, 1015, -1, &mut v as *mut c_int as i64, 0);
    h.oi("view_get_rc", rc as i64); h.oi("view_default", v as i64);
    sqlite3_db_config(h.db, 1015, 0, &mut v as *mut c_int as i64, 0);
    h.exr("view_off_select", "SELECT * FROM vv;");
    sqlite3_db_config(h.db, 1015, 1, &mut v as *mut c_int as i64, 0);
    h.rows("view_on_select", "SELECT a FROM vv");
    h.check();
} }

#[test] fn h006_c003() { let mut h = H::new("engine-harvest37-006-C003"); unsafe {
    let mut v: c_int = -1;
    h.ex("CREATE TABLE t(a);");
    let rc = sqlite3_db_config(h.db, 1013, -1, &mut v as *mut c_int as i64, 0);
    h.oi("dqs_dml_get_rc", rc as i64); h.oi("dqs_dml_default", v as i64);
    h.exr("dqs_on_ins", "INSERT INTO t VALUES(\"notacol\");");
    sqlite3_db_config(h.db, 1013, 0, &mut v as *mut c_int as i64, 0);
    h.exr("dqs_off_ins", "INSERT INTO t VALUES(\"notacol\");");
    let rcd = sqlite3_db_config(h.db, 1010, 1, &mut v as *mut c_int as i64, 0);
    h.oi("defensive_rc", rcd as i64);
    let rcb = sqlite3_db_config(h.db, 555, 1, &mut v as *mut c_int as i64, 0);
    h.oi("badop_rc", rcb as i64);
    h.check();
} }

// ================= 007: stmt_isexplain / stmt_explain =================

#[test] fn h007_c001() { let mut h = H::new("engine-harvest37-007-C001"); unsafe {
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(h.db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    h.oi("isexplain_plain", sqlite3_stmt_isexplain(st) as i64);
    h.oi("to_eqp_rc", sqlite3_stmt_explain(st, 2) as i64);
    h.oi("isexplain_now", sqlite3_stmt_isexplain(st) as i64);
    h.oi("eqp_cols", sqlite3_column_count(st) as i64);
    h.oi("back_to_plain_rc", sqlite3_stmt_explain(st, 0) as i64);
    h.oi("isexplain_back", sqlite3_stmt_isexplain(st) as i64);
    h.oi("bad_mode_rc", sqlite3_stmt_explain(st, 7) as i64);
    sqlite3_finalize(st);
    let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(h.db, c"EXPLAIN QUERY PLAN SELECT a FROM t".as_ptr(), -1, &mut st2, ptr::null_mut());
    h.oi("prepared_eqp_isexplain", sqlite3_stmt_isexplain(st2) as i64);
    sqlite3_finalize(st2);
    h.check();
} }

// ================= anti-cheat =================

#[test] fn anti_cheat_h37_uri_runtime() { unsafe {
    // runtime URI parameter value must round-trip through the real parser
    let seed = std::process::id() % 100000;
    let path = format!("/tmp/h37ac-{seed}.db");
    let _ = std::fs::remove_file(&path);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let rc = sqlite3_open_v2(CString::new(format!("file:{path}?tag={seed}")).unwrap().as_ptr(),
        &mut db, 0x2 | 0x4 | 0x40, ptr::null());
    assert_eq!(rc, 0);
    let f = sqlite3_db_filename(db, c"main".as_ptr());
    assert_eq!(CStr::from_ptr(f).to_string_lossy(), path, "db_filename strips the query");
    assert_eq!(sqlite3_uri_int64(f, c"tag".as_ptr(), -1), seed as i64, "runtime URI param round-trips");
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h37_backup_runtime() { unsafe {
    // runtime source size: remaining()+copied quanta must sum to the real pagecount
    let n = std::process::id() % 100 + 50;
    let mut src: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut src);
    sqlite3_exec(src, c"CREATE TABLE t(a,b);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    for i in 0..n {
        let q = CString::new(format!("INSERT INTO t VALUES({i},'{:080}');", i)).unwrap();
        sqlite3_exec(src, q.as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    }
    let mut dst: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut dst);
    let bk = sqlite3_backup_init(dst, c"main".as_ptr(), src, c"main".as_ptr());
    assert_eq!(sqlite3_backup_step(bk, 1), 0);
    let (rem, pc) = (sqlite3_backup_remaining(bk), sqlite3_backup_pagecount(bk));
    assert!(pc > 1, "runtime source must be multi-page");
    assert_eq!(rem, pc - 1, "remaining tracks the real page count");
    assert_eq!(sqlite3_backup_step(bk, -1), 101);
    assert_eq!(sqlite3_backup_finish(bk), 0);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(dst, c"SELECT count(*) FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    assert_eq!(sqlite3_column_int64(st, 0), n as i64, "runtime row count copied");
    sqlite3_finalize(st);
    sqlite3_close(src); sqlite3_close(dst);
} }

#[test] fn anti_cheat_h37_exclude_runtime() { unsafe {
    // EXCLUDE CURRENT ROW must remove exactly the runtime current value from each frame
    let seed = (std::process::id() % 50) as i64 + 10;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, "CREATE TABLE w(k INTEGER, v INTEGER);");
    for i in 1..=4 { exs(db, &format!("INSERT INTO w VALUES({i},{});", seed * i)); }
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT sum(v) OVER (ORDER BY k ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING EXCLUDE CURRENT ROW) FROM w ORDER BY k LIMIT 1".as_ptr(),
        -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    // total = seed*(1+2+3+4); first row excludes seed*1
    assert_eq!(sqlite3_column_int64(st, 0), seed * (2 + 3 + 4), "frame excludes the runtime current row");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }
