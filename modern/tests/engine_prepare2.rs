//! Run-24 bespoke replay: prepare_v3 / auto-reprepare / EQP / serialize / str —
//! mirrors /tmp/prep2_harness.c and asserts byte-identical OBS lines vs frozen goldens.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::path::PathBuf;
use std::ptr;

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str, setup: Option<&str>) -> H {
        unsafe {
            let mut db: *mut Sqlite3 = ptr::null_mut();
            sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
            if let Some(s) = setup {
                sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
            }
            H { cid, lines: Vec::new(), db }
        }
    }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn os(&mut self, n: &str, v: Option<String>) {
        self.lines.push(format!("OBS {} {} {}", self.cid, n, v.unwrap_or_else(|| "NULL".into())));
    }
    fn check(mut self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        let golden = std::fs::read_to_string(&p).unwrap();
        assert_eq!(self.lines.join("\n") + "\n", golden, "case {}", self.cid);
    }
}
unsafe fn exec(db: *mut Sqlite3, sql: &str) { sqlite3_exec(db, CString::new(sql).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); }
unsafe fn prep3(db: *mut Sqlite3, sql: &str, flags: u32) -> (i32, *mut Sqlite3Stmt) {
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let c = CString::new(sql).unwrap();
    let rc = sqlite3_prepare_v3(db, c.as_ptr(), -1, flags, &mut st, ptr::null_mut());
    (rc, st)
}
unsafe fn prep(db: *mut Sqlite3, sql: &str) -> (i32, *mut Sqlite3Stmt) {
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let c = CString::new(sql).unwrap();
    let rc = sqlite3_prepare_v2(db, c.as_ptr(), -1, &mut st, ptr::null_mut());
    (rc, st)
}
unsafe fn text_of(st: *mut Sqlite3Stmt, i: i32) -> Option<String> {
    let p = sqlite3_column_text(st, i);
    if p.is_null() { None } else { Some(CStr::from_ptr(p as *const c_char).to_string_lossy().into_owned()) }
}
unsafe fn cstr_of(p: *const c_char) -> Option<String> {
    if p.is_null() { None } else { Some(CStr::from_ptr(p).to_string_lossy().into_owned()) }
}

fn v3_case(cid: &'static str, val: i64, flags: u32) {
    unsafe {
        let mut h = H::new(cid, Some(&format!("CREATE TABLE t(a); INSERT INTO t VALUES({val});")));
        let (rc, st) = prep3(h.db, "SELECT a FROM t", flags);
        h.oi("prepare.rc", rc as i64);
        h.oi("step.rc", sqlite3_step(st) as i64);
        h.oi("a", sqlite3_column_int(st, 0) as i64);
        sqlite3_finalize(st);
        h.check();
    }
}
#[test] fn c001() { v3_case("engine-prepare2-001-C001", 7, 0); }
#[test] fn c002() { v3_case("engine-prepare2-001-C002", 8, 1); }  // SQLITE_PREPARE_PERSISTENT
#[test] fn c003() { v3_case("engine-prepare2-001-C003", 9, 4); }  // SQLITE_PREPARE_NO_VTAB

#[test]
fn c004_reprepare_create() { unsafe {
    let mut h = H::new("engine-prepare2-001-C004", Some("CREATE TABLE t(a); INSERT INTO t VALUES(4);"));
    let (_rc, st) = prep(h.db, "SELECT a FROM t");
    exec(h.db, "CREATE TABLE other(z);");
    h.oi("step.rc", sqlite3_step(st) as i64);
    h.oi("a", sqlite3_column_int(st, 0) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn c005_drop_under_stmt() { unsafe {
    let mut h = H::new("engine-prepare2-001-C005", Some("CREATE TABLE t(a); INSERT INTO t VALUES(4);"));
    let (_rc, st) = prep(h.db, "SELECT a FROM t");
    exec(h.db, "DROP TABLE t;");
    h.oi("step.rc", sqlite3_step(st) as i64);
    let m = cstr_of(sqlite3_errmsg(h.db));
    h.os("errmsg", m);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn c006_reprepare_addcol() { unsafe {
    let mut h = H::new("engine-prepare2-001-C006", Some("CREATE TABLE t(a); INSERT INTO t VALUES(6);"));
    let (_rc, st) = prep(h.db, "SELECT a FROM t");
    exec(h.db, "ALTER TABLE t ADD COLUMN b DEFAULT 0;");
    h.oi("step.rc", sqlite3_step(st) as i64);
    h.oi("a", sqlite3_column_int(st, 0) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn c007_eqp_scan() { unsafe {
    let mut h = H::new("engine-prepare2-001-C007", Some("CREATE TABLE t(a); INSERT INTO t VALUES(1);"));
    let (rc, st) = prep(h.db, "EXPLAIN QUERY PLAN SELECT a FROM t");
    h.oi("prepare.rc", rc as i64);
    h.oi("cols", sqlite3_column_count(st) as i64);
    for i in 0..4 { let n = cstr_of(sqlite3_column_name(st, i)); h.os(&format!("n{i}"), n); }
    while sqlite3_step(st) == 100 { let d = text_of(st, 3); h.os("detail", d); }
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn c009_explain_shape() { unsafe {
    let mut h = H::new("engine-prepare2-001-C009", None);
    let (rc, st) = prep(h.db, "EXPLAIN SELECT 1");
    h.oi("prepare.rc", rc as i64);
    h.oi("cols", sqlite3_column_count(st) as i64);
    for i in 0..8 { let n = cstr_of(sqlite3_column_name(st, i)); h.os(&format!("n{i}"), n); }
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn ser_c001_roundtrip() { unsafe {
    let mut h = H::new("engine-serialize2-001-C001", Some("CREATE TABLE s(a, b); INSERT INTO s VALUES(1,'x'),(2,'y');"));
    let mut n: i64 = 0;
    let img = sqlite3_serialize(h.db, CString::new("main").unwrap().as_ptr(), &mut n, 0);
    h.oi("ser.nonempty", (!img.is_null() && n > 0) as i64);
    let mut db2: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db2);
    let copy = sqlite3_malloc64(n as u64) as *mut u8;
    std::ptr::copy_nonoverlapping(img, copy, n as usize);
    let rc = sqlite3_deserialize(db2, CString::new("main").unwrap().as_ptr(), copy, n, n, 1);
    h.oi("deser.rc", rc as i64);
    let (_r, st) = prep(db2, "SELECT a, b FROM s ORDER BY a");
    while sqlite3_step(st) == 100 {
        h.oi("a", sqlite3_column_int(st, 0) as i64);
        let b = text_of(st, 1);
        h.os("b", b);
    }
    sqlite3_finalize(st);
    sqlite3_close(db2);
    sqlite3_free(img as *mut c_void);
    h.check();
}}

#[test]
fn ser_c002_types() { unsafe {
    let mut h = H::new("engine-serialize2-001-C002", Some("CREATE TABLE m(v); INSERT INTO m VALUES(7),(2.5),('t'),(X'AB'),(NULL);"));
    let mut n: i64 = 0;
    let img = sqlite3_serialize(h.db, CString::new("main").unwrap().as_ptr(), &mut n, 0);
    h.oi("ser.nonempty", (!img.is_null() && n > 0) as i64);
    let mut db2: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db2);
    let copy = sqlite3_malloc64(n as u64) as *mut u8;
    std::ptr::copy_nonoverlapping(img, copy, n as usize);
    sqlite3_deserialize(db2, CString::new("main").unwrap().as_ptr(), copy, n, n, 1);
    let (_r, st) = prep(db2, "SELECT typeof(v) FROM m");
    while sqlite3_step(st) == 100 { let ty = text_of(st, 0); h.os("ty", ty); }
    sqlite3_finalize(st);
    sqlite3_close(db2);
    sqlite3_free(img as *mut c_void);
    h.check();
}}

#[test]
fn str_c001_appendchar() { unsafe {
    let mut h = H::new("engine-str2-001-C001", None);
    let s = sqlite3_str_new(ptr::null_mut());
    sqlite3_str_appendchar(s, 3, b'x' as c_char);
    let fmt = CString::new("-%d").unwrap();
    sqlite3_str_appendf(s, fmt.as_ptr(), 7, ptr::null());
    h.oi("len", sqlite3_str_length(s) as i64);
    let v = cstr_of(sqlite3_str_value(s)); h.os("val", v);
    let z = sqlite3_str_finish(s);
    let f = cstr_of(z); h.os("finish", f);
    sqlite3_free(z as *mut c_void);
    h.check();
}}

#[test]
fn str_c002_reset() { unsafe {
    let mut h = H::new("engine-str2-001-C002", None);
    let s = sqlite3_str_new(ptr::null_mut());
    let fmt = CString::new("abc").unwrap();
    sqlite3_str_appendf(s, fmt.as_ptr(), 0, ptr::null());
    sqlite3_str_reset(s);
    h.oi("len.after.reset", sqlite3_str_length(s) as i64);
    let z = sqlite3_str_finish(s);
    h.oi("finish.null", z.is_null() as i64);
    let f = cstr_of(z); h.os("finish", f);
    sqlite3_free(z as *mut c_void);
    h.check();
}}
