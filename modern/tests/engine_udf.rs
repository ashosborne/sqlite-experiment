//! Run-28 UDF / value / result replay — Rust registers the SAME callbacks as
//! /tmp/udf_harness.c and asserts byte-identical OBS lines vs the frozen C goldens.
//! The engine invokes these C-ABI callbacks for real (no script_table).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicI64, Ordering};

const TRANSIENT: *mut c_void = usize::MAX as *mut c_void;

// ---- callbacks (mirror the C harness) ----
unsafe extern "C" fn fn_twice(c: *mut Sqlite3Context, _n: c_int, a: *mut *mut Sqlite3Value) {
    sqlite3_result_int(c, 2 * sqlite3_value_int(*a));
}
unsafe extern "C" fn fn_addu(c: *mut Sqlite3Context, _n: c_int, a: *mut *mut Sqlite3Value) {
    let base = sqlite3_user_data(c) as i64;
    sqlite3_result_int(c, (base + sqlite3_value_int(*a) as i64) as c_int);
}
unsafe extern "C" fn fn_echo(c: *mut Sqlite3Context, _n: c_int, a: *mut *mut Sqlite3Value) {
    sqlite3_result_text(c, sqlite3_value_text(*a) as *const c_char, -1, TRANSIENT);
}
unsafe extern "C" fn fn_finger(c: *mut Sqlite3Context, _n: c_int, a: *mut *mut Sqlite3Value) {
    let t = sqlite3_value_type(*a);
    let tn = match t { 1 => "int", 2 => "float", 3 => "text", 4 => "blob", _ => "null" };
    let s = format!("{}/nt{}/by{}", tn, sqlite3_value_numeric_type(*a), sqlite3_value_bytes(*a));
    let cs = CString::new(s).unwrap();
    sqlite3_result_text(c, cs.as_ptr(), -1, TRANSIENT);
}
unsafe extern "C" fn fn_err(c: *mut Sqlite3Context, _n: c_int, _a: *mut *mut Sqlite3Value) {
    let m = CString::new("boom").unwrap();
    sqlite3_result_error(c, m.as_ptr(), -1);
}
unsafe extern "C" fn fn_types_out(c: *mut Sqlite3Context, _n: c_int, a: *mut *mut Sqlite3Value) {
    match sqlite3_value_int(*a) {
        0 => sqlite3_result_null(c),
        1 => sqlite3_result_int(c, 7),
        2 => sqlite3_result_int64(c, 9_000_000_000),
        3 => sqlite3_result_double(c, 2.5),
        4 => { let s = CString::new("hi").unwrap(); sqlite3_result_text(c, s.as_ptr(), -1, TRANSIENT); }
        5 => { let b = [1u8, 2, 3]; sqlite3_result_blob(c, b.as_ptr() as *const c_void, 3, TRANSIENT); }
        _ => {}
    }
}
unsafe extern "C" fn fn_vi64(c: *mut Sqlite3Context, _n: c_int, a: *mut *mut Sqlite3Value) {
    sqlite3_result_int64(c, sqlite3_value_int64(*a) + 1);
}
unsafe extern "C" fn fn_argc(c: *mut Sqlite3Context, n: c_int, _a: *mut *mut Sqlite3Value) {
    sqlite3_result_int(c, n);
}
unsafe extern "C" fn agg_step(c: *mut Sqlite3Context, _n: c_int, a: *mut *mut Sqlite3Value) {
    let p = sqlite3_aggregate_context(c, 8) as *mut i64;
    if !p.is_null() && sqlite3_value_type(*a) != 5 { *p += sqlite3_value_int64(*a); }
}
unsafe extern "C" fn agg_final(c: *mut Sqlite3Context) {
    let p = sqlite3_aggregate_context(c, 8) as *mut i64;
    if !p.is_null() { sqlite3_result_int64(c, *p); } else { sqlite3_result_null(c); }
}
static DESTROY: AtomicI64 = AtomicI64::new(0);
unsafe extern "C" fn x_destroy(_p: *mut c_void) { DESTROY.fetch_add(1, Ordering::SeqCst); }

// ---- harness plumbing ----
struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, s: &str) -> i32 { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()) } }
    fn q1(&mut self, s: &str) -> i64 { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut(); let c = CString::new(s).unwrap(); let mut v = -999i64;
        if sqlite3_prepare_v2(self.db, c.as_ptr(), -1, &mut st, ptr::null_mut()) == 0 { if sqlite3_step(st) == 100 { v = sqlite3_column_int64(st, 0); } }
        sqlite3_finalize(st); v } }
    fn q1int(&mut self, s: &str) -> i64 { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut(); let c = CString::new(s).unwrap(); let mut v = -999i64;
        if sqlite3_prepare_v2(self.db, c.as_ptr(), -1, &mut st, ptr::null_mut()) == 0 { if sqlite3_step(st) == 100 { v = sqlite3_column_int(st, 0) as i64; } }
        sqlite3_finalize(st); v } }
    fn q1s(&mut self, s: &str, label: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut(); let c = CString::new(s).unwrap();
        let mut val = "(none)".to_string(); let mut prep_ok = true;
        if sqlite3_prepare_v2(self.db, c.as_ptr(), -1, &mut st, ptr::null_mut()) == 0 {
            if sqlite3_step(st) == 100 { let p = sqlite3_column_text(st, 0);
                val = if p.is_null() { "NULL".into() } else { CStr::from_ptr(p as *const c_char).to_string_lossy().into_owned() }; }
        } else { prep_ok = false; }
        sqlite3_finalize(st);
        if !prep_ok { val = "(prep-fail)".into(); }
        self.os(label, &val); } }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn os(&mut self, n: &str, v: &str) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn errmsg(&self) -> String { unsafe { CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy().into_owned() } }
    fn check(mut self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
type SFn = unsafe extern "C" fn(*mut Sqlite3Context, c_int, *mut *mut Sqlite3Value);
unsafe fn reg(db: *mut Sqlite3, nm: &str, n: c_int, f: SFn) -> c_int {
    sqlite3_create_function(db, CString::new(nm).unwrap().as_ptr(), n, 1, ptr::null_mut(), Some(f), None, None)
}

#[test] fn c001() { unsafe { let mut h = H::new("engine-udf-001-C001"); reg(h.db,"twice",1,fn_twice);
    let v = h.q1("SELECT twice(21)"); h.oi("v", v); h.check(); } }

#[test] fn c002() { unsafe { let mut h = H::new("engine-udf-001-C002"); reg(h.db,"twice",1,fn_twice);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare_v2(h.db, CString::new("SELECT twice(1,2)").unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    h.oi("prep.rc", rc as i64); let e = h.errmsg(); h.os("errmsg", &e); sqlite3_finalize(st); h.check(); } }

#[test] fn c003() { unsafe { let mut h = H::new("engine-udf-001-C003"); reg(h.db,"f",1,fn_twice);
    let v = h.q1("SELECT f(10)"); h.oi("first", v); reg(h.db,"f",1,fn_echo); h.q1s("SELECT f('hi')","second"); h.check(); } }

#[test] fn c004() { unsafe { let mut h = H::new("engine-udf-001-C004"); reg(h.db,"g",1,fn_twice);
    let v = h.q1("SELECT g(5)"); h.oi("before", v);
    sqlite3_create_function(h.db, CString::new("g").unwrap().as_ptr(), 1, 1, ptr::null_mut(), None, None, None);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare_v2(h.db, CString::new("SELECT g(5)").unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    h.oi("after.prep.rc", rc as i64); sqlite3_finalize(st); h.check(); } }

#[test] fn c005() { unsafe { let mut h = H::new("engine-udf-001-C005"); reg(h.db,"o",1,fn_types_out);
    h.q1s("SELECT quote(o(0))","r0"); let v = h.q1("SELECT o(1)"); h.oi("r1", v);
    h.q1s("SELECT typeof(o(2))","r2t"); h.q1s("SELECT typeof(o(3))","r3t");
    h.q1s("SELECT o(4)","r4"); h.q1s("SELECT quote(o(5))","r5"); h.check(); } }

#[test] fn c006() { unsafe { let mut h = H::new("engine-udf-001-C006");
    sqlite3_create_function(h.db, CString::new("boom").unwrap().as_ptr(), 0, 1, ptr::null_mut(), Some(fn_err), None, None);
    let rc = h.ex("SELECT boom()"); h.oi("rc", rc as i64); let e = h.errmsg(); h.os("errmsg", &e); h.check(); } }

#[test] fn c007() { unsafe { let mut h = H::new("engine-udf-001-C007");
    sqlite3_create_function(h.db, CString::new("addu").unwrap().as_ptr(), 1, 1, 100usize as *mut c_void, Some(fn_addu), None, None);
    let v = h.q1("SELECT addu(23)"); h.oi("v", v); h.check(); } }

#[test] fn c008() { unsafe { let mut h = H::new("engine-udf-001-C008"); reg(h.db,"twice",1,fn_twice);
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(3);");
    let c = h.q1("SELECT count(*) FROM t WHERE twice(a) > 3"); h.oi("cnt", c);
    let n = h.q1("SELECT twice(abs(-9))"); h.oi("nested", n); h.check(); } }

#[test] fn c009() { unsafe { let mut h = H::new("engine-udf-001-C009"); DESTROY.store(0, Ordering::SeqCst);
    sqlite3_create_function_v2(h.db, CString::new("d").unwrap().as_ptr(), 1, 1, ptr::null_mut(), Some(fn_twice), None, None, Some(x_destroy));
    sqlite3_create_function_v2(h.db, CString::new("d").unwrap().as_ptr(), 1, 1, ptr::null_mut(), Some(fn_echo), None, None, Some(x_destroy));
    h.oi("after_replace", DESTROY.load(Ordering::SeqCst));
    sqlite3_close(h.db); h.db = ptr::null_mut();
    h.oi("after_close", DESTROY.load(Ordering::SeqCst));
    // check without re-closing
    let feat = "engine-udf-001"; let cnum = "C009";
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
    p.push(format!("tests/characterization/engine-udf/cases/{feat}/{cnum}.approved.txt"));
    assert_eq!(h.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap()); } }

#[test] fn c010() { unsafe { let mut h = H::new("engine-udf-001-C010");
    let rc = sqlite3_create_function(h.db, CString::new("twd").unwrap().as_ptr(), 1, 1 | 0x800, ptr::null_mut(), Some(fn_twice), None, None);
    h.oi("reg.rc", rc as i64); let v = h.q1("SELECT twd(15)"); h.oi("v", v); h.check(); } }

#[test] fn v001() { unsafe { let mut h = H::new("engine-value-001-C001"); reg(h.db,"fp",1,fn_finger);
    h.q1s("SELECT fp(42)","int"); h.q1s("SELECT fp(2.5)","real"); h.q1s("SELECT fp('hello')","text");
    h.q1s("SELECT fp(X'AABB')","blob"); h.q1s("SELECT fp(NULL)","null"); h.check(); } }

#[test] fn v002() { unsafe { let mut h = H::new("engine-value-001-C002"); reg(h.db,"fp",1,fn_finger);
    h.q1s("SELECT fp('')","empty"); h.q1s("SELECT fp('abc')","abc"); h.check(); } }

#[test] fn v003() { unsafe { let mut h = H::new("engine-value-001-C003"); reg(h.db,"fp",1,fn_finger);
    h.q1s("SELECT fp('123')","t123"); h.q1s("SELECT fp('12.5')","t125"); h.check(); } }

#[test] fn v004() { unsafe { let mut h = H::new("engine-value-001-C004"); reg(h.db,"vi64",1,fn_vi64);
    let v = h.q1int("SELECT vi64(9000000000)"); h.oi("v", v); h.check(); } }

#[test] fn v005() { unsafe { let mut h = H::new("engine-value-001-C005"); reg(h.db,"echo",1,fn_echo);
    h.q1s("SELECT hex(echo(X'DEAD'))","hx"); h.check(); } }

#[test] fn v006() { unsafe { let mut h = H::new("engine-value-001-C006"); reg(h.db,"twice",1,fn_twice);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(h.db, CString::new("SELECT twice(?1)").unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_bind_int(st, 1, 50); sqlite3_step(st); h.oi("v", sqlite3_column_int(st,0) as i64); sqlite3_finalize(st); h.check(); } }

#[test] fn v007() { unsafe { let mut h = H::new("engine-value-001-C007"); reg(h.db,"fp",1,fn_finger);
    h.q1s("SELECT fp(zeroblob(0))","zblob"); h.check(); } }

#[test] fn v008() { unsafe { let mut h = H::new("engine-value-001-C008"); reg(h.db,"cnt",-1,fn_argc);
    let a2 = h.q1("SELECT cnt(1,2)"); h.oi("a2", a2); let a4 = h.q1("SELECT cnt(1,2,3,4)"); h.oi("a4", a4); h.check(); } }

unsafe fn reg_agg(db: *mut Sqlite3, nm: &str) {
    sqlite3_create_function(db, CString::new(nm).unwrap().as_ptr(), 1, 1, ptr::null_mut(), None, Some(agg_step), Some(agg_final));
}

#[test] fn a001() { unsafe { let mut h = H::new("engine-udf-002-C001"); reg_agg(h.db,"mysum");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(3),(4);");
    let v = h.q1("SELECT mysum(a) FROM t"); h.oi("v", v); h.check(); } }

#[test] fn a002() { unsafe { let mut h = H::new("engine-udf-002-C002"); reg_agg(h.db,"mysum");
    h.ex("CREATE TABLE t(a);"); h.q1s("SELECT mysum(a) FROM t","v"); h.check(); } }

#[test] fn a003() { unsafe { let mut h = H::new("engine-udf-002-C003"); reg_agg(h.db,"mysum");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(5),(NULL),(7);");
    let v = h.q1("SELECT mysum(a) FROM t"); h.oi("v", v); h.check(); } }

#[test] fn a004() { unsafe { let mut h = H::new("engine-udf-002-C004"); reg_agg(h.db,"mysum");
    h.ex("CREATE TABLE t(k TEXT, a INTEGER); INSERT INTO t VALUES('x',1),('x',2),('y',10);");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(h.db, CString::new("SELECT k, mysum(a) FROM t GROUP BY k ORDER BY k").unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {
        let p = sqlite3_column_text(st, 0); let k = CStr::from_ptr(p as *const c_char).to_string_lossy().into_owned();
        h.os("k", &k); h.oi("s", sqlite3_column_int(st, 1) as i64);
    }
    sqlite3_finalize(st); h.check(); } }

#[test] fn a005() { unsafe { let mut h = H::new("engine-udf-002-C005"); reg_agg(h.db,"f");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(3),(4);");
    let ag = h.q1("SELECT f(a) FROM t"); h.oi("agg", ag);
    reg(h.db,"f",1,fn_twice); let sc = h.q1("SELECT f(20)"); h.oi("scalar", sc); h.check(); } }

#[test] fn a006() { unsafe { let mut h = H::new("engine-udf-002-C006"); reg_agg(h.db,"mysum");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(100),(200),(300);");
    let v = h.q1("SELECT mysum(a) FROM t"); h.oi("v", v); h.check(); } }

// ---- pack v18 anti-cheat: registered UDFs compute from runtime args ----

fn runtime_int() -> i64 {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    (t.subsec_nanos() as i64 % 90_000) + 1000
}

#[test]
fn anti_cheat_udf_runtime_scalar() { unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    reg(db, "twice", 1, fn_twice);
    let n = runtime_int();
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let q = CString::new(format!("SELECT twice({n})")).unwrap();
    sqlite3_prepare_v2(db, q.as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), 2 * n, "callback computes from the runtime arg");
    sqlite3_finalize(st);
    // an unregistered name errors honestly
    let q2 = CString::new("SELECT notregistered(1)").unwrap();
    let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
    assert_ne!(sqlite3_prepare_v2(db, q2.as_ptr(), -1, &mut st2, ptr::null_mut()), 0);
    sqlite3_finalize(st2);
    sqlite3_close(db);
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}}

#[test]
fn anti_cheat_udf_value_types() { unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    reg(db, "fp", 1, fn_finger);
    let n = runtime_int();
    // a runtime int -> int/nt1/by<len of its decimal>
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let q = CString::new(format!("SELECT fp({n})")).unwrap();
    sqlite3_prepare_v2(db, q.as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    let got = CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy().into_owned();
    assert_eq!(got, format!("int/nt1/by{}", n.to_string().len()));
    sqlite3_finalize(st);
    sqlite3_close(db);
}}

#[test]
fn anti_cheat_udf_aggregate() { unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    reg_agg(db, "mysum");
    let n = runtime_int();
    let ex = |db, s: String| sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    assert_eq!(ex(db, format!("CREATE TABLE t(a); INSERT INTO t VALUES({n}),({}),({});", n + 1, n + 2)), 0);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new("SELECT mysum(a) FROM t").unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    assert_eq!(sqlite3_column_int64(st, 0), 3 * n + 3, "aggregate xStep/xFinal over runtime rows");
    sqlite3_finalize(st);
    sqlite3_close(db);
}}
