//! Run-37 ANALYZE replay — mirrors /tmp/an_harness.c, asserted byte-identical
//! against the frozen C goldens. Plain language: run ANALYZE, and real table and
//! index scans write row-count / selectivity numbers into sqlite_stat1.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::process::Command;
use std::ptr;

fn pin_cli() -> String { std::env::var("SQLITE_PIN_BIN").unwrap_or_else(|_| "/tmp/sqlite-build/sqlite3".into()) }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3, path: String }
impl H {
    fn open(cid: &'static str, path: &str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db, path: path.into() } } }
    fn new(cid: &'static str) -> H { H::open(cid, ":memory:") }
    fn fresh(cid: &'static str, path: &str) -> H {
        for sfx in ["", "-wal", "-shm", "-journal"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
        std::fs::create_dir_all("/tmp/eftest").unwrap();
        H::open(cid, path) }
    fn reopen(&mut self) { unsafe {
        sqlite3_close(self.db);
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(self.path.clone()).unwrap().as_ptr(), &mut db);
        self.db = db; } }
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
        if rc != 0 {
            let e = CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy().into_owned();
            self.lines.push(format!("OBS {} {} prep.rc={} err={}", self.cid, label, rc, e));
            return;
        }
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
    fn stat1(&mut self, label: &str) {
        self.rows(label, "SELECT tbl, ifnull(idx,'~'), ifnull(stat,'~') FROM sqlite_stat1 ORDER BY tbl, idx");
    }
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

#[test] fn c001() { let mut h = H::new("engine-analyze-001-C001");
    h.ex("CREATE TABLE e(a);");
    h.exr("analyze", "ANALYZE;");
    h.rows("exists", "SELECT count(*) FROM sqlite_master WHERE name='sqlite_stat1'");
    h.rows("cnt", "SELECT count(*) FROM sqlite_stat1");
    h.check(); }

#[test] fn c002() { let mut h = H::new("engine-analyze-001-C002");
    h.ex("CREATE TABLE u(z); INSERT INTO u VALUES(1),(2),(3);");
    h.exr("analyze", "ANALYZE;");
    h.stat1("stat1"); h.check(); }

#[test] fn c003() { let mut h = H::new("engine-analyze-001-C003");
    h.ex("CREATE TABLE t(a INT, b TEXT); INSERT INTO t VALUES(1,'x'),(2,'y'),(3,'x');CREATE INDEX ib ON t(b);");
    h.exr("analyze", "ANALYZE;");
    h.stat1("stat1"); h.check(); }

#[test] fn c004() { let mut h = H::new("engine-analyze-001-C004");
    h.ex("CREATE TABLE t(a INT, b TEXT); INSERT INTO t VALUES(1,'x'),(2,'y'),(3,'x'),(4,'x');CREATE UNIQUE INDEX ia ON t(a); CREATE INDEX ib ON t(b);");
    h.exr("analyze", "ANALYZE;");
    h.stat1("stat1"); h.check(); }

#[test] fn c005() { let mut h = H::new("engine-analyze-001-C005");
    h.ex("CREATE TABLE m(g TEXT, v INT);INSERT INTO m VALUES('a',1),('a',2),('a',2),('b',1),('b',1),('b',1);CREATE INDEX igv ON m(g, v);");
    h.exr("analyze", "ANALYZE;");
    h.stat1("stat1"); h.check(); }

#[test] fn c006() { let mut h = H::new("engine-analyze-001-C006");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2);CREATE TABLE u(z); INSERT INTO u VALUES(9);");
    h.exr("analyze", "ANALYZE t;");
    h.stat1("stat1"); h.check(); }

#[test] fn c007() { let mut h = H::new("engine-analyze-001-C007");
    h.ex("CREATE TABLE t(a INT, b TEXT); INSERT INTO t VALUES(1,'x'),(2,'y');CREATE INDEX ia ON t(a); CREATE INDEX ib ON t(b);CREATE TABLE u(z); INSERT INTO u VALUES(1);");
    h.exr("analyze", "ANALYZE ia;");
    h.stat1("stat1"); h.check(); }

#[test] fn c008() { let mut h = H::new("engine-analyze-001-C008");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2);CREATE TABLE u(z TEXT); INSERT INTO u VALUES('p'),('p'),('q');CREATE INDEX iz ON u(z);");
    h.exr("analyze", "ANALYZE;");
    h.stat1("stat1"); h.check(); }

#[test] fn c009() { let mut h = H::new("engine-analyze-001-C009");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2); CREATE INDEX ia ON t(a);");
    h.ex("ANALYZE;");
    h.stat1("before");
    h.ex("INSERT INTO t VALUES(3),(4),(4); DELETE FROM t WHERE a=1;");
    h.exr("analyze2", "ANALYZE;");
    h.stat1("after");
    h.rows("rowcount", "SELECT count(*) FROM sqlite_stat1");
    h.check(); }

#[test] fn c010() { let mut h = H::new("engine-analyze-001-C010");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(2); CREATE INDEX ia ON t(a);CREATE TABLE u(z); INSERT INTO u VALUES(5);");
    h.ex("ANALYZE;");
    h.stat1("before");
    h.exr("dropidx", "DROP INDEX ia;");
    h.stat1("after_dropidx");
    h.exr("droptbl", "DROP TABLE u;");
    h.stat1("after_droptbl");
    h.check(); }

#[test] fn c011() { let mut h = H::new("engine-analyze-001-C011");
    h.ex("CREATE TABLE w(k TEXT PRIMARY KEY, v INT) WITHOUT ROWID;INSERT INTO w VALUES('a',1),('b',2);");
    h.exr("analyze", "ANALYZE;");
    h.stat1("stat1"); h.check(); }

#[test] fn c012() { let mut h = H::new("engine-analyze-001-C012");
    h.ex("CREATE TABLE p(id INTEGER PRIMARY KEY, v TEXT);INSERT INTO p VALUES(10,'x'),(20,'x'),(30,'y');CREATE INDEX iv ON p(v);");
    h.exr("analyze", "ANALYZE;");
    h.stat1("stat1"); h.check(); }

#[test] fn c013() { let mut h = H::fresh("engine-analyze-001-C013", "/tmp/eftest/rw_an_c13.db");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(3); CREATE INDEX ia ON t(a);");
    h.exr("analyze", "ANALYZE;");
    h.reopen();
    h.stat1("reopen");
    h.rows("integ", "PRAGMA integrity_check");
    h.check(); }

#[test] fn c014() { let mut h = H::new("engine-analyze-001-C014");
    h.ex("CREATE TABLE q(a);");
    for i in 1..=11 { h.ex(&format!("INSERT INTO q VALUES({});", if i <= 10 { i } else { 10 })); }
    h.ex("CREATE INDEX iq ON q(a);");
    h.exr("analyze", "ANALYZE;");
    h.stat1("stat1"); h.check(); }

#[test] fn c015() { let mut h = H::new("engine-analyze-001-C015");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2);");
    h.exr("analyze", "ANALYZE main;");
    h.stat1("stat1"); h.check(); }

#[test] fn d001() { let mut h = H::fresh("engine-analyze-002-C001", "/tmp/eftest/rw_an_d1.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a); INSERT INTO t VALUES(1),(2);CREATE INDEX ia ON t(a);");
    h.exr("analyze", "ANALYZE;");
    h.rows("mode", "PRAGMA journal_mode");
    h.reopen();
    h.stat1("reopen");
    h.check(); }

#[test] fn d002() { let mut h = H::fresh("engine-analyze-002-C002", "/tmp/eftest/rw_an_d2.db");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(3); CREATE INDEX ia ON t(a);DELETE FROM t WHERE a=2;");
    h.ex("ANALYZE;");
    h.stat1("before_vac");
    h.exr("vac", "VACUUM;");
    h.stat1("after_vac");
    h.exr("analyze2", "ANALYZE;");
    h.stat1("after_re");
    h.check(); }

// ---- MANDATORY anti-cheat: runtime table / counts -> real stat1 rows ----
#[test] fn anti_cheat_analyze_runtime() { unsafe {
    let seed = (std::process::id() % 541 + 23) as i64;
    let tname = format!("an{seed}");
    let nrows = (seed % 7 + 4) as i64; // runtime-chosen row count
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, &format!("CREATE TABLE {tname}(v INT);"));
    for i in 0..nrows { exs(db, &format!("INSERT INTO {tname} VALUES({});", i / 2)); } // duplicates
    exs(db, &format!("CREATE INDEX i{tname} ON {tname}(v);"));
    exs(db, "ANALYZE;");
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT tbl, idx, stat FROM sqlite_stat1".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    let tbl = CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy().into_owned();
    let idx = CStr::from_ptr(sqlite3_column_text(st, 1) as *const c_char).to_string_lossy().into_owned();
    let stat = CStr::from_ptr(sqlite3_column_text(st, 2) as *const c_char).to_string_lossy().into_owned();
    sqlite3_finalize(st);
    assert_eq!(tbl, tname, "runtime table name must land in stat1");
    assert_eq!(idx, format!("i{tname}"));
    // real selectivity: nrows rows, ceil(nrows/2) distinct (i/2 duplicates)
    let ndist = (nrows + 1) / 2;
    let mut ival = (nrows + ndist - 1) / ndist;
    if ival == 2 && nrows * 10 <= ndist * 11 { ival = 1; }
    assert_eq!(stat, format!("{nrows} {ival}"), "stat integers must reflect the runtime data");
    sqlite3_close(db);
} }

// ---- MANDATORY C interop: pinned C reads Rust's sqlite_stat1 (and vice versa) ----
#[test] fn rust_analyze_c_read() { unsafe {
    let n = std::process::id() % 100000;
    let path = format!("/tmp/eftest/racr_{n}.db");
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path.clone()).unwrap().as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, "CREATE TABLE f(a INT); INSERT INTO f VALUES(1),(2),(2),(3); CREATE INDEX ifa ON f(a);");
    exs(db, "ANALYZE;");
    sqlite3_close(db);
    let cli = pin_cli();
    let out = Command::new(&cli).arg(&path).arg("SELECT tbl, idx, stat FROM sqlite_stat1;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "f|ifa|4 2",
               "pinned C must read Rust's sqlite_stat1 (stderr: {})", String::from_utf8_lossy(&out.stderr));
    let ic = Command::new(&cli).arg(&path).arg("PRAGMA integrity_check;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&ic.stdout).trim(), "ok");
    // reverse: C ANALYZEs its own file; modern reads the rows
    let path2 = format!("/tmp/eftest/cwan_{n}.db");
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path2}{sfx}")); }
    let st = Command::new(&cli).arg(&path2)
        .arg("CREATE TABLE g(z); INSERT INTO g VALUES(7),(8); CREATE INDEX igz ON g(z); ANALYZE;")
        .status().unwrap();
    assert!(st.success());
    let mut db2: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path2.clone()).unwrap().as_ptr(), &mut db2);
    let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db2, c"SELECT tbl, idx, stat FROM sqlite_stat1".as_ptr(), -1, &mut st2, ptr::null_mut());
    assert_eq!(sqlite3_step(st2), 100);
    let got = format!("{}|{}|{}",
        CStr::from_ptr(sqlite3_column_text(st2, 0) as *const c_char).to_string_lossy(),
        CStr::from_ptr(sqlite3_column_text(st2, 1) as *const c_char).to_string_lossy(),
        CStr::from_ptr(sqlite3_column_text(st2, 2) as *const c_char).to_string_lossy());
    assert_eq!(got, "g|igz|2 1", "modern must read C's sqlite_stat1");
    sqlite3_finalize(st2);
    sqlite3_close(db2);
    for p in [&path, &path2] { for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{p}{sfx}")); } }
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
