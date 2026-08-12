//! Run-43 attached-trigger FIRE replay — mirrors /tmp/atrig_harness.c.
//! Plain language: a trigger is an attached-schema object (CREATE TRIGGER aux.trg);
//! when it fires, unqualified names in its body resolve into that attached schema the
//! way C does — strictly, with no fallback to main.
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
        let mut h = H { cid, lines: Vec::new(), db };
        h.ex("ATTACH ':memory:' AS aux;");
        h } }
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
    fn check(self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-attach33/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}

// ================= A — fire on attached table, unqualified body =================

#[test] fn a001() { let mut h = H::new("engine-attach33-001-C001");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.exr("mktrig", "CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.exr("fire", "INSERT INTO aux.t VALUES(41);");
    h.rows("auxlog", "SELECT v FROM aux.log");
    h.check(); }

#[test] fn a002() { let mut h = H::new("engine-attach33-001-C002");
    h.ex("CREATE TABLE main.log(v); CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.ex("INSERT INTO aux.t VALUES(7);");
    h.rows("auxlog", "SELECT count(*) FROM aux.log");
    h.rows("mainlog", "SELECT count(*) FROM main.log");
    h.check(); }

#[test] fn a003() { let mut h = H::new("engine-attach33-001-C003");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.exr("mktrig", "CREATE TRIGGER trg AFTER INSERT ON aux.t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.exr("fire", "INSERT INTO aux.t VALUES(9);");
    h.rows("auxlog", "SELECT count(*) FROM aux.log");
    h.check(); }

#[test] fn a004() { let mut h = H::new("engine-attach33-001-C004");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a * 10); END;");
    h.ex("INSERT INTO aux.t VALUES(1),(2),(3);");
    h.rows("logs", "SELECT v FROM aux.log ORDER BY v");
    h.check(); }

#[test] fn a005() { let mut h = H::new("engine-attach33-001-C005");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.rows("aux_master", "SELECT type, name FROM aux.sqlite_master ORDER BY type, name");
    h.rows("main_master", "SELECT count(*) FROM main.sqlite_master");
    h.check(); }

#[test] fn a006() { let mut h = H::new("engine-attach33-001-C006");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg AFTER INSERT ON t WHEN new.a > 5 BEGIN INSERT INTO log VALUES(new.a); END;");
    h.ex("INSERT INTO aux.t VALUES(3); INSERT INTO aux.t VALUES(8);");
    h.rows("gated", "SELECT v FROM aux.log");
    h.check(); }

#[test] fn a007() { let mut h = H::new("engine-attach33-001-C007");
    h.ex("ATTACH ':memory:' AS a2;");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v); CREATE TABLE a2.t(a); CREATE TABLE a2.log(v);");
    h.ex("CREATE TRIGGER aux.trg1 AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.ex("CREATE TRIGGER a2.trg2 AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a + 100); END;");
    h.ex("INSERT INTO aux.t VALUES(1); INSERT INTO a2.t VALUES(2);");
    h.rows("auxlog", "SELECT v FROM aux.log");
    h.rows("a2log", "SELECT v FROM a2.log");
    h.check(); }

#[test] fn a008() { let mut h = H::new("engine-attach33-001-C008");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.exr("fire_unqual", "INSERT INTO t VALUES(55);");
    h.rows("auxlog", "SELECT v FROM aux.log");
    h.check(); }

// ================= B — resolution / collision / error edges =================

#[test] fn b001() { let mut h = H::new("engine-attach33-002-C001");
    h.ex("CREATE TABLE main.t(a); CREATE TABLE main.log(v); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.ex("INSERT INTO main.t VALUES(4);");
    h.rows("mainlog", "SELECT count(*) FROM main.log");
    h.rows("auxlog", "SELECT count(*) FROM aux.log");
    h.check(); }

#[test] fn b002() { let mut h = H::new("engine-attach33-002-C002");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.exr("xqual", "CREATE TRIGGER aux.bad AFTER INSERT ON t BEGIN INSERT INTO aux.log VALUES(new.a); END;");
    h.check(); }

#[test] fn b003() { let mut h = H::new("engine-attach33-002-C003");
    h.ex("CREATE TABLE m(a); CREATE TABLE mlog(v);");
    h.ex("CREATE TRIGGER trg AFTER INSERT ON m BEGIN INSERT INTO mlog VALUES(new.a); END;");
    h.ex("INSERT INTO m VALUES(6);");
    h.rows("mlog", "SELECT v FROM mlog");
    h.check(); }

#[test] fn b004() { let mut h = H::new("engine-attach33-002-C004");
    h.ex("CREATE TABLE aux.t(a);");
    h.exr("mktrig", "CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO nolog VALUES(new.a); END;");
    h.exr("fire", "INSERT INTO aux.t VALUES(1);");
    h.check(); }

#[test] fn b005() { let mut h = H::new("engine-attach33-002-C005");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE main.log(v);");
    h.exr("mktrig", "CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.exr("fire", "INSERT INTO aux.t VALUES(9);");
    h.rows("mainlog", "SELECT count(*) FROM main.log");
    h.check(); }

#[test] fn b006() { let mut h = H::new("engine-attach33-002-C006");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE main.m(a);");
    h.exr("xon", "CREATE TRIGGER aux.trg AFTER INSERT ON main.m BEGIN INSERT INTO t VALUES(new.a); END;");
    h.check(); }

#[test] fn b007() { let mut h = H::new("engine-attach33-002-C007");
    h.ex("CREATE TABLE aux.t(a);");
    h.exr("xmissing", "CREATE TRIGGER trg AFTER INSERT ON t BEGIN SELECT 1; END;");
    h.check(); }

// ================= C — event kinds + timing on attached tables =================

#[test] fn c001() { let mut h = H::new("engine-attach33-003-C001");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg AFTER UPDATE ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.ex("INSERT INTO aux.t VALUES(1);");
    h.exr("fire", "UPDATE aux.t SET a = 2;");
    h.rows("log", "SELECT v FROM aux.log");
    h.check(); }

#[test] fn c002() { let mut h = H::new("engine-attach33-003-C002");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg AFTER DELETE ON t BEGIN INSERT INTO log VALUES(old.a); END;");
    h.ex("INSERT INTO aux.t VALUES(5),(6);");
    h.exr("fire", "DELETE FROM aux.t WHERE a = 6;");
    h.rows("log", "SELECT v FROM aux.log");
    h.check(); }

#[test] fn c003() { let mut h = H::new("engine-attach33-003-C003");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg BEFORE INSERT ON t BEGIN INSERT INTO log VALUES(new.a - 1); END;");
    h.ex("INSERT INTO aux.t VALUES(10);");
    h.rows("log", "SELECT v FROM aux.log");
    h.rows("t", "SELECT a FROM aux.t");
    h.check(); }

#[test] fn c004() { let mut h = H::new("engine-attach33-003-C004");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg_b BEFORE INSERT ON t BEGIN INSERT INTO log VALUES('before'); END;");
    h.ex("CREATE TRIGGER aux.trg_a AFTER INSERT ON t BEGIN INSERT INTO log VALUES('after'); END;");
    h.ex("INSERT INTO aux.t VALUES(1);");
    h.rows("order", "SELECT v FROM aux.log");
    h.check(); }

// ================= D — DETACH teardown =================

#[test] fn d001() { let mut h = H::new("engine-attach33-004-C001");
    h.ex("CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    h.ex("CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    h.ex("INSERT INTO aux.t VALUES(1);");
    h.ex("DETACH aux;");
    h.exr("gone", "INSERT INTO aux.t VALUES(2);");
    h.ex("ATTACH ':memory:' AS aux;");
    h.rows("fresh", "SELECT count(*) FROM aux.sqlite_master");
    h.check(); }

// ================= anti-cheat =================

#[test] fn anti_cheat_attach33_runtime_fire() { unsafe {
    // runtime schema/table/trigger names and a runtime payload: the fired trigger must
    // write the runtime value into the attached log — impossible for a canned answer.
    let seed = (std::process::id() % 100000) as i64;
    let (sch, t, log, trg) = (format!("s{seed}"), format!("t{seed}"), format!("log{seed}"), format!("trg{seed}"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, &format!("ATTACH ':memory:' AS {sch};"));
    exs(db, &format!("CREATE TABLE {sch}.{t}(a); CREATE TABLE {sch}.{log}(v);"));
    exs(db, &format!("CREATE TRIGGER {sch}.{trg} AFTER INSERT ON {t} BEGIN INSERT INTO {log} VALUES(new.a + 1); END;"));
    exs(db, &format!("INSERT INTO {sch}.{t} VALUES({seed});"));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT v FROM {sch}.{log}")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed + 1, "fired body must write the runtime payload into the attached log");
    assert_eq!(sqlite3_step(st), 101, "exactly one fired row");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_attach33_collision_and_detach() { unsafe {
    // collision: body must NOT silently hit the wrong schema; DETACH leaves no trigger ghost.
    let seed = (std::process::id() % 100000) as i64;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, "ATTACH ':memory:' AS aux;");
    exs(db, "CREATE TABLE main.log(v); CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    exs(db, "CREATE TRIGGER aux.trg AFTER INSERT ON t BEGIN INSERT INTO log VALUES(new.a); END;");
    exs(db, &format!("INSERT INTO aux.t VALUES({seed});"));
    let count = |db: *mut Sqlite3, sql: &str| -> i64 {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        assert_eq!(sqlite3_step(st), 100);
        let v = sqlite3_column_int64(st, 0);
        sqlite3_finalize(st); v
    };
    assert_eq!(count(db, "SELECT count(*) FROM aux.log"), 1, "aux log gets the fired row");
    assert_eq!(count(db, "SELECT count(*) FROM main.log"), 0, "main log must stay untouched");
    exs(db, "DETACH aux;");
    exs(db, "ATTACH ':memory:' AS aux;");
    exs(db, "CREATE TABLE aux.t(a); CREATE TABLE aux.log(v);");
    exs(db, &format!("INSERT INTO aux.t VALUES({seed});"));
    assert_eq!(count(db, "SELECT count(*) FROM aux.log"), 0, "no trigger ghost after DETACH");
    assert_eq!(count(db, "SELECT count(*) FROM main.log"), 0);
    sqlite3_close(db);
} }
