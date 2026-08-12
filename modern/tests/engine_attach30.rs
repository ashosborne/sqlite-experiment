//! Run-40 attached-schema ownership replay — mirrors /tmp/attach_harness.c.
//! Plain language: ATTACH opens a real second schema that can own tables; DETACH
//! drops it; qualified names resolve across schemas.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
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
    fn dblist(&mut self, l: &str) { self.rows(l, "SELECT name FROM pragma_database_list ORDER BY seq"); }
    fn reopen_mem(&mut self) { unsafe { sqlite3_close(self.db); let mut db: *mut Sqlite3 = ptr::null_mut(); sqlite3_open(c":memory:".as_ptr(), &mut db); self.db = db; } }
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

#[test] fn a001() { let mut h = H::new("engine-attach30-001-C001");
    h.exr("attach", "ATTACH ':memory:' AS aux;"); h.dblist("dblist"); h.check(); }
#[test] fn a002() { let mut h = H::new("engine-attach30-001-C002");
    h.ex("ATTACH ':memory:' AS aux;");
    h.exr("create", "CREATE TABLE aux.t(a, b);");
    h.exr("ins", "INSERT INTO aux.t VALUES(1,'x'),(2,'y');");
    h.rows("qual", "SELECT a, b FROM aux.t ORDER BY a"); h.check(); }
#[test] fn a003() { let mut h = H::new("engine-attach30-001-C003");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); INSERT INTO aux.t VALUES(7);");
    h.rows("unqual", "SELECT a FROM t");
    h.ex("CREATE TABLE m(z); INSERT INTO m VALUES(9);");
    h.rows("main_unqual", "SELECT z FROM m");
    h.rows("main_qual", "SELECT z FROM main.m"); h.check(); }
#[test] fn a004() { let mut h = H::new("engine-attach30-001-C004");
    h.ex("ATTACH ':memory:' AS aux;");
    h.exr("dup", "ATTACH ':memory:' AS aux;");
    h.exr("resv_main", "ATTACH ':memory:' AS main;");
    h.exr("resv_temp", "ATTACH ':memory:' AS temp;"); h.check(); }
#[test] fn a005() { let mut h = H::new("engine-attach30-001-C005");
    h.ex("ATTACH ':memory:' AS a1; ATTACH ':memory:' AS a2; CREATE TABLE a1.x(v); CREATE TABLE a2.y(w);");
    h.ex("INSERT INTO a1.x VALUES(11); INSERT INTO a2.y VALUES(22);");
    h.rows("two", "SELECT (SELECT v FROM a1.x), (SELECT w FROM a2.y)");
    h.dblist("dblist"); h.check(); }
#[test] fn a006() { let mut h = H::new("engine-attach30-001-C006");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); CREATE TABLE main.t(a);");
    h.ex("INSERT INTO aux.t VALUES(1); INSERT INTO main.t VALUES(2);");
    h.rows("aux", "SELECT a FROM aux.t");
    h.rows("main", "SELECT a FROM main.t");
    h.rows("bare", "SELECT a FROM t"); h.check(); }
#[test] fn a007() { let path = "/tmp/eftest/rust_aux_d.db";
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
    std::fs::create_dir_all("/tmp/eftest").unwrap();
    let mut h = H::new("engine-attach30-001-C007");
    h.ex(&format!("ATTACH '{path}' AS aux; CREATE TABLE aux.t(a,b); INSERT INTO aux.t VALUES(5,'q');"));
    h.reopen_mem();
    h.ex(&format!("ATTACH '{path}' AS aux;"));
    h.rows("reopen", "SELECT a,b FROM aux.t"); h.check(); }
#[test] fn a008() { let mut h = H::new("engine-attach30-001-C008");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a INT, b TEXT); INSERT INTO aux.t VALUES(1,'x'),(2,'y'),(3,'z');");
    h.exr("upd", "UPDATE aux.t SET b='Q' WHERE a=2;");
    h.exr("del", "DELETE FROM aux.t WHERE a=3;");
    h.rows("after", "SELECT a,b FROM aux.t ORDER BY a"); h.check(); }

#[test] fn b001() { let mut h = H::new("engine-attach30-002-C001");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); INSERT INTO aux.t VALUES(1);");
    h.exr("detach", "DETACH aux;");
    h.dblist("dblist");
    h.rows("gone", "SELECT a FROM aux.t"); h.check(); }
#[test] fn b002() { let mut h = H::new("engine-attach30-002-C002");
    h.exr("detach_main", "DETACH main;");
    h.ex("ATTACH ':memory:' AS aux;");
    h.exr("detach_missing", "DETACH nope;"); h.check(); }
#[test] fn b003() { let mut h = H::new("engine-attach30-002-C003");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); INSERT INTO aux.t VALUES(1);");
    h.ex("DETACH aux;");
    h.ex("ATTACH ':memory:' AS aux;");
    h.exr("recreate", "CREATE TABLE aux.t(b);");
    h.exr("ins", "INSERT INTO aux.t VALUES(2);");
    h.rows("val", "SELECT b FROM aux.t"); h.check(); }
#[test] fn b004() { let mut h = H::new("engine-attach30-002-C004");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); INSERT INTO aux.t VALUES(1);");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(99);");
    h.ex("DETACH aux;");
    h.rows("bare", "SELECT a FROM t"); h.check(); }

#[test] fn c001() { let mut h = H::new("engine-attach30-003-C001");
    h.ex("ATTACH ':memory:' AS aux3; CREATE TABLE main.mm(v); CREATE TABLE aux3.t(a);");
    h.exr("xtrig", "CREATE TRIGGER aux3.tr AFTER INSERT ON aux3.t BEGIN INSERT INTO main.mm VALUES(1); END;");
    h.exr("xview", "CREATE VIEW aux3.vv AS SELECT * FROM main.mm;"); h.check(); }
#[test] fn c002() { let mut h = H::new("engine-attach30-003-C002");
    h.ex("ATTACH ':memory:' AS aux3; CREATE TABLE aux3.t(a);");
    h.exr("qual_same", "CREATE TRIGGER aux3.tr AFTER INSERT ON aux3.t BEGIN INSERT INTO aux3.t VALUES(2); END;"); h.check(); }

#[test] fn anti_cheat_attach30() { unsafe {
    let seed = std::process::id() % 100000;
    let (sch, tbl) = (format!("aux{seed}"), format!("t{seed}"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, &format!("ATTACH ':memory:' AS {sch};"));
    exs(db, &format!("CREATE TABLE {sch}.{tbl}(v INT);"));
    exs(db, &format!("INSERT INTO {sch}.{tbl} VALUES({});", seed as i64 + 3));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT v FROM {sch}.{tbl}")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed as i64 + 3);
    sqlite3_finalize(st);
    // DETACH then the qualified name is gone
    exs(db, &format!("DETACH {sch};"));
    let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare_v2(db, CString::new(format!("SELECT v FROM {sch}.{tbl}")).unwrap().as_ptr(), -1, &mut st2, ptr::null_mut());
    assert_eq!(rc, 1, "detached schema table must be gone");
    sqlite3_finalize(st2);
    sqlite3_close(db);
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
