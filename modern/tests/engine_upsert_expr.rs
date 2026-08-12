//! Run-31 upsert expression conflict-target replay — mirrors /tmp/upx_harness.c,
//! asserted byte-identical against the frozen C goldens.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;

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
    fn exr(&mut self, label: &str, s: &str) { unsafe {
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let e = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={}", self.cid, label, rc, e));
        sqlite3_free(em as *mut std::os::raw::c_void);
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

#[test] fn c001() { let mut h = H::new("engine-upsert-expr-001-C001");
    h.ex("CREATE TABLE t(c TEXT); CREATE UNIQUE INDEX i1 ON t(lower(c)); INSERT INTO t VALUES('Alpha');");
    h.exr("upsert", "INSERT INTO t VALUES('ALPHA') ON CONFLICT(lower(c)) DO NOTHING;");
    h.rows("after", "SELECT count(*), c FROM t");
    h.check(); }

#[test] fn c002() { let mut h = H::new("engine-upsert-expr-001-C002");
    h.ex("CREATE TABLE t(c TEXT, n INT); CREATE UNIQUE INDEX i1 ON t(lower(c)); INSERT INTO t VALUES('Alpha',1);");
    h.exr("upsert", "INSERT INTO t VALUES('ALPHA',9) ON CONFLICT(lower(c)) DO UPDATE SET n=excluded.n;");
    h.rows("after", "SELECT c, n FROM t");
    h.check(); }

#[test] fn c003() { let mut h = H::new("engine-upsert-expr-001-C003");
    h.ex("CREATE TABLE t(a INT, b INT, tag TEXT); CREATE UNIQUE INDEX i1 ON t(a+b); INSERT INTO t VALUES(1,2,'first');");
    h.exr("upsert", "INSERT INTO t VALUES(0,3,'second') ON CONFLICT(a+b) DO UPDATE SET tag=excluded.tag;");
    h.rows("after", "SELECT a, b, tag FROM t");
    h.check(); }

#[test] fn c004() { let mut h = H::new("engine-upsert-expr-001-C004");
    h.ex("CREATE TABLE t(a TEXT, b INT); CREATE UNIQUE INDEX i1 ON t(lower(a), b); INSERT INTO t VALUES('Kilo',5);");
    h.exr("dup", "INSERT INTO t VALUES('KILO',5) ON CONFLICT(lower(a), b) DO NOTHING;");
    h.exr("nodup", "INSERT INTO t VALUES('KILO',6) ON CONFLICT(lower(a), b) DO NOTHING;");
    h.rows("after", "SELECT a, b FROM t ORDER BY b");
    h.check(); }

#[test] fn c005() { let mut h = H::new("engine-upsert-expr-001-C005");
    h.ex("CREATE TABLE t(c TEXT, d TEXT); CREATE UNIQUE INDEX i1 ON t(lower(c));");
    h.exr("wrong_expr", "INSERT INTO t VALUES('x','y') ON CONFLICT(upper(c)) DO NOTHING;");
    h.exr("plain_col", "INSERT INTO t VALUES('x','y') ON CONFLICT(d) DO NOTHING;");
    h.check(); }

#[test] fn c006() { let mut h = H::new("engine-upsert-expr-001-C006");
    h.ex("CREATE TABLE t(c TEXT); CREATE UNIQUE INDEX i1 ON t(lower(c)); INSERT INTO t VALUES('Beta');");
    h.exr("catchall", "INSERT INTO t VALUES('BETA') ON CONFLICT DO NOTHING;");
    h.rows("after", "SELECT count(*) FROM t");
    h.check(); }

#[test] fn c007() { let path = "/tmp/eftest/rust_upx1.db";
    let _ = std::fs::remove_file(path);
    std::fs::create_dir_all("/tmp/eftest").unwrap();
    let mut h = H::open("engine-upsert-expr-001-C007", path);
    h.ex("CREATE TABLE t(c TEXT, n INT); CREATE UNIQUE INDEX i1 ON t(lower(c)); INSERT INTO t VALUES('Gamma',1);");
    h.reopen(path);
    h.exr("upsert", "INSERT INTO t VALUES('GAMMA',7) ON CONFLICT(lower(c)) DO UPDATE SET n=excluded.n;");
    h.rows("after", "SELECT c, n FROM t");
    h.check(); }

#[test] fn c008() { let mut h = H::new("engine-upsert-expr-001-C008");
    h.ex("CREATE TABLE t(c TEXT, n INT); CREATE UNIQUE INDEX i1 ON t(lower(c)); INSERT INTO t VALUES('Delta',1);");
    h.exr("ignore", "INSERT OR IGNORE INTO t VALUES('DELTA',2);");
    h.rows("mid", "SELECT c, n FROM t");
    h.exr("replace", "INSERT OR REPLACE INTO t VALUES('DELTA',3);");
    h.rows("after", "SELECT c, n FROM t");
    h.check(); }

#[test] fn c009() { let mut h = H::new("engine-upsert-expr-001-C009");
    h.ex("CREATE TABLE t(c TEXT); CREATE UNIQUE INDEX i1 ON t( LOWER(c) ); INSERT INTO t VALUES('Echo');");
    h.exr("target_lc", "INSERT INTO t VALUES('ECHO') ON CONFLICT(lower(c)) DO NOTHING;");
    h.exr("target_sp", "INSERT INTO t VALUES('ECHO') ON CONFLICT( lower( c ) ) DO NOTHING;");
    h.rows("after", "SELECT count(*) FROM t");
    h.check(); }

#[test] fn c010() { let mut h = H::new("engine-upsert-expr-001-C010");
    h.ex("CREATE TABLE t(c TEXT UNIQUE, d TEXT); CREATE UNIQUE INDEX i1 ON t(lower(d)); INSERT INTO t VALUES('u1','v1');");
    h.exr("other", "INSERT INTO t VALUES('u1','zz') ON CONFLICT(lower(d)) DO NOTHING;");
    h.exr("targeted", "INSERT INTO t VALUES('u9','V1') ON CONFLICT(lower(d)) DO NOTHING;");
    h.rows("after", "SELECT count(*) FROM t");
    h.check(); }

#[test] fn r001() { let mut h = H::new("engine-upsert-expr-002-C001");
    h.ex("CREATE TABLE t(c TEXT UNIQUE, n INT); INSERT INTO t VALUES('k',1);");
    h.exr("upsert", "INSERT INTO t VALUES('k',5) ON CONFLICT(c) DO UPDATE SET n=excluded.n;");
    h.rows("after", "SELECT c, n FROM t");
    h.check(); }

#[test] fn r002() { let mut h = H::new("engine-upsert-expr-002-C002");
    h.ex("CREATE TABLE t(a INT, b INT, n INT); CREATE UNIQUE INDEX i1 ON t(a,b); INSERT INTO t VALUES(1,2,0);");
    h.exr("upsert", "INSERT INTO t VALUES(1,2,9) ON CONFLICT(a,b) DO UPDATE SET n=excluded.n;");
    h.rows("after", "SELECT a, b, n FROM t");
    h.check(); }

#[test] fn r003() { let mut h = H::new("engine-upsert-expr-002-C003");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, n INT); INSERT INTO t VALUES(1,10);");
    h.exr("upsert", "INSERT INTO t VALUES(1,20) ON CONFLICT(id) DO UPDATE SET n=excluded.n;");
    h.rows("after", "SELECT id, n FROM t");
    h.check(); }

#[test] fn r004() { let mut h = H::new("engine-upsert-expr-002-C004");
    h.ex("CREATE TABLE t(c TEXT UNIQUE, n INT); INSERT INTO t VALUES('w',5);");
    h.exr("blocked", "INSERT INTO t VALUES('w',1) ON CONFLICT(c) DO UPDATE SET n=excluded.n WHERE excluded.n > n;");
    h.exr("applied", "INSERT INTO t VALUES('w',9) ON CONFLICT(c) DO UPDATE SET n=excluded.n WHERE excluded.n > n;");
    h.rows("after", "SELECT c, n FROM t");
    h.check(); }

#[test] fn p001() { let mut h = H::new("engine-upsert-expr-003-C001");
    h.ex("CREATE TABLE t(c TEXT, flag INT, n INT); CREATE UNIQUE INDEX i1 ON t(c) WHERE flag=1;INSERT INTO t VALUES('p',1,0);");
    h.exr("match", "INSERT INTO t VALUES('p',1,9) ON CONFLICT(c) WHERE flag=1 DO UPDATE SET n=excluded.n;");
    h.rows("after", "SELECT c, flag, n FROM t");
    h.check(); }

#[test] fn p002() { let mut h = H::new("engine-upsert-expr-003-C002");
    h.ex("CREATE TABLE t(c TEXT, flag INT); CREATE UNIQUE INDEX i1 ON t(c) WHERE flag=1;");
    h.exr("no_where", "INSERT INTO t VALUES('q',1) ON CONFLICT(c) DO NOTHING;");
    h.exr("wrong_where", "INSERT INTO t VALUES('q',1) ON CONFLICT(c) WHERE flag=2 DO NOTHING;");
    h.check(); }

// ---- run-31 mandatory anti-cheat (runtime values absent from every golden) ----

#[test] fn anti_cheat_upsert_expr_runtime() {
    let mut h = H::new("x");
    let seed = (std::process::id() % 887 + 11) as i64;
    h.ex("CREATE TABLE t(c TEXT, n INT); CREATE UNIQUE INDEX i1 ON t(lower(c));");
    h.ex(&format!("INSERT INTO t VALUES('Seed{seed}', {seed});"));
    h.exr("up", &format!("INSERT INTO t VALUES('SEED{seed}', {}) ON CONFLICT(lower(c)) DO UPDATE SET n=excluded.n;", seed * 2));
    assert_eq!(h.lines.pop().unwrap(), "OBS x up rc=0 err=-");
    h.rows("q", &format!("SELECT c, n FROM t"));
    assert_eq!(h.lines.pop().unwrap(), format!("OBS x q Seed{seed},{}", seed * 2));
    unsafe { sqlite3_close(h.db); }
}

#[test] fn anti_cheat_upsert_expr_mismatch() {
    let mut h = H::new("x");
    h.ex("CREATE TABLE t(c TEXT); CREATE UNIQUE INDEX i1 ON t(lower(c));");
    let bad = format!("rtcol{}", std::process::id() % 1000);
    h.ex(&format!("ALTER TABLE t ADD COLUMN {bad};"));
    h.exr("bad", &format!("INSERT INTO t(c) VALUES('z') ON CONFLICT({bad}) DO NOTHING;"));
    let line = h.lines.pop().unwrap();
    assert!(line.contains("rc=1") &&
            line.contains("ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint"),
            "got: {line}");
    unsafe { sqlite3_close(h.db); }
}

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
