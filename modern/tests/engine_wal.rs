//! Run-32 WAL slice replay — mirrors /tmp/wal_harness.c, asserted byte-identical
//! against the frozen C goldens, plus the mandatory C-interop and anti-cheat tests.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
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
    fn fresh(cid: &'static str, path: &str) -> H {
        for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
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
        sqlite3_free(em as *mut std::os::raw::c_void);
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
    fn ckpt(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={}", self.cid, label, rc)); return; }
        if sqlite3_step(st) == 100 {
            let busy = sqlite3_column_int(st, 0);
            let lg = sqlite3_column_int(st, 1); let ck = sqlite3_column_int(st, 2);
            self.lines.push(format!("OBS {} {} busy={} backfilled={}", self.cid, label, busy, (lg == ck) as i32));
        }
        sqlite3_finalize(st);
    } }
    fn wal(&self) -> String { format!("{}-wal", self.path) }
    fn shm(&self) -> String { format!("{}-shm", self.path) }
    fn fexists(p: &str) -> i64 { std::path::Path::new(p).exists() as i64 }
    fn fsize(p: &str) -> i64 { std::fs::metadata(p).map(|m| m.len() as i64).unwrap_or(-1) }
    fn check(self) {
        unsafe { sqlite3_close(self.db); }
        self.finish() }
    fn finish(self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}

// serialized: cases share /tmp path state within a feature, so use distinct paths per case
#[test] fn c001() { let mut h = H::fresh("engine-wal-001-C001", "/tmp/eftest/rw_wal_c1.db");
    h.rows("set", "PRAGMA journal_mode=WAL");
    let (w, s) = (h.wal(), h.shm());
    h.oi("wal_after_pragma", H::fexists(&w)); h.oi("shm_after_pragma", H::fexists(&s));
    h.check(); }

fn c2_c3(pass: u8) {
    // separate files per test: cargo runs tests concurrently
    let path = if pass == 2 { "/tmp/eftest/rw_wal_c2.db" } else { "/tmp/eftest/rw_wal_c3.db" };
    let mut h = H::fresh("engine-wal-001-C002", path);
    h.ex("PRAGMA journal_mode=WAL;");
    h.exr("write", "CREATE TABLE t(a INT); INSERT INTO t VALUES(7);");
    let (w, s) = (h.wal(), h.shm());
    h.oi("wal_exists", H::fexists(&w));
    h.oi("wal_nonempty", (H::fsize(&w) > 0) as i64);
    h.oi("shm_exists", H::fexists(&s));
    h.rows("read", "SELECT a FROM t");
    if pass == 2 { h.check(); return; }
    unsafe { sqlite3_close(h.db); }
    let mut h2 = H { cid: "engine-wal-001-C003", lines: Vec::new(), db: ptr::null_mut(), path: path.into() };
    h2.oi("wal_after_close", H::fexists(&w));
    h2.oi("shm_after_close", H::fexists(&s));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    unsafe { sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db); }
    h2.db = db;
    h2.rows("mode", "PRAGMA journal_mode");
    h2.rows("rows", "SELECT a FROM t");
    h2.check();
}
#[test] fn c002() { c2_c3(2); }
#[test] fn c003() { c2_c3(3); }

#[test] fn c004() { let path = "/tmp/eftest/rw_wal_c4.db";
    { let mut h = H::fresh("x", path);
      h.ex("PRAGMA journal_mode=WAL;");
      h.ex("CREATE TABLE t(a INT); INSERT INTO t VALUES(7);");
      unsafe { sqlite3_close(h.db); } }
    let mut h = H::open("engine-wal-001-C004", path);
    h.ex("INSERT INTO t VALUES(8);");
    h.rows("set", "PRAGMA journal_mode=delete");
    let w = h.wal();
    h.oi("wal_after_switch", H::fexists(&w));
    h.rows("rows", "SELECT a FROM t ORDER BY a");
    h.reopen();
    h.rows("mode2", "PRAGMA journal_mode");
    h.check(); }

#[test] fn c005() { let mut h = H::open("engine-wal-001-C005", ":memory:");
    h.rows("set", "PRAGMA journal_mode=WAL");
    h.rows("get", "PRAGMA journal_mode");
    h.check(); }

#[test] fn c006() { let mut h = H::fresh("engine-wal-001-C006", "/tmp/eftest/rw_wal_c6.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT);");
    h.ex("INSERT INTO t VALUES(1);"); h.ex("INSERT INTO t VALUES(2);"); h.ex("INSERT INTO t VALUES(3);");
    h.reopen();
    h.rows("rows", "SELECT count(*), sum(a) FROM t");
    h.check(); }

#[test] fn c007() { let mut h = H::fresh("engine-wal-001-C007", "/tmp/eftest/rw_wal_c7.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(1);");
    h.ex("BEGIN; INSERT INTO t VALUES(99); ROLLBACK;");
    h.rows("live", "SELECT count(*) FROM t");
    h.reopen();
    h.rows("reopen", "SELECT count(*) FROM t");
    h.check(); }

#[test] fn c008() { let mut h = H::fresh("engine-wal-001-C008", "/tmp/eftest/rw_wal_c8.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE big(n INT, s TEXT);");
    h.ex("BEGIN;");
    for i in 1..=60 {
        h.ex(&format!("INSERT INTO big VALUES({i},'row-{:04}-{:04}-{:04}');", i, i * 3, i * 7));
    }
    h.ex("COMMIT;");
    h.reopen();
    h.rows("agg", "SELECT count(*), sum(n), min(s), max(s) FROM big");
    h.check(); }

#[test] fn c009() { let path = "/tmp/eftest/rw_wal_c9.db";
    let mut h = H::fresh("engine-wal-001-C009", path);
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(41);");
    unsafe {
        let mut db2: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db2);
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db2, CString::new("SELECT a FROM t").unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        let mut buf = String::new();
        while sqlite3_step(st) == 100 {
            if !buf.is_empty() { buf.push('|'); }
            buf.push_str(&CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy());
        }
        sqlite3_finalize(st);
        h.lines.push(format!("OBS {} conn2 {}", h.cid, buf));
        sqlite3_close(db2);
    }
    h.check(); }

#[test] fn c010() { let path = "/tmp/eftest/rw_wal_c10.db";
    let mut h = H::fresh("engine-wal-001-C010", path);
    h.rows("set", "PRAGMA journal_mode=WAL");
    h.reopen();
    h.rows("mode", "PRAGMA journal_mode");
    h.rows("integ", "PRAGMA integrity_check");
    h.check(); }

#[test] fn k001() { let mut h = H::fresh("engine-wal-002-C001", "/tmp/eftest/rw_wal_k1.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(5); INSERT INTO t VALUES(6);");
    h.ckpt("passive", "PRAGMA wal_checkpoint(PASSIVE)");
    h.rows("rows", "SELECT sum(a) FROM t");
    h.reopen();
    h.rows("reopen", "SELECT sum(a) FROM t");
    h.check(); }

#[test] fn k002() { let mut h = H::fresh("engine-wal-002-C002", "/tmp/eftest/rw_wal_k2.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(1);");
    h.ckpt("cp1", "PRAGMA wal_checkpoint(PASSIVE)");
    h.ex("INSERT INTO t VALUES(2);");
    h.reopen();
    h.rows("rows", "SELECT count(*), sum(a) FROM t");
    h.check(); }

#[test] fn k003() { let mut h = H::fresh("engine-wal-002-C003", "/tmp/eftest/rw_wal_k3.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(9);");
    h.ckpt("bare", "PRAGMA wal_checkpoint");
    h.rows("rows", "SELECT a FROM t");
    h.check(); }

#[test] fn k004() { let mut h = H::fresh("engine-wal-002-C004", "/tmp/eftest/rw_wal_k4.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(11);");
    h.ckpt("full", "PRAGMA wal_checkpoint(FULL)");
    h.rows("rows", "SELECT a FROM t");
    h.check(); }

#[test] fn k005() { let mut h = H::fresh("engine-wal-002-C005", "/tmp/eftest/rw_wal_k5.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(12);");
    let w = h.wal();
    h.oi("wal_before", (H::fsize(&w) > 0) as i64);
    h.ckpt("trunc", "PRAGMA wal_checkpoint(TRUNCATE)");
    h.oi("wal_size_zero", (H::fsize(&w) == 0) as i64);
    h.rows("rows", "SELECT a FROM t");
    h.reopen();
    h.rows("reopen", "SELECT a FROM t");
    h.check(); }

#[test] fn k006() { let mut h = H::fresh("engine-wal-002-C006", "/tmp/eftest/rw_wal_k6.db");
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(a INT); INSERT INTO t VALUES(13);");
    h.ckpt("restart", "PRAGMA wal_checkpoint(RESTART)");
    h.ex("INSERT INTO t VALUES(14);");
    h.rows("rows", "SELECT count(*), sum(a) FROM t");
    h.reopen();
    h.rows("reopen", "SELECT count(*), sum(a) FROM t");
    h.check(); }

// ---- MANDATORY: C reads a Rust-written WAL-mode database mid-session ----
// The copy holds an (empty) main db + all data ONLY in the -wal, so C accepting it
// proves frame format, salts and cumulative checksums — not just the main-db writer.
#[test] fn rust_write_c_read_wal() {
    let cli = pin_cli();
    assert!(std::path::Path::new(&cli).exists(), "pinned sqlite3 CLI required at {cli}");
    let n: i64 = 700_000 + (std::process::id() as i64 % 1000);
    let path = format!("/tmp/eftest/rwcr_wal_{}.db", std::process::id());
    let copy = format!("/tmp/eftest/rwcr_walcopy_{}.db", std::process::id());
    for p in [&path, &copy] { for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{p}{sfx}")); } }
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path.clone()).unwrap().as_ptr(), &mut db);
        let sql = format!("PRAGMA journal_mode=WAL; CREATE TABLE f(a INTEGER); INSERT INTO f VALUES({n});");
        sqlite3_exec(db, CString::new(sql).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
        // mid-session: -wal must exist and carry the committed data
        assert!(std::fs::metadata(format!("{path}-wal")).map(|m| m.len() > 0).unwrap_or(false), "-wal missing");
        std::fs::copy(&path, &copy).unwrap();
        std::fs::copy(format!("{path}-wal"), format!("{copy}-wal")).unwrap();
        sqlite3_close(db);
    }
    let out = Command::new(&cli).arg(&copy).arg("SELECT a FROM f;").output().expect("run pin cli");
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), n.to_string(),
               "pinned C must recover Rust's WAL (stderr: {})", String::from_utf8_lossy(&out.stderr));
    let ic = Command::new(&cli).arg(&copy).arg("PRAGMA integrity_check;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&ic.stdout).trim(), "ok");
    for p in [&path, &copy] { for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{p}{sfx}")); } }
}

// ---- MANDATORY anti-cheat: runtime value survives close/reopen in WAL mode ----
#[test] fn anti_cheat_wal_runtime_reopen() {
    let seed = (std::process::id() % 971 + 29) as i64;
    let path = format!("/tmp/eftest/ac_wal_{}.db", std::process::id());
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
    let mut h = H::open("x", &path);
    h.ex("PRAGMA journal_mode=WAL;");
    h.ex(&format!("CREATE TABLE t(k INT, v TEXT); INSERT INTO t VALUES({seed}, 'w{seed}');"));
    h.reopen();
    h.rows("q", "SELECT k, v FROM t");
    assert_eq!(h.lines.pop().unwrap(), format!("OBS x q {seed},w{seed}"));
    h.rows("mode", "PRAGMA journal_mode");
    assert_eq!(h.lines.pop().unwrap(), "OBS x mode wal");
    unsafe { sqlite3_close(h.db); }
}

// ---- MANDATORY anti-cheat: PASSIVE checkpoint observable in the main db file ----
#[test] fn anti_cheat_wal_checkpoint_passive() {
    let seed = (std::process::id() % 953 + 17) as i64;
    let path = format!("/tmp/eftest/ac_ckpt_{}.db", std::process::id());
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
    let mut h = H::open("x", &path);
    h.ex("PRAGMA journal_mode=WAL;");
    h.ex(&format!("CREATE TABLE t(k INT); INSERT INTO t VALUES({seed});"));
    // before checkpoint: main db lacks the row (all data lives in the -wal)
    let cli = pin_cli();
    let pre = Command::new(&cli).arg(format!("file:{path}?immutable=1")).arg("SELECT count(*) FROM t;").output().unwrap();
    let pre_txt = String::from_utf8_lossy(&pre.stdout).trim().to_string();
    h.ckpt("cp", "PRAGMA wal_checkpoint(PASSIVE)");
    assert_eq!(h.lines.pop().unwrap(), "OBS x cp busy=0 backfilled=1");
    // after checkpoint: the row is IN the main db even ignoring the wal
    let post = Command::new(&cli).arg(format!("file:{path}?immutable=1")).arg("SELECT k FROM t;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&post.stdout).trim(), seed.to_string(),
               "main db must contain the row after PASSIVE (pre-ckpt view was '{pre_txt}')");
    unsafe { sqlite3_close(h.db); }
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
}

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
