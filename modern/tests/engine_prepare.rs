//! Run-23 prepare/bind/column replay — mirrors /tmp/prep_harness.c call-for-call and
//! asserts byte-identical OBS lines against the frozen C goldens. The statement API
//! executes through the SAME store/eval engine as sqlite3_exec (pack v13 law).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

const TRANSIENT: *mut c_void = usize::MAX as *mut c_void; // SQLITE_TRANSIENT (-1)

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
    fn od(&mut self, n: &str, v: f64) { self.lines.push(format!("OBS {} {} {:.6}", self.cid, n, fmt_g6(v))); }
    fn check(mut self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop();
        p.push(format!("tests/characterization/engine-prepare/cases/{feat}/{cnum}.approved.txt"));
        let golden = std::fs::read_to_string(&p).unwrap();
        assert_eq!(self.lines.join("\n") + "\n", golden, "case {}", self.cid);
    }
}
// C printed doubles with %.6g
fn fmt_g6(v: f64) -> String { let s = format!("{}", v); s }
impl H { fn odg(&mut self, n: &str, v: f64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, fmt_g6(v))); } }

unsafe fn prep(db: *mut Sqlite3, sql: &str) -> (i32, *mut Sqlite3Stmt) {
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let c = CString::new(sql).unwrap();
    let rc = sqlite3_prepare_v2(db, c.as_ptr(), -1, &mut st, ptr::null_mut());
    (rc, st)
}
unsafe fn text_of(st: *mut Sqlite3Stmt, i: c_int) -> Option<String> {
    let p = sqlite3_column_text(st, i);
    if p.is_null() { None } else { Some(CStr::from_ptr(p as *const c_char).to_string_lossy().into_owned()) }
}
unsafe fn cstr_of(p: *const c_char) -> Option<String> {
    if p.is_null() { None } else { Some(CStr::from_ptr(p).to_string_lossy().into_owned()) }
}

#[test]
fn engine_prepare_001_c001() { unsafe {
    let mut h = H::new("engine-prepare-001-C001", Some("CREATE TABLE t(a,b); INSERT INTO t VALUES(1,'x'),(2,'y');"));
    let (rc, st) = prep(h.db, "SELECT a, b FROM t ORDER BY a"); h.oi("prepare.rc", rc as i64);
    h.oi("step1.rc", sqlite3_step(st) as i64); h.oi("a1", sqlite3_column_int(st,0) as i64); let b1 = text_of(st,1); h.os("b1", b1);
    h.oi("step2.rc", sqlite3_step(st) as i64); h.oi("a2", sqlite3_column_int(st,0) as i64); let b2 = text_of(st,1); h.os("b2", b2);
    h.oi("step3.rc", sqlite3_step(st) as i64); h.oi("finalize.rc", sqlite3_finalize(st) as i64);
    h.check();
}}

#[test]
fn engine_prepare_001_c002() { unsafe {
    let mut h = H::new("engine-prepare-001-C002", Some("CREATE TABLE t2(v);"));
    let (rc, st) = prep(h.db, "INSERT INTO t2 VALUES(5)"); h.oi("prepare.rc", rc as i64);
    h.oi("step.rc", sqlite3_step(st) as i64); h.oi("finalize.rc", sqlite3_finalize(st) as i64);
    let (rc2, st2) = prep(h.db, "SELECT count(*), v FROM t2"); h.oi("prepare2.rc", rc2 as i64);
    h.oi("step2.rc", sqlite3_step(st2) as i64);
    h.oi("count", sqlite3_column_int(st2,0) as i64); h.oi("v", sqlite3_column_int(st2,1) as i64);
    sqlite3_finalize(st2);
    h.check();
}}

#[test]
fn engine_prepare_001_c003() { unsafe {
    let mut h = H::new("engine-prepare-001-C003", None);
    let sql = CString::new("SELECT 1; SELECT 2").unwrap();
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let mut tail: *const c_char = ptr::null();
    let rc = sqlite3_prepare_v2(h.db, sql.as_ptr(), -1, &mut st, &mut tail);
    h.oi("prepare.rc", rc as i64);
    let tail_s = cstr_of(tail); h.os("tail", tail_s.clone());
    h.oi("step.rc", sqlite3_step(st) as i64); h.oi("v1", sqlite3_column_int(st,0) as i64); sqlite3_finalize(st);
    let t2 = CString::new(tail_s.unwrap_or_default()).unwrap();
    let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
    let rc2 = sqlite3_prepare_v2(h.db, t2.as_ptr(), -1, &mut st2, &mut tail);
    h.oi("prepare2.rc", rc2 as i64);
    h.oi("step2.rc", sqlite3_step(st2) as i64); h.oi("v2", sqlite3_column_int(st2,0) as i64); sqlite3_finalize(st2);
    h.check();
}}

#[test]
fn engine_prepare_001_c004() { unsafe {
    let mut h = H::new("engine-prepare-001-C004", None);
    let (rc, _st) = prep(h.db, "SELECT * FROM nosuch"); h.oi("prepare.rc", rc as i64);
    h.oi("errcode", sqlite3_errcode(h.db) as i64);
    let m = cstr_of(sqlite3_errmsg(h.db)); h.os("errmsg", m);
    h.check();
}}

#[test]
fn engine_prepare_001_c005() { unsafe {
    let mut h = H::new("engine-prepare-001-C005", None);
    let (rc, _st) = prep(h.db, "SELECT frob(1)"); h.oi("prepare.rc", rc as i64);
    h.oi("errcode", sqlite3_errcode(h.db) as i64);
    let m = cstr_of(sqlite3_errmsg(h.db)); h.os("errmsg", m);
    h.check();
}}

#[test]
fn engine_prepare_001_c006() { unsafe {
    let mut h = H::new("engine-prepare-001-C006", Some("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2);"));
    let (_rc, st) = prep(h.db, "SELECT a FROM t ORDER BY a");
    h.oi("step1.rc", sqlite3_step(st) as i64); h.oi("a1", sqlite3_column_int(st,0) as i64);
    h.oi("reset.rc", sqlite3_reset(st) as i64);
    h.oi("step2.rc", sqlite3_step(st) as i64); h.oi("a2", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_001_c007() { unsafe {
    let mut h = H::new("engine-prepare-001-C007", Some("CREATE TABLE t(a); INSERT INTO t VALUES(9);"));
    let (_rc, st) = prep(h.db, "SELECT a FROM t");
    h.oi("step1.rc", sqlite3_step(st) as i64);
    h.oi("step2.rc", sqlite3_step(st) as i64);
    h.oi("step3.rc", sqlite3_step(st) as i64);
    h.oi("a", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_001_c008() { unsafe {
    let mut h = H::new("engine-prepare-001-C008", Some("CREATE TABLE t(a); INSERT INTO t VALUES(1);"));
    let (_rc, st) = prep(h.db, "SELECT a FROM t");
    h.oi("ro.select", sqlite3_stmt_readonly(st) as i64); h.oi("busy.before", sqlite3_stmt_busy(st) as i64);
    sqlite3_step(st); h.oi("busy.row", sqlite3_stmt_busy(st) as i64);
    sqlite3_step(st); h.oi("busy.done", sqlite3_stmt_busy(st) as i64);
    sqlite3_finalize(st);
    let (_r2, st2) = prep(h.db, "INSERT INTO t VALUES(2)");
    h.oi("ro.insert", sqlite3_stmt_readonly(st2) as i64);
    sqlite3_finalize(st2);
    h.check();
}}

#[test]
fn engine_prepare_002_c001() { unsafe {
    let mut h = H::new("engine-prepare-002-C001", Some("CREATE TABLE b1(c1,c2,c3,c4,c5,c6);"));
    let (_rc, st) = prep(h.db, "INSERT INTO b1 VALUES(?1,?2,?3,?4,?5,?6)");
    h.oi("param_count", sqlite3_bind_parameter_count(st) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_002_c002() { unsafe {
    let mut h = H::new("engine-prepare-002-C002", Some("CREATE TABLE b1(c1,c2,c3,c4,c5,c6);"));
    let (_rc, st) = prep(h.db, "INSERT INTO b1 VALUES(?1,?2,?3,?4,?5,?6)");
    h.oi("b.null", sqlite3_bind_null(st,1) as i64);
    h.oi("b.int", sqlite3_bind_int(st,2,42) as i64);
    h.oi("b.int64", sqlite3_bind_int64(st,3,9_000_000_000) as i64);
    h.oi("b.double", sqlite3_bind_double(st,4,2.5) as i64);
    let hi = CString::new("hi").unwrap();
    h.oi("b.text", sqlite3_bind_text(st,5,hi.as_ptr(),-1,TRANSIENT) as i64);
    let bl = [1u8,2u8];
    h.oi("b.blob", sqlite3_bind_blob(st,6,bl.as_ptr() as *const c_void,2,TRANSIENT) as i64);
    h.oi("step.rc", sqlite3_step(st) as i64); sqlite3_finalize(st);
    let (_r2, st2) = prep(h.db, "SELECT typeof(c1),typeof(c2),typeof(c3),typeof(c4),typeof(c5),typeof(c6) FROM b1");
    sqlite3_step(st2);
    for i in 0..6 { let v = text_of(st2, i); h.os(&format!("ty{i}"), v); }
    sqlite3_finalize(st2);
    h.check();
}}

#[test]
fn engine_prepare_002_c003() { unsafe {
    let mut h = H::new("engine-prepare-002-C003", None);
    let (_rc, st) = prep(h.db, "SELECT ?1");
    h.oi("idx7.rc", sqlite3_bind_int(st,7,1) as i64);
    h.oi("idx0.rc", sqlite3_bind_int(st,0,1) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_002_c004() { unsafe {
    let mut h = H::new("engine-prepare-002-C004", None);
    let (_rc, st) = prep(h.db, "SELECT ?1 + ?2");
    sqlite3_bind_int(st,1,3); sqlite3_bind_int(st,2,4);
    sqlite3_step(st); h.oi("sum1", sqlite3_column_int(st,0) as i64);
    sqlite3_reset(st);
    sqlite3_bind_int(st,1,10); sqlite3_bind_int(st,2,20);
    sqlite3_step(st); h.oi("sum2", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_002_c005() { unsafe {
    let mut h = H::new("engine-prepare-002-C005", None);
    let (_rc, st) = prep(h.db, "SELECT ?1");
    let mut buf = *b"alpha\0          ";
    sqlite3_bind_text(st,1,buf.as_ptr() as *const c_char,-1,TRANSIENT);
    buf[..12].copy_from_slice(b"OVERWRITTEN\0");
    sqlite3_step(st);
    let v = text_of(st,0); h.os("text", v);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_002_c006() { unsafe {
    let mut h = H::new("engine-prepare-002-C006", None);
    let (_rc, st) = prep(h.db, "SELECT ?1");
    sqlite3_step(st);
    h.oi("type", sqlite3_column_type(st,0) as i64); h.oi("int", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_002_c007() { unsafe {
    let mut h = H::new("engine-prepare-002-C007", None);
    let (_rc, st) = prep(h.db, "SELECT :k + ?2");
    h.oi("count", sqlite3_bind_parameter_count(st) as i64);
    let n1 = cstr_of(sqlite3_bind_parameter_name(st,1)); h.os("name1", n1);
    let n2 = cstr_of(sqlite3_bind_parameter_name(st,2)); h.os("name2", n2);
    let k = CString::new(":k").unwrap();
    h.oi("idx.k", sqlite3_bind_parameter_index(st, k.as_ptr()) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_002_c008() { unsafe {
    let mut h = H::new("engine-prepare-002-C008", Some("CREATE TABLE b8(x);"));
    let (_rc, st) = prep(h.db, "INSERT INTO b8 VALUES(?1)");
    sqlite3_bind_double(st,1,2.5); sqlite3_step(st); sqlite3_finalize(st);
    let (_r2, st2) = prep(h.db, "SELECT typeof(x), x FROM b8");
    sqlite3_step(st2);
    let ty = text_of(st2,0); h.os("typeof", ty);
    h.odg("x", sqlite3_column_double(st2,1));
    sqlite3_finalize(st2);
    h.check();
}}

#[test]
fn engine_prepare_002_c009() { unsafe {
    let mut h = H::new("engine-prepare-002-C009", Some("CREATE TABLE b9(x);"));
    let (_rc, st) = prep(h.db, "INSERT INTO b9 VALUES(?1)");
    sqlite3_bind_int(st,1,7); sqlite3_finalize(st);
    let (_r2, st2) = prep(h.db, "SELECT count(*) FROM b9");
    sqlite3_step(st2); h.oi("count", sqlite3_column_int(st2,0) as i64);
    sqlite3_finalize(st2);
    h.check();
}}

#[test]
fn engine_prepare_002_c010() { unsafe {
    let mut h = H::new("engine-prepare-002-C010", None);
    let (_rc, st) = prep(h.db, "SELECT ?1");
    sqlite3_bind_int64(st,1,9_000_000_000);
    sqlite3_step(st);
    h.oi("v", sqlite3_column_int64(st,0));
    h.oi("type", sqlite3_column_type(st,0) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_003_c001() { unsafe {
    let mut h = H::new("engine-prepare-003-C001", None);
    let (_rc, st) = prep(h.db, "SELECT NULL, 7, 2.5, 'txt', X'414243'");
    h.oi("count", sqlite3_column_count(st) as i64);
    for i in 0..5 { let n = cstr_of(sqlite3_column_name(st,i)); h.os(&format!("n{i}"), n); }
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_003_c002() { unsafe {
    let mut h = H::new("engine-prepare-003-C002", None);
    let (_rc, st) = prep(h.db, "SELECT NULL, 7, 2.5, 'txt', X'414243'");
    sqlite3_step(st);
    for i in 0..5 { h.oi(&format!("t{i}"), sqlite3_column_type(st,i) as i64); }
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_003_c003() { unsafe {
    let mut h = H::new("engine-prepare-003-C003", None);
    let (_rc, st) = prep(h.db, "SELECT NULL, 7, 2.5, 'txt', '42abc'");
    sqlite3_step(st);
    for i in 0..5 { h.oi(&format!("i{i}"), sqlite3_column_int(st,i) as i64); }
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_003_c004() { unsafe {
    let mut h = H::new("engine-prepare-003-C004", None);
    let (_rc, st) = prep(h.db, "SELECT 7, 2.5, '3.25xyz'");
    sqlite3_step(st);
    h.oi("i64", sqlite3_column_int64(st,0));
    h.odg("d0", sqlite3_column_double(st,0));
    h.odg("d1", sqlite3_column_double(st,1));
    h.odg("d2", sqlite3_column_double(st,2));
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_003_c005() { unsafe {
    let mut h = H::new("engine-prepare-003-C005", None);
    let (_rc, st) = prep(h.db, "SELECT NULL, 7, 2.5, 'txt', X'414243'");
    sqlite3_step(st);
    for i in 0..5 { let v = text_of(st,i); h.os(&format!("s{i}"), v); }
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_003_c006() { unsafe {
    let mut h = H::new("engine-prepare-003-C006", None);
    let (_rc, st) = prep(h.db, "SELECT 'txt', X'414243', 7, NULL");
    sqlite3_step(st);
    for (i, n) in ["b0","b1","b2","b3"].iter().enumerate() { h.oi(n, sqlite3_column_bytes(st, i as c_int) as i64); }
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_003_c007() { unsafe {
    let mut h = H::new("engine-prepare-003-C007", None);
    let (_rc, st) = prep(h.db, "SELECT 5");
    h.oi("type.before", sqlite3_column_type(st,0) as i64); h.oi("int.before", sqlite3_column_int(st,0) as i64);
    sqlite3_step(st); sqlite3_step(st);
    h.oi("type.done", sqlite3_column_type(st,0) as i64); h.oi("int.done", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st);
    h.check();
}}

#[test]
fn engine_prepare_003_c008() { unsafe {
    let mut h = H::new("engine-prepare-003-C008", None);
    let (_rc, st) = prep(h.db, "SELECT 5");
    sqlite3_step(st);
    let n = cstr_of(sqlite3_column_name(st,99)); h.os("name99", n);
    h.oi("type99", sqlite3_column_type(st,99) as i64);
    h.oi("int99", sqlite3_column_int(st,99) as i64);
    sqlite3_finalize(st);
    h.check();
}}
