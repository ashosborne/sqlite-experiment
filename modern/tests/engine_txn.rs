//! Run-25 bespoke replay: OR ROLLBACK / autocommit / file-reopen txn cases —
//! mirrors /tmp/txn_harness.c and asserts byte-identical OBS lines vs frozen goldens.
use sqlite3_rust_spine::*;
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr;

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn open(cid: &'static str, path: &str) -> H {
        unsafe {
            let mut db: *mut Sqlite3 = ptr::null_mut();
            sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
            H { cid, lines: Vec::new(), db }
        }
    }
    fn ex(&mut self, sql: &str) -> i32 {
        unsafe { sqlite3_exec(self.db, CString::new(sql).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()) }
    }
    fn q1(&mut self, sql: &str) -> i64 {
        unsafe {
            let mut st: *mut Sqlite3Stmt = ptr::null_mut();
            let c = CString::new(sql).unwrap();
            let mut v = -999i64;
            if sqlite3_prepare_v2(self.db, c.as_ptr(), -1, &mut st, ptr::null_mut()) == 0 {
                if sqlite3_step(st) == 100 { v = sqlite3_column_int64(st, 0); }
            }
            sqlite3_finalize(st);
            v
        }
    }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn reopen(&mut self, path: &str) {
        unsafe {
            sqlite3_close(self.db);
            let mut db: *mut Sqlite3 = ptr::null_mut();
            sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
            self.db = db;
        }
    }
    fn check(mut self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
unsafe fn ac(db: *mut Sqlite3) -> i64 { sqlite3_get_autocommit(db) as i64 }

#[test]
fn orrollback_c001() { unsafe {
    let mut h = H::open("engine-orrollback-001-C001", ":memory:");
    h.ex("CREATE TABLE q(a INTEGER UNIQUE); ");
    let r = h.ex("BEGIN; INSERT INTO q VALUES(1);"); h.oi("begin.rc", r as i64);
    let r = h.ex("INSERT OR ROLLBACK INTO q VALUES(1);"); h.oi("dup.rc", r as i64);
    let a = ac(h.db); h.oi("autocommit", a);
    let r = h.ex("COMMIT;"); h.oi("commit.rc", r as i64);
    let c = h.q1("SELECT count(*) FROM q"); h.oi("count", c);
    h.check();
}}

#[test]
fn orrollback_c002() { unsafe {
    let mut h = H::open("engine-orrollback-001-C002", ":memory:");
    h.ex("CREATE TABLE q(a INTEGER UNIQUE);");
    let r = h.ex("BEGIN; INSERT INTO q VALUES(1);"); h.oi("begin.rc", r as i64);
    let r = h.ex("INSERT OR ABORT INTO q VALUES(1);"); h.oi("dup.rc", r as i64);
    let a = ac(h.db); h.oi("autocommit", a);
    let r = h.ex("COMMIT;"); h.oi("commit.rc", r as i64);
    let c = h.q1("SELECT count(*) FROM q"); h.oi("count", c);
    h.check();
}}

#[test]
fn orrollback_c003() { unsafe {
    let mut h = H::open("engine-orrollback-001-C003", ":memory:");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id));");
    let r = h.ex("BEGIN; INSERT INTO p VALUES(1); INSERT INTO c VALUES(1);"); h.oi("begin.rc", r as i64);
    let r = h.ex("INSERT INTO c VALUES(99);"); h.oi("orphan.rc", r as i64);
    let r = h.ex("ROLLBACK;"); h.oi("rollback.rc", r as i64);
    let p = h.q1("SELECT count(*) FROM p"); h.oi("p", p);
    let c = h.q1("SELECT count(*) FROM c"); h.oi("c", c);
    h.check();
}}

#[test]
fn orrollback_c004() { unsafe {
    let mut h = H::open("engine-orrollback-001-C004", ":memory:");
    h.ex("CREATE TABLE q(a INTEGER UNIQUE); INSERT INTO q VALUES(1);");
    let r = h.ex("INSERT OR ROLLBACK INTO q VALUES(1);"); h.oi("dup.rc", r as i64);
    let a = ac(h.db); h.oi("autocommit", a);
    let c = h.q1("SELECT count(*) FROM q"); h.oi("count", c);
    h.check();
}}

#[test]
fn orrollback_c005() { unsafe {
    let mut h = H::open("engine-orrollback-001-C005", ":memory:");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id)); INSERT INTO p VALUES(1); INSERT INTO c VALUES(1);");
    let r = h.ex("BEGIN; INSERT INTO p VALUES(2);"); h.oi("begin.rc", r as i64);
    let r = h.ex("DELETE OR ROLLBACK FROM p WHERE id=1;"); h.oi("del.rc", r as i64);
    let a = ac(h.db); h.oi("autocommit", a);
    let r = h.ex("COMMIT;"); h.oi("commit.rc", r as i64);
    let p = h.q1("SELECT count(*) FROM p"); h.oi("p", p);
    h.check();
}}

#[test]
fn orrollback_c006() { unsafe {
    let mut h = H::open("engine-orrollback-001-C006", ":memory:");
    let a = ac(h.db); h.oi("ac0", a);
    h.ex("BEGIN;");
    let a = ac(h.db); h.oi("ac1", a);
    h.ex("COMMIT;");
    let a = ac(h.db); h.oi("ac2", a);
    h.check();
}}

fn file_case(cid: &'static str, w: &str, extra_setup: Option<&str>, q: &str, label: &str) {
    let path = format!("/tmp/txn25r_{}_{}.db", std::process::id(), cid.rsplit('-').next().unwrap());
    let _ = std::fs::remove_file(&path);
    let mut h = H::open(cid, &path);
    if let Some(s) = extra_setup { h.ex(s); }
    let r = h.ex(w); h.oi("w.rc", r as i64);
    h.reopen(&path);
    let v = h.q1(q); h.oi(label, v);
    h.check();
    let _ = std::fs::remove_file(&path);
}

#[test]
fn txnfile_c001() { file_case("engine-txnfile-001-C001",
    "CREATE TABLE t(a INTEGER); BEGIN; INSERT INTO t VALUES(1); COMMIT;", None,
    "SELECT count(*) FROM t", "count"); }
#[test]
fn txnfile_c002() { file_case("engine-txnfile-001-C002",
    "CREATE TABLE t(a INTEGER); BEGIN; INSERT INTO t VALUES(1); ROLLBACK;", None,
    "SELECT count(*) FROM t", "count"); }
#[test]
fn txnfile_c003() { file_case("engine-txnfile-001-C003",
    "BEGIN; INSERT INTO t VALUES(9);", Some("CREATE TABLE t(a INTEGER);"),
    "SELECT count(*) FROM t", "count"); }
#[test]
fn txnfile_c004() { file_case("engine-txnfile-001-C004",
    "CREATE TABLE t(a INTEGER); BEGIN; INSERT INTO t VALUES(777001); COMMIT;", None,
    "SELECT a FROM t", "v"); }
