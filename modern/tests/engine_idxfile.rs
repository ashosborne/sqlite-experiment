//! Run-27 bespoke replay: durable indexes (multi-col UNIQUE, expression, partial,
//! multi-leaf) + honest EQP — mirrors /tmp/idx_harness.c, byte-identical vs goldens.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::ptr;

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3, path: String }
impl H {
    fn open(cid: &'static str, path: &str) -> H {
        unsafe {
            let mut db: *mut Sqlite3 = ptr::null_mut();
            sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
            H { cid, lines: Vec::new(), db, path: path.to_string() }
        }
    }
    fn ex(&mut self, s: &str) -> i32 { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()) } }
    fn q1(&mut self, s: &str) -> i64 { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut(); let c = CString::new(s).unwrap(); let mut v = -999i64;
        if sqlite3_prepare_v2(self.db, c.as_ptr(), -1, &mut st, ptr::null_mut()) == 0 { if sqlite3_step(st) == 100 { v = sqlite3_column_int64(st, 0); } }
        sqlite3_finalize(st); v } }
    fn q1s(&mut self, s: &str, label: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut(); let c = CString::new(s).unwrap();
        let mut val = "(none)".to_string();
        if sqlite3_prepare_v2(self.db, c.as_ptr(), -1, &mut st, ptr::null_mut()) == 0 {
            if sqlite3_step(st) == 100 { let p = sqlite3_column_text(st, 0);
                val = if p.is_null() { "NULL".into() } else { CStr::from_ptr(p as *const c_char).to_string_lossy().into_owned() }; } }
        sqlite3_finalize(st); self.os(label, &val); } }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn os(&mut self, n: &str, v: &str) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn reopen(&mut self) { unsafe {
        sqlite3_close(self.db); let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(self.path.clone()).unwrap().as_ptr(), &mut db); self.db = db; } }
    fn check(mut self) {
        unsafe { sqlite3_close(self.db); }
        let _ = std::fs::remove_file(&self.path);
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
fn path(tag: &str) -> String { let p = format!("/tmp/idx27r_{}_{}.db", std::process::id(), tag); let _ = std::fs::remove_file(&p); p }

#[test]
fn c001() { let p = path("1"); let mut h = H::open("engine-idxfile-001-C001", &p);
    let rows: String = (1..=500).map(|i| format!("({i},'v{i}')")).collect::<Vec<_>>().join(",");
    let r = h.ex(&format!("CREATE TABLE t(k INTEGER, v TEXT); CREATE INDEX ik ON t(k); INSERT INTO t VALUES {rows};")); h.oi("w.rc", r as i64);
    h.reopen(); h.q1s("SELECT v FROM t WHERE k = 333", "v333");
    let c = h.q1("SELECT count(*) FROM t"); h.oi("count", c);
    let il = h.q1("SELECT count(*) FROM pragma_index_list('t')"); h.oi("ilist", il);
    h.check(); }

#[test]
fn c002() { let p = path("2"); let mut h = H::open("engine-idxfile-001-C002", &p);
    let r = h.ex("CREATE TABLE mu(a INTEGER, b INTEGER); CREATE UNIQUE INDEX mu2 ON mu(a, b); INSERT INTO mu VALUES(1,1),(1,2);"); h.oi("w.rc", r as i64);
    h.reopen();
    let r = h.ex("INSERT INTO mu VALUES(1,1);"); h.oi("dup.rc", r as i64);
    let r = h.ex("INSERT INTO mu VALUES(2,1);"); h.oi("ok.rc", r as i64);
    let c = h.q1("SELECT count(*) FROM mu"); h.oi("count", c);
    h.check(); }

#[test]
fn c003() { let p = path("3"); let mut h = H::open("engine-idxfile-001-C003", &p);
    let r = h.ex("CREATE TABLE nx(nm TEXT); CREATE INDEX ie ON nx(lower(nm)); INSERT INTO nx VALUES('Ann'),('BOB');"); h.oi("w.rc", r as i64);
    h.reopen();
    let ic = h.q1("SELECT count(*) FROM sqlite_master WHERE type='index'"); h.oi("icount", ic);
    h.q1s("SELECT nm FROM nx WHERE lower(nm) = 'bob'", "nm");
    h.check(); }

#[test]
fn c004() { let p = path("4"); let mut h = H::open("engine-idxfile-001-C004", &p);
    let r = h.ex("CREATE TABLE pu(a INTEGER); CREATE UNIQUE INDEX pux ON pu(a) WHERE a > 10; INSERT INTO pu VALUES(5),(5),(20);"); h.oi("w.rc", r as i64);
    h.reopen();
    let r = h.ex("INSERT INTO pu VALUES(5);"); h.oi("dup5.rc", r as i64);
    let r = h.ex("INSERT INTO pu VALUES(20);"); h.oi("dup20.rc", r as i64);
    let c = h.q1("SELECT count(*) FROM pu"); h.oi("count", c);
    h.check(); }

#[test]
fn c005() { let p = path("5"); let mut h = H::open("engine-idxfile-001-C005", &p);
    let rows: String = (1..=600).map(|i| format!("('key{:04}',{i})", i)).collect::<Vec<_>>().join(",");
    let r = h.ex(&format!("CREATE TABLE big(nm TEXT, n INTEGER); CREATE INDEX bi ON big(nm); INSERT INTO big VALUES {rows};")); h.oi("w.rc", r as i64);
    h.reopen();
    let n = h.q1("SELECT n FROM big WHERE nm = 'key0432'"); h.oi("n", n);
    let c = h.q1("SELECT count(*) FROM big"); h.oi("count", c);
    h.q1s("PRAGMA integrity_check", "ic");
    h.check(); }

#[test]
fn c006() { let p = path("6"); let mut h = H::open("engine-idxfile-001-C006", &p);
    let r = h.ex("CREATE TABLE m(a INTEGER, b INTEGER); CREATE INDEX mab ON m(a, b); DROP INDEX mab; INSERT INTO m VALUES(1,2);"); h.oi("w.rc", r as i64);
    h.reopen();
    let ic = h.q1("SELECT count(*) FROM sqlite_master WHERE type='index'"); h.oi("icount", ic);
    let c = h.q1("SELECT count(*) FROM m"); h.oi("count", c);
    h.check(); }

#[test]
fn c007_eqp() { let mut h = H::open("engine-idxfile-001-C007", ":memory:");
    h.ex("CREATE TABLE t(k INTEGER, v INTEGER); CREATE INDEX ik ON t(k); INSERT INTO t VALUES(1,2),(5,6);");
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let c = CString::new("EXPLAIN QUERY PLAN SELECT k, v FROM t WHERE k = 5").unwrap();
        sqlite3_prepare_v2(h.db, c.as_ptr(), -1, &mut st, ptr::null_mut());
        while sqlite3_step(st) == 100 { let p = sqlite3_column_text(st, 3);
            let d = CStr::from_ptr(p as *const c_char).to_string_lossy().into_owned(); h.os("detail", &d); }
        sqlite3_finalize(st);
    }
    h.check(); }

#[test]
fn c008() { let p = path("8"); let mut h = H::open("engine-idxfile-001-C008", &p);
    let r = h.ex("CREATE TABLE t(k INTEGER, v TEXT); CREATE INDEX ik ON t(k); INSERT INTO t VALUES(1,'a'),(2,'b');"); h.oi("w.rc", r as i64);
    h.reopen();
    h.q1s("SELECT v FROM t WHERE k = 99", "v99");
    let c = h.q1("SELECT count(*) FROM t WHERE k = 99"); h.oi("cnt", c);
    h.check(); }
