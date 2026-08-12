//! Run-26 bespoke replay: CHECK-on-UPDATE OR-modes + file twins — mirrors
//! /tmp/chk_harness.c, asserted byte-identical against frozen goldens.
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
    fn qall(&mut self, sql: &str, label: &str) {
        unsafe {
            let mut st: *mut Sqlite3Stmt = ptr::null_mut();
            let c = CString::new(sql).unwrap();
            if sqlite3_prepare_v2(self.db, c.as_ptr(), -1, &mut st, ptr::null_mut()) == 0 {
                while sqlite3_step(st) == 100 {
                    let v = sqlite3_column_int64(st, 0);
                    self.lines.push(format!("OBS {} {} {}", self.cid, label, v));
                }
            }
            sqlite3_finalize(st);
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
fn c001_row_unchanged() {
    let mut h = H::open("engine-checkupd-002-C001", ":memory:");
    h.ex("CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5);");
    let r = h.ex("UPDATE c SET a = -1;"); h.oi("upd.rc", r as i64);
    let a = h.q1("SELECT a FROM c"); h.oi("a", a);
    h.check();
}

#[test]
fn c002_or_abort_txn() { unsafe {
    let mut h = H::open("engine-checkupd-002-C002", ":memory:");
    h.ex("CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5);");
    let r = h.ex("BEGIN; INSERT INTO c VALUES(6);"); h.oi("begin.rc", r as i64);
    let r = h.ex("UPDATE OR ABORT c SET a = -1 WHERE a = 5;"); h.oi("upd.rc", r as i64);
    let a = ac(h.db); h.oi("autocommit", a);
    let r = h.ex("COMMIT;"); h.oi("commit.rc", r as i64);
    let c = h.q1("SELECT count(*) FROM c"); h.oi("count", c);
    let a5 = h.q1("SELECT a FROM c WHERE a = 5"); h.oi("a5", a5);
    h.check();
}}

#[test]
fn c003_or_rollback_txn() { unsafe {
    let mut h = H::open("engine-checkupd-002-C003", ":memory:");
    h.ex("CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5);");
    let r = h.ex("BEGIN; INSERT INTO c VALUES(6);"); h.oi("begin.rc", r as i64);
    let r = h.ex("UPDATE OR ROLLBACK c SET a = -1 WHERE a = 5;"); h.oi("upd.rc", r as i64);
    let a = ac(h.db); h.oi("autocommit", a);
    let r = h.ex("COMMIT;"); h.oi("commit.rc", r as i64);
    let c = h.q1("SELECT count(*) FROM c"); h.oi("count", c);
    h.check();
}}

#[test]
fn c004_or_fail_keeps_prefix() {
    let mut h = H::open("engine-checkupd-002-C004", ":memory:");
    h.ex("CREATE TABLE f(a INTEGER CHECK(a < 12)); INSERT INTO f VALUES(1),(9);");
    let r = h.ex("UPDATE OR FAIL f SET a = a + 5;"); h.oi("upd.rc", r as i64);
    h.qall("SELECT a FROM f ORDER BY a", "a");
    h.check();
}

#[test]
fn c005_abort_undoes_statement() {
    let mut h = H::open("engine-checkupd-002-C005", ":memory:");
    h.ex("CREATE TABLE f(a INTEGER CHECK(a < 12)); INSERT INTO f VALUES(1),(9);");
    let r = h.ex("UPDATE f SET a = a + 5;"); h.oi("upd.rc", r as i64);
    h.qall("SELECT a FROM f ORDER BY a", "a");
    h.check();
}

#[test]
fn f001_pass_durable() {
    let path = format!("/tmp/chk26r_{}_1.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    let mut h = H::open("engine-checkupd-003-C001", &path);
    let r = h.ex("CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5); UPDATE c SET a = 7;");
    h.oi("w.rc", r as i64);
    h.reopen(&path);
    let a = h.q1("SELECT a FROM c"); h.oi("a", a);
    h.check();
    let _ = std::fs::remove_file(&path);
}

#[test]
fn f002_fail_not_applied() {
    let path = format!("/tmp/chk26r_{}_2.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    let mut h = H::open("engine-checkupd-003-C002", &path);
    h.ex("CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5);");
    let r = h.ex("UPDATE c SET a = -1;"); h.oi("upd.rc", r as i64);
    h.reopen(&path);
    let a = h.q1("SELECT a FROM c"); h.oi("a", a);
    h.check();
    let _ = std::fs::remove_file(&path);
}
