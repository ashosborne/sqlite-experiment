//! Run-39 none-batch replay — qualified-table-name-in-trigger rejection
//! (attach-detach-003 core rule), mirroring /tmp/none29_harness.c.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
        sqlite3_exec(db, c"CREATE TABLE t(a); CREATE TABLE u(v);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
        H { cid, lines: Vec::new(), db } } }
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

#[test] fn c001() { let mut h = H::new("engine-none29-001-C001");
    h.exr("qual_insert", "CREATE TRIGGER tr AFTER INSERT ON t BEGIN INSERT INTO main.u VALUES(1); END;");
    h.rows("not_made", "SELECT count(*) FROM sqlite_master WHERE type='trigger'"); h.check(); }
#[test] fn c002() { let mut h = H::new("engine-none29-001-C002");
    h.exr("qual_update", "CREATE TRIGGER tr AFTER INSERT ON t BEGIN UPDATE main.u SET v=1; END;");
    h.exr("qual_delete", "CREATE TRIGGER tr2 AFTER INSERT ON t BEGIN DELETE FROM main.u; END;"); h.check(); }
#[test] fn c003() { let mut h = H::new("engine-none29-001-C003");
    h.exr("unqual", "CREATE TRIGGER tr AFTER INSERT ON t BEGIN INSERT INTO u VALUES(9); END;");
    h.exr("fire", "INSERT INTO t VALUES(1);");
    h.rows("cnt", "SELECT count(*), v FROM u"); h.check(); }
#[test] fn c005() { let mut h = H::new("engine-none29-001-C005");
    h.exr("before_qual", "CREATE TRIGGER tr BEFORE UPDATE ON t BEGIN INSERT INTO main.u VALUES(1); END;"); h.check(); }
#[test] fn c006() { let mut h = H::new("engine-none29-001-C006");
    h.exr("multi", "CREATE TRIGGER tr AFTER INSERT ON t BEGIN INSERT INTO u VALUES(1); INSERT INTO main.u VALUES(2); END;");
    h.rows("not_made", "SELECT count(*) FROM sqlite_master WHERE type='trigger'"); h.check(); }
#[test] fn c007() { let mut h = H::new("engine-none29-001-C007");
    h.exr("temp_qual", "CREATE TEMP TRIGGER tr AFTER INSERT ON t BEGIN DELETE FROM main.u; END;"); h.check(); }

#[test] fn anti_cheat_none29() { unsafe {
    // runtime table names: a qualified DML target still rejected, unqualified fires
    let seed = std::process::id() % 100000;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    let (a, b) = (format!("t{seed}"), format!("u{seed}"));
    exs(db, &format!("CREATE TABLE {a}(x); CREATE TABLE {b}(y);"));
    let mut em: *mut c_char = ptr::null_mut();
    let rc = sqlite3_exec(db, CString::new(format!("CREATE TRIGGER g AFTER INSERT ON {a} BEGIN INSERT INTO main.{b} VALUES(1); END;")).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
    assert_eq!(rc, 1, "qualified target must be rejected");
    assert_eq!(CStr::from_ptr(em).to_string_lossy(), "qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers");
    sqlite3_free(em as *mut c_void);
    // unqualified fires
    exs(db, &format!("CREATE TRIGGER g2 AFTER INSERT ON {a} BEGIN INSERT INTO {b} VALUES(99); END;"));
    exs(db, &format!("INSERT INTO {a} VALUES(1);"));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT y FROM {b}")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    assert_eq!(sqlite3_column_int(st, 0), 99);
    sqlite3_finalize(st); sqlite3_close(db);
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
