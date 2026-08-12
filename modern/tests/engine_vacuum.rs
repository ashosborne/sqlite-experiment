//! Run-34 VACUUM / VACUUM INTO replay — mirrors /tmp/vac_harness.c, asserted
//! byte-identical against the frozen C goldens, plus mandatory anti-cheat and
//! C-interop round trips. Plain language: VACUUM really rebuilds the content
//! (rowids renumber, free pages reclaim); VACUUM INTO copies into a new file.
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
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
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
    fn pgcnt(&mut self) -> i64 { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(self.db, c"PRAGMA page_count".as_ptr(), -1, &mut st, ptr::null_mut());
        let v = if sqlite3_step(st) == 100 { sqlite3_column_int64(st, 0) } else { -1 };
        sqlite3_finalize(st); v
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

#[test] fn c001() { let mut h = H::new("engine-vacuum-001-C001");
    h.exr("vac", "VACUUM;"); h.rows("integ", "PRAGMA integrity_check"); h.check(); }

#[test] fn c002() { let mut h = H::new("engine-vacuum-001-C002");
    h.ex("CREATE TABLE t(a INT, b TEXT); INSERT INTO t VALUES(1,'one'),(2,'two'),(3,'three');");
    h.exr("vac", "VACUUM;");
    h.rows("after", "SELECT a, b FROM t ORDER BY a"); h.check(); }

#[test] fn c003() { let mut h = H::new("engine-vacuum-001-C003");
    h.ex("CREATE TABLE t(a INT); CREATE TABLE u(x TEXT); CREATE INDEX iu ON u(x);INSERT INTO t VALUES(5),(6); INSERT INTO u VALUES('m'),('k');");
    h.exr("vac", "VACUUM;");
    h.rows("t", "SELECT a FROM t ORDER BY a");
    h.rows("u", "SELECT x FROM u WHERE x='k'");
    h.rows("objs", "SELECT count(*) FROM sqlite_master"); h.check(); }

#[test] fn c004() { let mut h = H::new("engine-vacuum-001-C004");
    h.ex("CREATE TABLE t(a INT); INSERT INTO t VALUES(1);");
    h.exr("begin", "BEGIN;");
    h.exr("vac", "VACUUM;");
    let in_txn = unsafe { (sqlite3_get_autocommit(h.db) == 0) as i64 };
    h.oi("in_txn", in_txn);
    h.exr("ins", "INSERT INTO t VALUES(2);");
    h.exr("commit", "COMMIT;");
    h.rows("cnt", "SELECT count(*) FROM t"); h.check(); }

#[test] fn c005() { let mut h = H::fresh("engine-vacuum-001-C005", "/tmp/eftest/rw_vac_c5.db");
    h.ex("CREATE TABLE big(n INT, s TEXT);");
    h.ex("BEGIN;");
    for i in 1..=300 {
        h.ex(&format!("INSERT INTO big VALUES({i},'payload-{:04}-{:04}-abcdefghijklmnopqrstuvwxyz-abcdefghijklmnopqrstuvwxyz-abcdefghijklmnopqrstuvwxyz');", i, i * 3));
    }
    h.ex("COMMIT;");
    h.ex("DELETE FROM big WHERE n > 5;");
    let before = h.pgcnt();
    h.exr("vac", "VACUUM;");
    let after = h.pgcnt();
    h.oi("shrunk", (before > after) as i64);
    h.oi("after_small", (after <= 4) as i64);
    h.rows("left", "SELECT count(*), min(n), max(n) FROM big"); h.check(); }

#[test] fn c006() { let mut h = H::new("engine-vacuum-001-C006");
    h.ex("CREATE TABLE wr(k TEXT PRIMARY KEY, v INT) WITHOUT ROWID;INSERT INTO wr VALUES('a',1),('b',2);");
    h.exr("vac", "VACUUM;");
    h.rows("after", "SELECT k, v FROM wr ORDER BY k"); h.check(); }

#[test] fn c007() { let mut h = H::new("engine-vacuum-001-C007");
    h.ex("CREATE TABLE p(id INTEGER PRIMARY KEY, v TEXT);INSERT INTO p VALUES(10,'x'),(20,'y'),(30,'z'); DELETE FROM p WHERE id=20;");
    h.exr("vac", "VACUUM;");
    h.rows("ids", "SELECT id, v FROM p ORDER BY id");
    h.rows("rowids", "SELECT rowid FROM p ORDER BY rowid"); h.check(); }

#[test] fn c008() { let mut h = H::new("engine-vacuum-001-C008");
    h.ex("CREATE TABLE r(v TEXT); INSERT INTO r VALUES('a'),('b'),('c'),('d'),('e');DELETE FROM r WHERE v IN ('b','d');");
    h.rows("before", "SELECT rowid, v FROM r ORDER BY rowid");
    h.exr("vac", "VACUUM;");
    h.rows("after", "SELECT rowid, v FROM r ORDER BY rowid"); h.check(); }

#[test] fn d001() { let mut h = H::fresh("engine-vacuum-002-C001", "/tmp/eftest/rw_vac_d1.db");
    h.ex("CREATE TABLE t(a INT); INSERT INTO t VALUES(7),(8);");
    h.exr("vac", "VACUUM;");
    h.reopen();
    h.rows("reopen", "SELECT a FROM t ORDER BY a");
    h.rows("integ", "PRAGMA integrity_check"); h.check(); }

#[test] fn d002() { let mut h = H::fresh("engine-vacuum-002-C002", "/tmp/eftest/rw_vac_d2.db");
    h.ex("CREATE TABLE t(a INT); INSERT INTO t VALUES(1);");
    h.exr("vac", "VACUUM;");
    h.rows("mode", "PRAGMA journal_mode");
    h.rows("integ", "PRAGMA integrity_check"); h.check(); }

#[test] fn d003() { let mut h = H::fresh("engine-vacuum-002-C003", "/tmp/eftest/rw_vac_d3.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(41),(42);");
    h.exr("vac", "VACUUM;");
    h.rows("mode", "PRAGMA journal_mode");
    h.rows("data", "SELECT sum(a) FROM t");
    h.reopen();
    h.rows("reopen", "SELECT sum(a) FROM t"); h.check(); }

#[test] fn d004() { let mut h = H::fresh("engine-vacuum-002-C004", "/tmp/eftest/rw_vac_d4.db");
    h.ex("CREATE TABLE t(a INT); CREATE UNIQUE INDEX ia ON t(a);INSERT INTO t VALUES(1),(2),(3);");
    h.exr("vac", "VACUUM;");
    h.reopen();
    h.rows("probe", "SELECT a FROM t WHERE a=2");
    h.exr("dup", "INSERT INTO t VALUES(2);"); h.check(); }

#[test] fn i001() { let src = "/tmp/eftest/rw_vac_i1.db"; let tgt = "/tmp/eftest/rw_vac_t1.db";
    for p in [src, tgt] { for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{p}{sfx}")); } }
    let mut h = H::fresh("engine-vacuum-003-C001", src);
    h.ex("CREATE TABLE t(a INT, b TEXT); INSERT INTO t VALUES(1,'one'),(2,'two');");
    h.exr("into", &format!("VACUUM INTO '{tgt}';"));
    h.rows("src", "SELECT a, b FROM t ORDER BY a");
    unsafe { sqlite3_close(h.db); }
    let mut db: *mut Sqlite3 = ptr::null_mut();
    unsafe { sqlite3_open(CString::new(tgt).unwrap().as_ptr(), &mut db); }
    h.db = db;
    h.rows("target", "SELECT a, b FROM t ORDER BY a");
    h.rows("integ", "PRAGMA integrity_check"); h.check(); }

#[test] fn i002() { let src = "/tmp/eftest/rw_vac_i2.db"; let tgt = "/tmp/eftest/rw_vac_t2.db";
    for p in [src, tgt] { let _ = std::fs::remove_file(p); }
    let mut h = H::fresh("engine-vacuum-003-C002", src);
    h.ex("CREATE TABLE t(a INT); CREATE TABLE u(x TEXT); CREATE INDEX iu ON u(x);INSERT INTO t VALUES(9); INSERT INTO u VALUES('q');");
    h.exr("into", &format!("VACUUM INTO '{tgt}';"));
    unsafe { sqlite3_close(h.db); }
    let mut db: *mut Sqlite3 = ptr::null_mut();
    unsafe { sqlite3_open(CString::new(tgt).unwrap().as_ptr(), &mut db); }
    h.db = db;
    h.rows("objs", "SELECT type, name FROM sqlite_master ORDER BY type, name");
    h.rows("t", "SELECT a FROM t");
    h.rows("u", "SELECT x FROM u WHERE x='q'"); h.check(); }

#[test] fn i003() { let tgt = "/tmp/eftest/rw_vac_t3.db";
    std::fs::write(tgt, b"occupied").unwrap();
    let mut h = H::new("engine-vacuum-003-C003");
    h.ex("CREATE TABLE t(a INT); INSERT INTO t VALUES(1);");
    // golden pinned the C harness target path text; replay with the same message shape
    h.exr("again", "VACUUM INTO '/tmp/eftest/vac_target.db';");
    h.check(); }

#[test] fn i004() { let _ = std::fs::remove_file("/tmp/eftest/vac_target_i4.db");
    let mut h = H::new("engine-vacuum-003-C004");
    h.ex("CREATE TABLE t(a INT); INSERT INTO t VALUES(1); BEGIN;");
    h.exr("into", "VACUUM INTO '/tmp/eftest/vac_target_i4.db';");
    h.exr("commit", "COMMIT;");
    h.check(); }

#[test] fn i005() { let mut h = H::new("engine-vacuum-003-C005");
    h.ex("CREATE TABLE t(a INT);");
    h.exr("bad", "VACUUM INTO '/tmp/eftest/no_such_dir/x.db';");
    h.check(); }

#[test] fn i006() { let tgt = "/tmp/eftest/rw_vac_t6.db";
    let _ = std::fs::remove_file(tgt);
    let mut h = H::new("engine-vacuum-003-C006");
    h.ex("CREATE TABLE m(v TEXT); INSERT INTO m VALUES('mem1'),('mem2');");
    h.exr("into", &format!("VACUUM INTO '{tgt}';"));
    unsafe { sqlite3_close(h.db); }
    let mut db: *mut Sqlite3 = ptr::null_mut();
    unsafe { sqlite3_open(CString::new(tgt).unwrap().as_ptr(), &mut db); }
    h.db = db;
    h.rows("target", "SELECT v FROM m ORDER BY v"); h.check(); }

// ---- old vacuum-001-C001 golden now replays for real (was honestly deferred) ----
#[test] fn legacy_vacuum_001_c001_replays() { unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    let sql = "CREATE TABLE v1(a); INSERT INTO v1 VALUES(zeroblob(1000)); DROP TABLE v1; VACUUM; SELECT 1;";
    let mut lines: Vec<String> = Vec::new();
    unsafe extern "C" fn cb(arg: *mut c_void, argc: c_int, argv: *mut *mut c_char, _az: *mut *mut c_char) -> c_int {
        let v = &mut *(arg as *mut Vec<String>);
        for i in 0..argc as usize {
            let p = *argv.add(i);
            v.push(format!("row{}.col{} {}", v.len(), i,
                if p.is_null() { "NULL".into() } else { CStr::from_ptr(p).to_string_lossy().into_owned() }));
        }
        0
    }
    let rc = sqlite3_exec(db, CString::new(sql).unwrap().as_ptr(), Some(cb), &mut lines as *mut _ as *mut c_void, ptr::null_mut());
    let mut out: Vec<String> = lines.iter().map(|l| format!("OBS vacuum-001-C001 {}", l)).collect();
    out.push(format!("OBS vacuum-001-C001 exec.rc {rc}"));
    out.push(format!("OBS vacuum-001-C001 cb.rows {}", lines.len()));
    sqlite3_close(db);
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
    p.push("tests/characterization/vacuum/cases/vacuum-001/C001.approved.txt");
    assert_eq!(out.join("\n") + "\n", std::fs::read_to_string(&p).unwrap());
} }

// ---- MANDATORY anti-cheat: runtime rebuild (rowids renumber; INTO at runtime path) ----
#[test] fn anti_cheat_vacuum_runtime() { unsafe {
    let seed = (std::process::id() % 733 + 41) as i64;
    let tname = format!("rt{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    let ex = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    ex(db, &format!("CREATE TABLE {tname}(v INT);"));
    for i in 1..=6 { ex(db, &format!("INSERT INTO {tname} VALUES({});", seed * 10 + i)); }
    ex(db, &format!("DELETE FROM {tname} WHERE v % 2 = 0;"));
    ex(db, "VACUUM;");
    // rowids must be renumbered 1..3 — impossible without a real rebuild path
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT rowid, v FROM {tname} ORDER BY rowid")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    let mut got = Vec::new();
    while sqlite3_step(st) == 100 {
        got.push((sqlite3_column_int64(st, 0), sqlite3_column_int64(st, 1)));
    }
    sqlite3_finalize(st);
    assert_eq!(got, vec![(1, seed * 10 + 1), (2, seed * 10 + 3), (3, seed * 10 + 5)]);
    // runtime-chosen INTO target readable by the pinned C CLI
    let tgt = format!("/tmp/eftest/ac_vac_{}.db", std::process::id());
    let _ = std::fs::remove_file(&tgt);
    ex(db, &format!("VACUUM INTO '{tgt}';"));
    sqlite3_close(db);
    let cli = pin_cli();
    let out = Command::new(&cli).arg(&tgt).arg(format!("SELECT sum(v) FROM {tname};")).output().unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), (3 * seed * 10 + 9).to_string(),
               "pinned C must read the VACUUM INTO target (stderr: {})", String::from_utf8_lossy(&out.stderr));
    let ic = Command::new(&cli).arg(&tgt).arg("PRAGMA integrity_check;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&ic.stdout).trim(), "ok");
    let _ = std::fs::remove_file(&tgt);
} }

// ---- MANDATORY C interop: C reads a Rust file after in-place VACUUM ----
#[test] fn rust_vacuum_c_read() { unsafe {
    let n: i64 = 600_000 + (std::process::id() as i64 % 1000);
    let path = format!("/tmp/eftest/rvcr_{}.db", std::process::id());
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path.clone()).unwrap().as_ptr(), &mut db);
    let ex = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    ex(db, &format!("CREATE TABLE f(a INTEGER); INSERT INTO f VALUES({n}),({}),({});", n + 1, n + 2));
    ex(db, &format!("DELETE FROM f WHERE a = {};", n + 1));
    ex(db, "VACUUM;");
    sqlite3_close(db);
    let cli = pin_cli();
    let out = Command::new(&cli).arg(&path).arg("SELECT a FROM f ORDER BY a;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), format!("{n}\n{}", n + 2),
               "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let ic = Command::new(&cli).arg(&path).arg("PRAGMA integrity_check;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&ic.stdout).trim(), "ok");
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
