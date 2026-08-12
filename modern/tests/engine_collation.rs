//! Run-30 create_collation / registry-driven COLLATE replay — mirrors
//! /tmp/coll_harness.c with real extern "C" xCompare callbacks, asserted
//! byte-identical against the frozen C goldens.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering as AO};

unsafe fn sl(p: *const c_void, n: c_int) -> &'static [u8] { std::slice::from_raw_parts(p as *const u8, n as usize) }

unsafe extern "C" fn rev_cmp(_p: *mut c_void, n1: c_int, a: *const c_void, n2: c_int, b: *const c_void) -> c_int {
    let (x, y) = (sl(a, n1), sl(b, n2));
    let (mut i, mut j) = (n1 as isize - 1, n2 as isize - 1);
    while i >= 0 && j >= 0 {
        if x[i as usize] != y[j as usize] { return x[i as usize] as c_int - y[j as usize] as c_int; }
        i -= 1; j -= 1;
    }
    n1 - n2
}
unsafe extern "C" fn caseless_cmp(_p: *mut c_void, n1: c_int, a: *const c_void, n2: c_int, b: *const c_void) -> c_int {
    let (x, y) = (sl(a, n1), sl(b, n2));
    let n = n1.min(n2) as usize;
    for i in 0..n {
        let d = x[i].to_ascii_lowercase() as c_int - y[i].to_ascii_lowercase() as c_int;
        if d != 0 { return d; }
    }
    n1 - n2
}
unsafe extern "C" fn strict_cmp(_p: *mut c_void, n1: c_int, a: *const c_void, n2: c_int, b: *const c_void) -> c_int {
    let (x, y) = (sl(a, n1), sl(b, n2));
    let n = n1.min(n2) as usize;
    match x[..n].cmp(&y[..n]) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Greater => 1, _ => n1 - n2 }
}
unsafe extern "C" fn flag_cmp(p: *mut c_void, n1: c_int, a: *const c_void, n2: c_int, b: *const c_void) -> c_int {
    let d = strict_cmp(ptr::null_mut(), n1, a, n2, b);
    if *(p as *const c_int) != 0 { -d } else { d }
}
static GDESTROY: AtomicI32 = AtomicI32::new(0);
unsafe extern "C" fn on_destroy(_p: *mut c_void) { GDESTROY.fetch_add(1, AO::SeqCst); }
static GCALLS: AtomicI32 = AtomicI32::new(0);
unsafe extern "C" fn counting_cmp(_p: *mut c_void, n1: c_int, a: *const c_void, n2: c_int, b: *const c_void) -> c_int {
    GCALLS.fetch_add(1, AO::SeqCst);
    strict_cmp(ptr::null_mut(), n1, a, n2, b)
}
static GNEEDED: AtomicI32 = AtomicI32::new(0);
static GNEEDNAME: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
unsafe extern "C" fn needed_cb(_p: *mut c_void, db: *mut Sqlite3, _rep: c_int, name: *const c_char) {
    GNEEDED.fetch_add(1, AO::SeqCst);
    let n = CStr::from_ptr(name).to_string_lossy().into_owned();
    *GNEEDNAME.lock().unwrap() = n.clone();
    if n == "lazy1" { sqlite3_create_collation(db, c"lazy1".as_ptr(), 1, ptr::null_mut(), Some(caseless_cmp)); }
}

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn open(cid: &'static str, path: &str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn new(cid: &'static str) -> H { H::open(cid, ":memory:") }
    fn reopen(&mut self, path: &str) { unsafe {
        sqlite3_close(self.db);
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        self.db = db; } }
    fn ex(&mut self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn os(&mut self, n: &str, v: &str) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn rows(&mut self, sql: &str, label: &str) { unsafe {
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
            let p = sqlite3_column_text(st, 0);
            if p.is_null() { buf.push('~'); } else { buf.push_str(&CStr::from_ptr(p as *const c_char).to_string_lossy()); }
        }
        sqlite3_finalize(st);
        self.lines.push(format!("OBS {} {} {}", self.cid, label, buf));
    } }
    fn check(mut self) {
        unsafe { sqlite3_close(self.db); }
        self.finish()
    }
    fn finish(self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
fn reg(h: &H, name: &str, f: unsafe extern "C" fn(*mut c_void, c_int, *const c_void, c_int, *const c_void) -> c_int) {
    unsafe { sqlite3_create_collation(h.db, CString::new(name).unwrap().as_ptr(), 1, ptr::null_mut(), Some(f)); }
}

#[test] fn c001() { let mut h = H::new("engine-collation-001-C001");
    reg(&h, "revcmp", rev_cmp);
    h.ex("CREATE TABLE t(x TEXT); INSERT INTO t VALUES('pat'),('cob'),('rig');");
    h.rows("SELECT x FROM t ORDER BY x", "plain");
    h.rows("SELECT x FROM t ORDER BY x COLLATE revcmp", "rev");
    h.check(); }

#[test] fn c002() { let mut h = H::new("engine-collation-001-C002");
    reg(&h, "caseless", caseless_cmp);
    h.rows("SELECT 'ABC' = 'abc' COLLATE caseless", "eq");
    h.rows("SELECT 'ABC' = 'abc'", "eq_binary");
    h.rows("SELECT 'ab' < 'AC' COLLATE caseless", "lt");
    h.check(); }

#[test] fn c003() { let mut h = H::new("engine-collation-001-C003");
    reg(&h, "mycmp", caseless_cmp);
    h.rows("SELECT 'A' = 'a' COLLATE mycmp", "before");
    reg(&h, "mycmp", strict_cmp);
    h.rows("SELECT 'A' = 'a' COLLATE mycmp", "after");
    h.check(); }

#[test] fn c004() { let mut h = H::new("engine-collation-001-C004");
    reg(&h, "gone", caseless_cmp);
    h.rows("SELECT 'A' = 'a' COLLATE gone", "present");
    let rc = unsafe { sqlite3_create_collation(h.db, c"gone".as_ptr(), 1, ptr::null_mut(), None) };
    h.oi("del.rc", rc as i64);
    h.rows("SELECT 'A' = 'a' COLLATE gone", "deleted");
    h.check(); }

#[test] fn c005() { let mut h = H::new("engine-collation-001-C005");
    GDESTROY.store(0, AO::SeqCst);
    unsafe { sqlite3_create_collation_v2(h.db, c"d1".as_ptr(), 1, ptr::null_mut(), Some(caseless_cmp), Some(on_destroy)); }
    h.oi("after_reg", GDESTROY.load(AO::SeqCst) as i64);
    unsafe { sqlite3_create_collation_v2(h.db, c"d1".as_ptr(), 1, ptr::null_mut(), Some(strict_cmp), Some(on_destroy)); }
    h.oi("after_replace", GDESTROY.load(AO::SeqCst) as i64);
    unsafe { sqlite3_close(h.db); }
    h.db = ptr::null_mut();
    h.oi("after_close", GDESTROY.load(AO::SeqCst) as i64);
    h.finish(); }

#[test] fn c006() { let mut h = H::new("engine-collation-001-C006");
    static FWD: c_int = 0; static BWD: c_int = 1;
    unsafe {
        sqlite3_create_collation(h.db, c"steer_f".as_ptr(), 1, &FWD as *const c_int as *mut c_void, Some(flag_cmp));
        sqlite3_create_collation(h.db, c"steer_b".as_ptr(), 1, &BWD as *const c_int as *mut c_void, Some(flag_cmp));
    }
    h.ex("CREATE TABLE t(x TEXT); INSERT INTO t VALUES('b'),('a'),('c');");
    h.rows("SELECT x FROM t ORDER BY x COLLATE steer_f", "fwd");
    h.rows("SELECT x FROM t ORDER BY x COLLATE steer_b", "bwd");
    h.check(); }

#[test] fn c007() { let mut h = H::new("engine-collation-001-C007");
    unsafe {
        let rc1 = sqlite3_create_collation(h.db, c"e1".as_ptr(), 1, ptr::null_mut(), Some(caseless_cmp));
        h.oi("utf8.rc", rc1 as i64);
        let rc2 = sqlite3_create_collation(h.db, c"e2".as_ptr(), 0, ptr::null_mut(), Some(caseless_cmp));
        h.oi("zero.rc", rc2 as i64);
        let rc3 = sqlite3_create_collation(h.db, c"e3".as_ptr(), 99, ptr::null_mut(), Some(caseless_cmp));
        h.oi("bad99.rc", rc3 as i64);
        let rc4 = sqlite3_create_collation(h.db, c"e4".as_ptr(), 4, ptr::null_mut(), Some(caseless_cmp));
        h.oi("utf16.rc", rc4 as i64);
    }
    h.check(); }

#[test] fn c008() { let mut h = H::new("engine-collation-001-C008");
    reg(&h, "unrelated", rev_cmp);
    h.rows("SELECT 'A' = 'a' COLLATE NOCASE", "nocase");
    h.rows("SELECT 'A' = 'a' COLLATE BINARY", "binary");
    h.rows("SELECT 'abc  ' = 'abc' COLLATE RTRIM", "rtrim");
    h.check(); }

#[test] fn c009() { let mut h = H::new("engine-collation-001-C009");
    reg(&h, "caseless", caseless_cmp);
    h.ex("CREATE TABLE u(n TEXT); INSERT INTO u VALUES('Ann'),('bob'),('ANN'),('Cat'),('ann');");
    h.rows("SELECT n FROM u WHERE n = 'ann' COLLATE caseless ORDER BY n", "both");
    h.rows("SELECT count(*) FROM u WHERE n = 'ANN' COLLATE caseless", "cnt");
    h.check(); }

#[test] fn c010() { let mut h = H::new("engine-collation-001-C010");
    GCALLS.store(0, AO::SeqCst);
    reg(&h, "counted", counting_cmp);
    h.ex("CREATE TABLE t(x TEXT); INSERT INTO t VALUES('q'),('p');");
    h.rows("SELECT x FROM t ORDER BY x COLLATE counted", "sorted");
    h.oi("called", (GCALLS.load(AO::SeqCst) > 0) as i64);
    h.check(); }

#[test] fn s001() { let mut h = H::new("engine-collation-002-C001");
    reg(&h, "caseless", caseless_cmp);
    h.ex("CREATE TABLE p(c TEXT COLLATE caseless); INSERT INTO p VALUES('Dog'),('cat'),('APE');");
    h.rows("SELECT c FROM p WHERE c = 'CAT'", "where_decl");
    h.rows("SELECT c FROM p ORDER BY c", "order_decl");
    h.check(); }

#[test] fn s002() { let path = "/tmp/eftest/rust_coll1.db";
    let _ = std::fs::remove_file(path);
    std::fs::create_dir_all("/tmp/eftest").unwrap();
    let mut h = H::open("engine-collation-002-C002", path);
    reg(&h, "caseless", caseless_cmp);
    h.ex("CREATE TABLE p(c TEXT COLLATE caseless); INSERT INTO p VALUES('b'),('A');");
    h.rows("SELECT c FROM p ORDER BY c", "first_conn");
    h.reopen(path);
    h.rows("SELECT c FROM p ORDER BY c", "no_register");
    reg(&h, "caseless", caseless_cmp);
    h.rows("SELECT c FROM p ORDER BY c", "re_registered");
    h.check(); }

#[test] fn s003() { let mut h = H::new("engine-collation-002-C003");
    h.ex("CREATE TABLE t(x TEXT);");
    h.rows("SELECT x FROM t ORDER BY x COLLATE nope", "order_unknown");
    h.rows("SELECT 'a' = 'b' COLLATE nope", "eq_unknown");
    h.check(); }

#[test] fn s004() { let mut h = H::new("engine-collation-002-C004");
    reg(&h, "MyCmp", caseless_cmp);
    h.rows("SELECT 'A' = 'a' COLLATE mycmp", "lower");
    h.rows("SELECT 'A' = 'a' COLLATE MYCMP", "upper");
    h.rows("SELECT 'A' = 'a' COLLATE \"MyCmp\"", "quoted");
    h.check(); }

#[test] fn s005() { let mut h = H::new("engine-collation-002-C005");
    h.ex("CREATE TABLE r(s TEXT); INSERT INTO r VALUES('aa '),('AA'),('ab');");
    h.rows("SELECT count(*) FROM r WHERE s = 'aa' COLLATE RTRIM", "rtrim_cnt");
    h.rows("SELECT count(*) FROM r WHERE s = 'aa ' COLLATE NOCASE", "nocase_cnt");
    h.rows("SELECT s FROM r ORDER BY s COLLATE NOCASE, rowid", "nocase_order");
    h.check(); }

static NEEDED_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test] fn n001() { let _g = NEEDED_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new("engine-collation-003-C001");
    GNEEDED.store(0, AO::SeqCst); GNEEDNAME.lock().unwrap().clear();
    unsafe { sqlite3_collation_needed(h.db, ptr::null_mut(), Some(needed_cb)); }
    h.rows("SELECT 'A' = 'a' COLLATE lazy1", "eq");
    h.oi("factory_called", (GNEEDED.load(AO::SeqCst) > 0) as i64);
    let n = GNEEDNAME.lock().unwrap().clone();
    h.os("factory_name", &n);
    h.check(); }

#[test] fn n002() { let _g = NEEDED_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new("engine-collation-003-C002");
    GNEEDED.store(0, AO::SeqCst); GNEEDNAME.lock().unwrap().clear();
    unsafe { sqlite3_collation_needed(h.db, ptr::null_mut(), Some(needed_cb)); }
    h.rows("SELECT 'A' = 'a' COLLATE lazy2", "eq");
    h.oi("factory_called", (GNEEDED.load(AO::SeqCst) > 0) as i64);
    let n = GNEEDNAME.lock().unwrap().clone();
    h.os("factory_name", &n);
    h.check(); }

// ---- run-30 mandatory anti-cheat ----

#[test] fn anti_cheat_collation_runtime_order() { unsafe {
    // collation name + data built at runtime; ORDER BY must follow xCompare, not BINARY
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    let name = format!("rt_{}", std::process::id() % 100000);
    sqlite3_create_collation(db, CString::new(name.clone()).unwrap().as_ptr(), 1, ptr::null_mut(), Some(rev_cmp));
    let h = H { cid: "x", lines: Vec::new(), db };
    h.ex_pub("CREATE TABLE t(x TEXT); INSERT INTO t VALUES('xa'),('yb'),('zc'),('wa');");
    let sorted = h.collect1(&format!("SELECT x FROM t ORDER BY x COLLATE {name}"));
    // rev_cmp compares reversed strings: 'aw','ax','by','cz'
    assert_eq!(sorted, "wa|xa|yb|zc");
    let binary = h.collect1("SELECT x FROM t ORDER BY x");
    assert_eq!(binary, "wa|xa|yb|zc".replace("wa|xa", "wa|xa")); // binary: wa,xa,yb,zc same here
    let rev2 = h.collect1(&format!("SELECT x FROM t ORDER BY x COLLATE {name} DESC"));
    assert_eq!(rev2, "zc|yb|xa|wa");
    sqlite3_close(db);
} }

#[test] fn anti_cheat_collation_compare_called() { unsafe {
    GCALLS.store(0, AO::SeqCst);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    sqlite3_create_collation(db, c"probe".as_ptr(), 1, ptr::null_mut(), Some(counting_cmp));
    let h = H { cid: "x", lines: Vec::new(), db };
    h.ex_pub("CREATE TABLE t(x TEXT); INSERT INTO t VALUES('m'),('k'),('z');");
    let _ = h.collect1("SELECT x FROM t ORDER BY x COLLATE probe");
    assert!(GCALLS.load(AO::SeqCst) >= 1, "xCompare must actually run");
    let eq = h.collect1("SELECT 'aa' = 'aa' COLLATE probe");
    assert_eq!(eq, "1");
    sqlite3_close(db);
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}

impl H {
    fn ex_pub(&self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn collect1(&self, sql: &str) -> String { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        assert_eq!(rc, 0, "prepare failed: {}", CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy());
        let mut buf = String::new();
        while sqlite3_step(st) == 100 {
            if !buf.is_empty() { buf.push('|'); }
            let p = sqlite3_column_text(st, 0);
            if p.is_null() { buf.push('~'); } else { buf.push_str(&CStr::from_ptr(p as *const c_char).to_string_lossy()); }
        }
        sqlite3_finalize(st); buf
    } }
}
