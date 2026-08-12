//! Run-29 UTF-16 prepare + column16 replay — mirrors /tmp/utf16_harness.c using real
//! u16 buffers, asserted byte-identical against the frozen C goldens.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::path::PathBuf;
use std::ptr;

fn to16(s: &str) -> Vec<u16> { let mut v: Vec<u16> = s.encode_utf16().collect(); v.push(0); v }
unsafe fn from16(p: *const c_void) -> Option<String> {
    if p.is_null() { return None; }
    let u = p as *const u16; let mut v = Vec::new(); let mut i = 0isize;
    loop { let c = *u.offset(i); if c == 0 { break; } v.push(c); i += 1; }
    Some(String::from_utf16_lossy(&v))
}

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn os(&mut self, n: &str, v: Option<String>) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v.unwrap_or_else(|| "NULL".into()))); }
    fn errmsg(&self) -> String { unsafe { CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy().into_owned() } }
    fn check(self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
unsafe fn text8(st: *mut Sqlite3Stmt, i: i32) -> Option<String> {
    let p = sqlite3_column_text(st, i); if p.is_null() { None } else { Some(CStr::from_ptr(p as *const c_char).to_string_lossy().into_owned()) }
}

#[test] fn c001() { unsafe { let mut h = H::new("engine-utf16-001-C001");
    let q = to16("SELECT 1"); let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare16_v2(h.db, q.as_ptr() as *const c_void, -1, &mut st, ptr::null_mut());
    h.oi("prep.rc", rc as i64); h.oi("step.rc", sqlite3_step(st) as i64); h.oi("v", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st); h.check(); } }

#[test] fn c002() { unsafe { let mut h = H::new("engine-utf16-001-C002");
    let q = to16("SELECT 2+3"); let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare16(h.db, q.as_ptr() as *const c_void, -1, &mut st, ptr::null_mut());
    h.oi("prep.rc", rc as i64); sqlite3_step(st); h.oi("v", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st); h.check(); } }

#[test] fn c003() { unsafe { let mut h = H::new("engine-utf16-001-C003");
    let q = to16("SELECT 7"); let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare16_v3(h.db, q.as_ptr() as *const c_void, -1, 1, &mut st, ptr::null_mut());
    h.oi("prep.rc", rc as i64); sqlite3_step(st); h.oi("v", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st); h.check(); } }

#[test] fn c004() { unsafe { let mut h = H::new("engine-utf16-001-C004");
    let q = to16("SELECT 1; SELECT 2"); let mut st: *mut Sqlite3Stmt = ptr::null_mut(); let mut tail: *const c_void = ptr::null();
    let rc = sqlite3_prepare16_v2(h.db, q.as_ptr() as *const c_void, -1, &mut st, &mut tail);
    h.oi("prep.rc", rc as i64); h.os("tail", from16(tail));
    sqlite3_step(st); h.oi("v1", sqlite3_column_int(st,0) as i64); sqlite3_finalize(st);
    let rc2 = sqlite3_prepare16_v2(h.db, tail, -1, &mut st, &mut tail);
    h.oi("prep2.rc", rc2 as i64); sqlite3_step(st); h.oi("v2", sqlite3_column_int(st,0) as i64); sqlite3_finalize(st);
    h.check(); } }

#[test] fn c005() { unsafe { let mut h = H::new("engine-utf16-001-C005");
    let q = to16("SELECT 9 xxx"); let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare16_v2(h.db, q.as_ptr() as *const c_void, 8*2, &mut st, ptr::null_mut());
    h.oi("prep.rc", rc as i64); sqlite3_step(st); h.oi("v", sqlite3_column_int(st,0) as i64);
    sqlite3_finalize(st); h.check(); } }

#[test] fn c006() { unsafe { let mut h = H::new("engine-utf16-001-C006");
    let q = to16("   "); let mut st: *mut Sqlite3Stmt = 1usize as *mut Sqlite3Stmt;
    let rc = sqlite3_prepare16_v2(h.db, q.as_ptr() as *const c_void, -1, &mut st, ptr::null_mut());
    h.oi("prep.rc", rc as i64); h.oi("stmt.null", st.is_null() as i64); sqlite3_finalize(st); h.check(); } }

#[test] fn c007() { unsafe { let mut h = H::new("engine-utf16-001-C007");
    let q = to16("SELECTT 1"); let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare16_v2(h.db, q.as_ptr() as *const c_void, -1, &mut st, ptr::null_mut());
    h.oi("prep.rc", rc as i64); let e = h.errmsg(); h.os("errmsg", Some(e)); sqlite3_finalize(st); h.check(); } }

#[test] fn c008() { unsafe { let mut h = H::new("engine-utf16-001-C008");
    let q = to16("CREATE TABLE t(a); INSERT INTO t VALUES(41),(42)"); let mut st: *mut Sqlite3Stmt = ptr::null_mut(); let mut tail: *const c_void = ptr::null();
    sqlite3_prepare16_v2(h.db, q.as_ptr() as *const c_void, -1, &mut st, &mut tail); sqlite3_step(st); sqlite3_finalize(st);
    sqlite3_prepare16_v2(h.db, tail, -1, &mut st, &mut tail); sqlite3_step(st); sqlite3_finalize(st);
    let q2 = to16("SELECT count(*), max(a) FROM t"); sqlite3_prepare16_v2(h.db, q2.as_ptr() as *const c_void, -1, &mut st, ptr::null_mut());
    sqlite3_step(st); h.oi("cnt", sqlite3_column_int(st,0) as i64); h.oi("mx", sqlite3_column_int(st,1) as i64); sqlite3_finalize(st);
    h.check(); } }

#[test] fn c009() { unsafe { let mut h = H::new("engine-utf16-001-C009");
    let q = to16("SELECT 'café 😀'"); let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare16_v2(h.db, q.as_ptr() as *const c_void, -1, &mut st, ptr::null_mut());
    sqlite3_step(st); h.os("utf8", text8(st,0)); h.os("utf16", from16(sqlite3_column_text16(st,0)));
    sqlite3_finalize(st); h.check(); } }

#[test] fn c010() { unsafe { let mut h = H::new("engine-utf16-001-C010");
    let q = to16("SELECT ?1"); let p = to16("héllo"); let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare16_v2(h.db, q.as_ptr() as *const c_void, -1, &mut st, ptr::null_mut());
    sqlite3_bind_text16(st, 1, p.as_ptr() as *const c_void, -1, usize::MAX as *mut c_void);
    sqlite3_step(st); h.os("utf8", text8(st,0)); h.os("utf16", from16(sqlite3_column_text16(st,0)));
    sqlite3_finalize(st); h.check(); } }

unsafe fn prep8(db: *mut Sqlite3, s: &str) -> *mut Sqlite3Stmt {
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(s).unwrap().as_ptr(), -1, &mut st, ptr::null_mut()); st }

#[test] fn v001() { unsafe { let mut h = H::new("engine-utf16-002-C001");
    let st = prep8(h.db, "SELECT NULL, 7, 2.5, 'txt', X'414243'"); sqlite3_step(st);
    for i in 0..5 { h.os(&format!("s{i}"), from16(sqlite3_column_text16(st,i))); } sqlite3_finalize(st); h.check(); } }

#[test] fn v002() { unsafe { let mut h = H::new("engine-utf16-002-C002");
    let st = prep8(h.db, "SELECT 'hi', 'héllo', 7, NULL"); sqlite3_step(st);
    h.oi("b0", sqlite3_column_bytes16(st,0) as i64); h.oi("b1", sqlite3_column_bytes16(st,1) as i64);
    h.oi("b2", sqlite3_column_bytes16(st,2) as i64); h.oi("b3", sqlite3_column_bytes16(st,3) as i64);
    sqlite3_finalize(st); h.check(); } }

#[test] fn v003() { unsafe { let mut h = H::new("engine-utf16-002-C003");
    let st = prep8(h.db, "SELECT 1 AS alpha, 2 AS beta");
    h.os("n0", from16(sqlite3_column_name16(st,0))); h.os("n1", from16(sqlite3_column_name16(st,1)));
    sqlite3_finalize(st); h.check(); } }

#[test] fn v004() { unsafe { let mut h = H::new("engine-utf16-002-C004");
    h.ex("CREATE TABLE t(a INTEGER, b TEXT);");
    let st = prep8(h.db, "SELECT a, b, a+1 FROM t");
    h.os("d0", from16(sqlite3_column_decltype16(st,0))); h.os("d1", from16(sqlite3_column_decltype16(st,1)));
    h.os("d2", from16(sqlite3_column_decltype16(st,2))); sqlite3_finalize(st); h.check(); } }

#[test] fn v005() { unsafe { let mut h = H::new("engine-utf16-002-C005");
    let st = prep8(h.db, "SELECT 'abc'"); sqlite3_step(st);
    h.os("t8", text8(st,0)); h.os("t16", from16(sqlite3_column_text16(st,0))); h.os("t8_after", text8(st,0));
    sqlite3_finalize(st); h.check(); } }

#[test] fn v006() { unsafe { let mut h = H::new("engine-utf16-002-C006");
    let st = prep8(h.db, "SELECT 5");
    h.oi("before.b16", sqlite3_column_bytes16(st,0) as i64);
    sqlite3_step(st); sqlite3_step(st);
    h.oi("done.b16", sqlite3_column_bytes16(st,0) as i64);
    h.os("oor.name16", from16(sqlite3_column_name16(st,99))); sqlite3_finalize(st); h.check(); } }

#[test] fn v007() { unsafe { let mut h = H::new("engine-utf16-002-C007");
    let st = prep8(h.db, "SELECT '', NULL"); sqlite3_step(st);
    h.os("empty", from16(sqlite3_column_text16(st,0))); h.oi("empty.b16", sqlite3_column_bytes16(st,0) as i64);
    h.os("null", from16(sqlite3_column_text16(st,1))); h.oi("null.b16", sqlite3_column_bytes16(st,1) as i64);
    sqlite3_finalize(st); h.check(); } }

#[test] fn v008() { unsafe { let mut h = H::new("engine-utf16-002-C008");
    h.ex("CREATE TABLE t(x TEXT); INSERT INTO t VALUES('café'),('😀');");
    let st = prep8(h.db, "SELECT x FROM t ORDER BY rowid");
    sqlite3_step(st); h.os("a", from16(sqlite3_column_text16(st,0))); h.oi("a.b16", sqlite3_column_bytes16(st,0) as i64);
    sqlite3_step(st); h.os("b", from16(sqlite3_column_text16(st,0))); h.oi("b.b16", sqlite3_column_bytes16(st,0) as i64);
    sqlite3_finalize(st); h.check(); } }

// ---- run-29 mandatory anti-cheat (runtime-built inputs; cannot be script-matched) ----

#[test] fn anti_cheat_prepare16_runtime() { unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    let seed = (std::process::id() % 977 + 13) as i64;
    let q = to16(&format!("SELECT {seed} * 3 + 1, 'r' || {seed}"));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    assert_eq!(sqlite3_prepare16_v2(db, q.as_ptr() as *const c_void, -1, &mut st, ptr::null_mut()), 0);
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed * 3 + 1);
    assert_eq!(from16(sqlite3_column_text16(st, 1)).unwrap(), format!("r{seed}"));
    sqlite3_finalize(st); sqlite3_close(db);
} }

#[test] fn anti_cheat_column_text16_roundtrip() { unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    let word = format!("Zürich😀-{}", std::process::id() % 100000);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new("SELECT ?1").unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    let p = to16(&word);
    sqlite3_bind_text16(st, 1, p.as_ptr() as *const c_void, -1, usize::MAX as *mut c_void);
    sqlite3_step(st);
    assert_eq!(from16(sqlite3_column_text16(st, 0)).unwrap(), word);
    assert_eq!(sqlite3_column_bytes16(st, 0) as usize, word.encode_utf16().count() * 2);
    // UTF-8 twin of the same cell must agree
    assert_eq!(text8(st, 0).unwrap(), word);
    sqlite3_finalize(st); sqlite3_close(db);
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
