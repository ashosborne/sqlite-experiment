//! Run-46 partial-to-full harvest replay — mirrors /tmp/h36a.c (wave 1) + /tmp/h36b.c (wave 2).
//! Each batch closes a named residual on a Partial estate card; pins cover the former hole.
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
    fn open_path(cid: &'static str, path: &str) -> H { unsafe {
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(format!("{path}-journal"));
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
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
    fn oi(&mut self, label: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, label, v)); }
    fn prep(&mut self, sql: &str) -> *mut Sqlite3Stmt { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        st
    } }
    fn check(self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-harvest36/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
fn complete(s: &str) -> i64 { unsafe { sqlite3_complete(CString::new(s).unwrap().as_ptr()) as i64 } }

// ================= 001: sqlite3_complete audit (tokenizer-002) =================

#[test] fn h001_c001() { let mut h = H::new("engine-harvest36-001-C001");
    h.oi("cmt_eats_semi", complete("SELECT 1 -- done;\n"));
    h.oi("cmt_then_semi", complete("SELECT 1 -- x\n;"));
    h.oi("block_cmt_semi", complete("/* ; */ SELECT 1;"));
    h.oi("open_string", complete("SELECT ';"));
    h.check(); }

#[test] fn h001_c002() { let mut h = H::new("engine-harvest36-001-C002");
    h.oi("quoted_end_in_trigger", complete("CREATE TRIGGER t AFTER INSERT ON x BEGIN SELECT 'END'; END;"));
    h.oi("nested_case_end", complete("CREATE TRIGGER t AFTER INSERT ON x BEGIN SELECT CASE WHEN 1 THEN 2 END; END;"));
    h.oi("bare_semi", complete(";"));
    h.oi("whitespace_only", complete("   "));
    h.check(); }

// ================= 002: quoted reserved table names (parser-grammar-002) =================

#[test] fn h002_c001() { let mut h = H::new("engine-harvest36-002-C001");
    h.exr("create", "CREATE TABLE \"select\"(x, y);");
    h.exr("insert", "INSERT INTO \"select\" VALUES(1,'a'),(2,'b');");
    h.rows("select", "SELECT x, y FROM \"select\" ORDER BY x");
    h.check(); }

#[test] fn h002_c002() { let mut h = H::new("engine-harvest36-002-C002");
    h.ex("CREATE TABLE \"select\"(x, y); INSERT INTO \"select\" VALUES(1,'a'),(2,'b');");
    h.exr("update", "UPDATE \"select\" SET y = 'z' WHERE x = 1;");
    h.rows("after_upd", "SELECT y FROM \"select\" WHERE x = 1");
    h.exr("delete", "DELETE FROM \"select\" WHERE x = 2;");
    h.rows("count", "SELECT count(*) FROM \"select\"");
    h.check(); }

#[test] fn h002_c003() { let mut h = H::new("engine-harvest36-002-C003");
    h.ex("CREATE TABLE \"select\"(x); INSERT INTO \"select\" VALUES(1); CREATE TABLE \"order\"(a); INSERT INTO \"order\" VALUES(7);");
    h.rows("from_order", "SELECT a FROM \"order\"");
    h.rows("qualified", "SELECT \"select\".x FROM \"select\"");
    h.rows("master", "SELECT name FROM sqlite_master ORDER BY name");
    h.exr("drop", "DROP TABLE \"select\";");
    h.rows("master2", "SELECT count(*) FROM sqlite_master");
    h.check(); }

#[test] fn h002_c004() { let mut h = H::new("engine-harvest36-002-C004");
    h.exr("brackets", "CREATE TABLE [where](w);");
    h.exr("ins", "INSERT INTO [where] VALUES(3);");
    h.rows("from_brackets", "SELECT w FROM [where]");
    h.rows("from_backtick", "SELECT w FROM `where`");
    h.rows("from_dquote", "SELECT w FROM \"where\"");
    h.check(); }

// ================= 003: locked DETACH (attach-detach-002) =================

#[test] fn h003_c001() { let mut h = H::new("engine-harvest36-003-C001");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); INSERT INTO aux.t VALUES(1),(2);");
    let st = h.prep("SELECT a FROM aux.t");
    unsafe { sqlite3_step(st); }
    h.exr("detach_locked", "DETACH aux;");
    unsafe { sqlite3_finalize(st); }
    h.exr("detach_after_fin", "DETACH aux;");
    h.check(); }

#[test] fn h003_c002() { let mut h = H::new("engine-harvest36-003-C002");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); CREATE TABLE m(z); INSERT INTO m VALUES(9);");
    let st = h.prep("SELECT z FROM m");
    unsafe { sqlite3_step(st); }
    h.exr("detach_main_stmt_ok", "DETACH aux;");
    unsafe { sqlite3_finalize(st); }
    h.check(); }

// ================= 004: json_valid flags (json-funcs-003) =================

#[test] fn h004_c001() { let mut h = H::new("engine-harvest36-004-C001");
    h.rows("default_eq_f1", "SELECT json_valid('{\"a\":1}'), json_valid('{a:1}'), json_valid('junk'), json_valid('{\"a\":1}',1), json_valid('{a:1}',1)");
    h.check(); }

#[test] fn h004_c002() { let mut h = H::new("engine-harvest36-004-C002");
    h.rows("json5_accepts", "SELECT json_valid('{a:1}',2), json_valid('[1,2,]',2), json_valid('{''a'':1}',2), json_valid('/*c*/1',2), json_valid('0x1A',2), json_valid('+Infinity',2), json_valid('.5',2)");
    h.rows("strict_rejects", "SELECT json_valid('[1,2,]',1), json_valid('0x1A',1), json_valid('.5',1), json_valid('junk',2)");
    h.check(); }

#[test] fn h004_c003() { let mut h = H::new("engine-harvest36-004-C003");
    h.rows("f6_union", "SELECT json_valid('{a:1}',6), json_valid('{\"a\":1}',6), json_valid('junk',6)");
    h.rows("nulls", "SELECT json_valid(NULL), json_valid(NULL,2)");
    h.rows("numeric_args", "SELECT json_valid(7), json_valid(7,1), json_valid(7.5,1)");
    h.check(); }

#[test] fn h004_c004() { let mut h = H::new("engine-harvest36-004-C004");
    h.rows("blob_flags", "SELECT json_valid(x'1331',1), json_valid(x'1331',4), json_valid(x'1331',8), json_valid(x'00',4), json_valid(x'00',8), json_valid(x'',4)");
    h.rows("text_with_blob_flags", "SELECT json_valid('{\"a\":1}',8), json_valid('{\"a\":1}',4)");
    h.check(); }

#[test] fn h004_c005() { let mut h = H::new("engine-harvest36-004-C005");
    h.exr("flag0", "SELECT json_valid('{\"a\":1}',0);");
    h.exr("flag16", "SELECT json_valid('x',16);");
    h.check(); }

// ================= 005: FK drop-order edges (foreign-keys-003) =================

#[test] fn h005_c001() { let mut h = H::new("engine-harvest36-005-C001");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id));");
    h.ex("INSERT INTO p VALUES(1); INSERT INTO c VALUES(1);");
    h.exr("drop_parent_blocked", "DROP TABLE p;");
    h.exr("drop_child_first", "DROP TABLE c;");
    h.exr("then_parent_ok", "DROP TABLE p;");
    h.check(); }

#[test] fn h005_c002() { let mut h = H::new("engine-harvest36-005-C002");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id));");
    h.ex("INSERT INTO p VALUES(1); INSERT INTO c VALUES(NULL);");
    h.exr("null_children_ok", "DROP TABLE p;");
    h.check(); }

#[test] fn h005_c003() { let mut h = H::new("engine-harvest36-005-C003");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id) DEFERRABLE INITIALLY DEFERRED);");
    h.ex("INSERT INTO p VALUES(1); INSERT INTO c VALUES(1);");
    h.exr("deferred_drop_ok", "BEGIN; DROP TABLE p;");
    h.exr("commit_blocked", "COMMIT;");
    h.exr("rollback_ok", "ROLLBACK;");
    h.rows("parent_restored", "SELECT count(*) FROM p");
    h.check(); }

#[test] fn h005_c004() { let mut h = H::new("engine-harvest36-005-C004");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id));");
    h.ex("INSERT INTO p VALUES(1); INSERT INTO c VALUES(1);");
    h.exr("both_in_txn", "BEGIN; DROP TABLE c; DROP TABLE p; COMMIT;");
    h.rows("all_gone", "SELECT count(*) FROM sqlite_master");
    h.check(); }

#[test] fn h005_c005() { let mut h = H::new("engine-harvest36-005-C005");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id));");
    h.ex("INSERT INTO p VALUES(1); INSERT INTO c VALUES(1);");
    h.exr("immediate_in_txn_blocked", "BEGIN; DROP TABLE p;");
    h.exr("rb", "ROLLBACK;");
    h.check(); }

// ================= 006: VACUUM extras (vacuum-001 / vacuum-002) =================

#[test] fn h006_c001() { let mut h = H::new("engine-harvest36-006-C001");
    let _ = std::fs::remove_file("/tmp/h36u-rs.db");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    h.exr("uri_into", "VACUUM INTO 'file:/tmp/h36u.db';");
    h.oi("no_uri_file", (!std::path::Path::new("/tmp/h36u.db").exists()) as i64);
    h.exr("uri_into_params", "VACUUM INTO 'file:/tmp/h36u2.db?mode=rwc';");
    h.check(); }

#[test] fn h006_c002() { let mut h = H::open_path("engine-harvest36-006-C002", &format!("/tmp/h36p-rs-{}.db", std::process::id()));
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2);");
    h.rows("ps_before", "PRAGMA page_size");
    h.ex("PRAGMA page_size=8192;");
    h.rows("ps_pending", "PRAGMA page_size");
    h.exr("vacuum", "VACUUM;");
    h.rows("ps_after", "PRAGMA page_size");
    h.check(); }

#[test] fn h006_c003() { let mut h = H::open_path("engine-harvest36-006-C003", &format!("/tmp/h36q-rs-{}.db", std::process::id()));
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    h.rows("av_before", "PRAGMA auto_vacuum");
    h.ex("PRAGMA auto_vacuum=1;");
    h.rows("av_pending", "PRAGMA auto_vacuum");
    h.exr("vacuum", "VACUUM;");
    h.rows("av_after", "PRAGMA auto_vacuum");
    h.check(); }

#[test] fn h006_c004() { let mut h = H::new("engine-harvest36-006-C004");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t2(b); INSERT INTO aux.t2 VALUES(5);");
    h.exr("vacuum_schema", "VACUUM aux;");
    h.rows("aux_intact", "SELECT b FROM aux.t2");
    h.exr("vacuum_badschema", "VACUUM nosuch;");
    h.check(); }

// ================= 007: attached ANALYZE + PRAGMA optimize (analyze-stats-001) =================

#[test] fn h007_c001() { let mut h = H::new("engine-harvest36-007-C001");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); CREATE INDEX aux.ti ON t(a); INSERT INTO aux.t VALUES(1),(2),(3);");
    h.exr("analyze_aux_table", "ANALYZE aux.t;");
    h.rows("aux_stat1", "SELECT tbl, idx, stat FROM aux.sqlite_stat1");
    h.rows("main_no_stat1", "SELECT count(*) FROM sqlite_master WHERE name='sqlite_stat1'");
    h.check(); }

#[test] fn h007_c002() { let mut h = H::new("engine-harvest36-007-C002");
    h.ex("ATTACH ':memory:' AS aux; CREATE TABLE aux.t(a); CREATE INDEX aux.ti ON t(a); INSERT INTO aux.t VALUES(1),(2);");
    h.exr("analyze_aux_schema", "ANALYZE aux;");
    h.rows("aux_stat1_count", "SELECT count(*) FROM aux.sqlite_stat1");
    h.check(); }

#[test] fn h007_c003() { let mut h = H::new("engine-harvest36-007-C003");
    h.ex("CREATE TABLE t(a); CREATE INDEX ti ON t(a); INSERT INTO t VALUES(1),(2),(3),(4);");
    h.exr("optimize_fresh", "PRAGMA optimize;");
    h.rows("stat1_after", "SELECT count(*) FROM sqlite_master WHERE name='sqlite_stat1'");
    h.rows("stat1_rows", "SELECT tbl, idx, stat FROM sqlite_stat1");
    h.check(); }

// ================= 008: sqlite3_stmt_status (error-status-api-003 deepen) =================

#[test] fn h008_c001() { let mut h = H::new("engine-harvest36-008-C001");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(3),(4),(5);");
    unsafe {
        let st = h.prep("SELECT a FROM t");
        while sqlite3_step(st) == 100 {}
        h.oi("fullscan_after_run", sqlite3_stmt_status(st, 1, 0) as i64);
        h.oi("run_count", sqlite3_stmt_status(st, 6, 0) as i64);
        h.oi("vm_step_pos", (sqlite3_stmt_status(st, 4, 0) > 0) as i64);
        h.oi("memused_pos", (sqlite3_stmt_status(st, 99, 0) > 0) as i64);
        sqlite3_reset(st);
        while sqlite3_step(st) == 100 {}
        h.oi("run_count2", sqlite3_stmt_status(st, 6, 0) as i64);
        h.oi("fullscan_accumulates", sqlite3_stmt_status(st, 1, 0) as i64);
        sqlite3_finalize(st);
    }
    h.check(); }

#[test] fn h008_c002() { let mut h = H::new("engine-harvest36-008-C002");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1),(2),(3),(4),(5);");
    unsafe {
        let st = h.prep("SELECT a FROM t WHERE a = 3");
        while sqlite3_step(st) == 100 {}
        h.oi("where_fullscan", sqlite3_stmt_status(st, 1, 0) as i64);
        sqlite3_finalize(st);
        let st = h.prep("SELECT 1");
        while sqlite3_step(st) == 100 {}
        h.oi("no_table_fullscan", sqlite3_stmt_status(st, 1, 0) as i64);
        h.oi("run_one", sqlite3_stmt_status(st, 6, 0) as i64);
        sqlite3_finalize(st);
    }
    h.check(); }

// ================= anti-cheat (wave 1) =================

#[test] fn anti_cheat_h36_quoted_runtime() { unsafe {
    // runtime-chosen reserved word as a quoted table name with a runtime payload
    let seed = (std::process::id() % 100000) as i64;
    let kw = ["select", "order", "where", "group"][(seed % 4) as usize];
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, &format!("CREATE TABLE \"{kw}\"(v);"));
    exs(db, &format!("INSERT INTO \"{kw}\" VALUES({seed});"));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT v FROM \"{kw}\"")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed, "runtime payload through a quoted reserved table name");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h36_fullscan_runtime() { unsafe {
    // runtime row count N -> FULLSCAN_STEP must report exactly N-1 after one scan
    let n = (std::process::id() % 7) as i64 + 3;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, "CREATE TABLE t(a);");
    for i in 0..n { exs(db, &format!("INSERT INTO t VALUES({i});")); }
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {}
    assert_eq!(sqlite3_stmt_status(st, 1, 0) as i64, n - 1, "FULLSCAN_STEP tracks the runtime row count");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }
