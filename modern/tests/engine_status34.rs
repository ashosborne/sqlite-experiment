//! Run-44 status/pragma matrix replay — mirrors /tmp/sp_harness.c.
//! Plain language: ask the global/connection counters (status64/db_status) and
//! get/set pragmas + read their rows the way C does. Counter magnitudes are machine
//! state, so the pins are predicates + exact zeros + exact rc codes + exact rows.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

// global allocator counters are process-wide: serialize this file's tests
static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

struct H { cid: String, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn nodb(cid: &str) -> H { H { cid: cid.into(), lines: Vec::new(), db: ptr::null_mut() } }
    fn open(&mut self, path: &str) { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        self.db = db;
    } }
    fn close(&mut self) { unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } } }
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
    fn obs(&mut self, tail: String) { self.lines.push(format!("OBS {} {}", self.cid, tail)); }
    fn pragma_i64(&mut self, sql: &str) -> i64 { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        sqlite3_step(st);
        let v = sqlite3_column_int64(st, 0);
        sqlite3_finalize(st); v
    } }
    /// flush this case's lines against its golden and start the next case
    fn case_done(&mut self, next_cid: Option<&str>) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap().to_string();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
        if let Some(n) = next_cid { self.cid = n.to_string(); }
    }
}

fn fresh(path: &str) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(format!("{path}-wal"));
    let _ = std::fs::remove_file(format!("{path}-journal"));
}

// ================= A — global status64 matrix (engine-status34-001) =================

#[test] fn a001() { let _g = lockg(); unsafe { let mut h = H::nodb("engine-status34-001-C001");
    let (mut c, mut hi): (i64, i64) = (0, 0);
    let rcs: Vec<String> = (0..=9).map(|op| sqlite3_status64(op, &mut c, &mut hi, 0).to_string()).collect();
    h.obs(format!("valid_rcs {}", rcs.join(",")));
    h.oi("op10_rc", sqlite3_status64(10, &mut c, &mut hi, 0) as i64);
    h.oi("neg_rc", sqlite3_status64(-1, &mut c, &mut hi, 0) as i64);
    h.case_done(None); } }

#[test] fn a002() { let _g = lockg(); unsafe { let mut h = H::nodb("engine-status34-001-C002");
    let (mut c, mut hi): (i64, i64) = (0, 0);
    let mut allz = 1i64;
    for op in [1, 3, 4, 6, 8] {
        sqlite3_status64(op, &mut c, &mut hi, 0);
        if c != 0 || hi != 0 { allz = 0; }
    }
    h.oi("unused_ops_all_zero", allz);
    h.case_done(None); } }

#[test] fn a003() { let _g = lockg(); unsafe { let mut h = H::nodb("engine-status34-001-C003");
    let (mut c, mut hi): (i64, i64) = (0, 0);
    let p = sqlite3_malloc64(3000);
    sqlite3_status64(5, &mut c, &mut hi, 0);
    h.oi("malloc_size_cur0", (c == 0) as i64); h.oi("malloc_size_hi_pos", (hi > 0) as i64);
    sqlite3_status64(7, &mut c, &mut hi, 0);
    h.oi("pagecache_size_cur0", (c == 0) as i64);
    sqlite3_status64(9, &mut c, &mut hi, 0);
    h.oi("malloc_count_pos", (c > 0) as i64); h.oi("malloc_count_hi_ge_cur", (hi >= c) as i64);
    sqlite3_free(p);
    h.case_done(None); } }

#[test] fn a004() { let _g = lockg(); unsafe { let mut h = H::nodb("engine-status34-001-C004");
    let path = format!("/tmp/st34a-rs-{}.db", std::process::id());
    fresh(&path);
    let (mut c, mut hi): (i64, i64) = (0, 0);
    h.open(&path);
    h.ex("CREATE TABLE t(a,b); INSERT INTO t VALUES(1,'x'),(2,'y');");
    h.rows("probe", "SELECT count(*) FROM t");
    sqlite3_status64(2, &mut c, &mut hi, 0);
    h.oi("pagecache_ovf_pos", (c > 0) as i64); h.oi("pagecache_ovf_hi_ge_cur", (hi >= c) as i64);
    h.close();
    h.case_done(None); } }

#[test] fn a005() { let _g = lockg(); unsafe { let mut h = H::nodb("engine-status34-001-C005");
    let (mut c1, mut h1, mut c2, mut h2): (i64, i64, i64, i64) = (0, 0, 0, 0);
    let p = sqlite3_malloc64(4000);
    sqlite3_free(p);
    sqlite3_status64(9, &mut c1, &mut h1, 1);
    sqlite3_status64(9, &mut c2, &mut h2, 0);
    h.oi("reset_hi_eq_cur", (h2 == c2) as i64); h.oi("hi_le_prev", (h2 <= h1) as i64);
    h.case_done(None); } }

#[test] fn a006() { let _g = lockg(); unsafe { let mut h = H::nodb("engine-status34-001-C006");
    let (mut c64, mut h64): (i64, i64) = (0, 0);
    let (mut c32, mut h32): (c_int, c_int) = (0, 0);
    let rc64 = sqlite3_status64(0, &mut c64, &mut h64, 0);
    let rc32 = sqlite3_status(0, &mut c32, &mut h32, 0);
    h.oi("rc_both_ok", (rc64 == 0 && rc32 == 0) as i64);
    h.oi("cur_matches", (c32 as i64 == c64) as i64);
    h.oi("hi_matches", (h32 as i64 == h64) as i64);
    h.case_done(None); } }

// ================= B — db_status matrix (engine-status34-002, one shared db) =================

#[test] fn b_batch() { let _g = lockg(); unsafe {
    let path = format!("/tmp/st34b-rs-{}.db", std::process::id());
    fresh(&path);
    let mut h = H::nodb("engine-status34-002-C001");
    h.open(&path);
    h.ex("CREATE TABLE t(a,b); CREATE INDEX ti ON t(a); INSERT INTO t VALUES(1,'x'),(2,'y'),(3,'z');");

    let (mut c, mut hi): (c_int, c_int) = (0, 0);
    let rcs: Vec<String> = (0..=12).map(|op| sqlite3_db_status(h.db, op, &mut c, &mut hi, 0).to_string()).collect();
    h.obs(format!("valid_rcs {}", rcs.join(",")));
    h.oi("badop_rc", sqlite3_db_status(h.db, 9999, &mut c, &mut hi, 0) as i64);
    h.case_done(Some("engine-status34-002-C002"));

    let (mut c0, mut h0, mut c1, mut h1): (c_int, c_int, c_int, c_int) = (0, 0, 0, 0);
    sqlite3_db_status(h.db, 2, &mut c0, &mut h0, 0);
    h.ex("CREATE TABLE more_schema(a,b,c,d,e);");
    sqlite3_db_status(h.db, 2, &mut c1, &mut h1, 0);
    h.oi("schema_pos", (c0 > 0) as i64); h.oi("schema_grew", (c1 > c0) as i64);
    h.oi("schema_hi_zero", (h0 == 0 && h1 == 0) as i64);
    h.case_done(Some("engine-status34-002-C003"));

    let (mut cs, mut hs): (c_int, c_int) = (0, 0);
    sqlite3_db_status(h.db, 1, &mut c, &mut hi, 0);
    sqlite3_db_status(h.db, 11, &mut cs, &mut hs, 0);
    h.oi("cache_used_pos", (c > 0) as i64); h.oi("cache_used_hi_zero", (hi == 0) as i64);
    h.oi("shared_eq_used", (cs == c) as i64);
    h.case_done(Some("engine-status34-002-C004"));

    let (mut c2, mut h2): (c_int, c_int) = (0, 0);
    sqlite3_db_status(h.db, 3, &mut c0, &mut h0, 0);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(h.db, c"SELECT a, b FROM t WHERE a > 1".as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_db_status(h.db, 3, &mut c1, &mut h1, 0);
    sqlite3_finalize(st);
    sqlite3_db_status(h.db, 3, &mut c2, &mut h2, 0);
    h.oi("none_zero", (c0 == 0) as i64); h.oi("live_pos", (c1 > 0) as i64);
    h.oi("finalized_zero", (c2 == 0) as i64); h.oi("hi_zero", (h1 == 0) as i64);
    h.case_done(Some("engine-status34-002-C005"));

    h.ex("INSERT INTO t VALUES(4,'w');");
    h.rows("probe", "SELECT sum(a) FROM t");
    sqlite3_db_status(h.db, 9, &mut c, &mut hi, 0);
    h.oi("cache_write_pos", (c > 0) as i64); h.oi("write_hi_zero", (hi == 0) as i64);
    sqlite3_db_status(h.db, 7, &mut c, &mut hi, 0);
    h.oi("cache_hit_pos", (c > 0) as i64); h.oi("hit_hi_zero", (hi == 0) as i64);
    h.case_done(Some("engine-status34-002-C006"));

    h.close();
    h.open(&path); // cold cache: reads must miss
    h.rows("probe", "SELECT count(*) FROM t");
    sqlite3_db_status(h.db, 8, &mut c, &mut hi, 0);
    h.oi("cache_miss_pos", (c > 0) as i64); h.oi("miss_hi_zero", (hi == 0) as i64);
    h.case_done(Some("engine-status34-002-C007"));

    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY);\
          CREATE TABLE ch(pid REFERENCES p(id) DEFERRABLE INITIALLY DEFERRED);");
    sqlite3_db_status(h.db, 10, &mut c, &mut hi, 0);
    h.oi("deferred_before", c as i64);
    h.ex("BEGIN;");
    h.exr("def_ins", "INSERT INTO ch VALUES(999);");
    sqlite3_db_status(h.db, 10, &mut c, &mut hi, 0);
    h.oi("deferred_in_txn", c as i64); h.oi("deferred_hi_zero", (hi == 0) as i64);
    h.ex("ROLLBACK;");
    sqlite3_db_status(h.db, 10, &mut c, &mut hi, 0);
    h.oi("deferred_after_rb", c as i64);
    h.case_done(Some("engine-status34-002-C008"));

    let mut allz = 1i64;
    for op in [5, 6, 12] {
        sqlite3_db_status(h.db, op, &mut c, &mut hi, 0);
        if c != 0 || hi != 0 { allz = 0; }
    }
    h.oi("quiet_ops_all_zero", allz);
    h.case_done(None);
    h.close();
} }

// ================= C+D — pragma dispatcher + TVFs (one shared db, C order) =================

#[test] fn cd_batch() { let _g = lockg(); unsafe {
    let path = format!("/tmp/st34c-rs-{}.db", std::process::id());
    fresh(&path);
    let mut h = H::nodb("engine-pragma34-001-C001");
    h.open(&path);
    h.ex("CREATE TABLE t(a,b); CREATE INDEX ti ON t(a); INSERT INTO t VALUES(1,'x');");
    h.ex("CREATE TABLE u(c CHECK(c > 0));");

    h.rows("uv0", "PRAGMA user_version");
    h.exr("uv_set", "PRAGMA user_version = 42;");
    h.rows("uv1", "PRAGMA user_version");
    h.rows("app0", "PRAGMA application_id");
    h.exr("app_set", "PRAGMA application_id = 1234;");
    h.rows("app1", "PRAGMA application_id");
    h.case_done(Some("engine-pragma34-001-C002"));

    let v1 = h.pragma_i64("PRAGMA data_version");
    h.ex("INSERT INTO t VALUES(2,'y');");
    let v2 = h.pragma_i64("PRAGMA data_version");
    {
        let mut db2: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path.clone()).unwrap().as_ptr(), &mut db2);
        sqlite3_exec(db2, c"INSERT INTO t VALUES(3,'z');".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
        sqlite3_close(db2);
    }
    let v3 = h.pragma_i64("PRAGMA data_version");
    h.oi("own_write_same", (v1 == v2) as i64); h.oi("sibling_bumps", (v3 > v2) as i64);
    h.case_done(Some("engine-pragma34-001-C003"));

    let s1 = h.pragma_i64("PRAGMA schema_version");
    h.ex("CREATE TABLE sv_probe(x);");
    let s2 = h.pragma_i64("PRAGMA schema_version");
    h.oi("ddl_bumps_by_one", (s2 == s1 + 1) as i64);
    h.case_done(Some("engine-pragma34-001-C004"));

    h.exr("unknown_get", "PRAGMA not_a_real_pragma_zz;");
    h.rows("unknown_rows", "PRAGMA not_a_real_pragma_zz");
    h.exr("unknown_set", "PRAGMA not_a_real_pragma_zz = 7;");
    h.case_done(Some("engine-pragma34-001-C005"));

    h.rows("qo0", "PRAGMA query_only");
    h.exr("qo_on", "PRAGMA query_only = 1;");
    h.exr("write_blocked", "INSERT INTO t VALUES(9,'q');");
    h.exr("qo_off", "PRAGMA query_only = 0;");
    h.exr("write_ok", "INSERT INTO t VALUES(9,'q');");
    h.case_done(Some("engine-pragma34-001-C006"));

    h.exr("chk_violate", "INSERT INTO u VALUES(-5);");
    h.exr("icc_on", "PRAGMA ignore_check_constraints = 1;");
    h.exr("chk_pass", "INSERT INTO u VALUES(-5);");
    h.rows("u_rows", "SELECT c FROM u");
    h.exr("icc_off", "PRAGMA ignore_check_constraints = 0;");
    h.exr("chk_violate2", "INSERT INTO u VALUES(-6);");
    h.case_done(Some("engine-pragma34-001-C007"));

    h.rows("freelist", "PRAGMA freelist_count");
    h.rows("quick_check", "PRAGMA quick_check");
    h.case_done(Some("engine-pragma34-001-C008"));

    h.rows("collations", "PRAGMA collation_list");
    h.case_done(Some("engine-pragma34-001-C009"));

    h.rows("txinfo", "PRAGMA table_xinfo(t)");
    h.rows("iinfo", "PRAGMA index_info(ti)");
    h.rows("ixinfo", "PRAGMA index_xinfo(ti)");
    h.case_done(Some("engine-pragma34-002-C001"));

    h.rows("tvf_coll", "SELECT name FROM pragma_collation_list ORDER BY name");
    h.case_done(Some("engine-pragma34-002-C002"));

    h.rows("tvf_txinfo", "SELECT name, type, hidden FROM pragma_table_xinfo('t')");
    h.case_done(Some("engine-pragma34-002-C003"));

    h.rows("tvf_iinfo", "SELECT seqno, cid, name FROM pragma_index_info('ti')");
    h.case_done(Some("engine-pragma34-002-C004"));

    h.rows("tvf_copt_count", "SELECT count(*) FROM pragma_compile_options");
    h.rows("tvf_copt_first", "SELECT compile_options FROM pragma_compile_options LIMIT 2");
    h.case_done(None);
    h.close();
} }

// ================= anti-cheat =================

#[test] fn anti_cheat_status34_runtime_counters() { let _g = lockg(); unsafe {
    // a runtime-sized allocation must move MEMORY_USED by at least that size, and a
    // runtime-named schema object must grow SCHEMA_USED — a cheat sheet cannot know either.
    let seed = (std::process::id() % 1000) as i64 + 100;
    let n = (seed * 37) as u64;
    let (mut c0, mut h0, mut c1, mut h1): (i64, i64, i64, i64) = (0, 0, 0, 0);
    sqlite3_status64(0, &mut c0, &mut h0, 0);
    let p = sqlite3_malloc64(n);
    sqlite3_status64(0, &mut c1, &mut h1, 0);
    assert!(c1 - c0 >= n as i64, "runtime allocation must move MEMORY_USED");
    sqlite3_free(p);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let (mut s0, mut sh): (c_int, c_int) = (0, 0);
    sqlite3_db_status(db, 2, &mut s0, &mut sh, 0);
    sqlite3_exec(db, CString::new(format!("CREATE TABLE rt_{seed}(a,b,c);")).unwrap().as_ptr(),
        None, ptr::null_mut(), ptr::null_mut());
    let (mut s1, mut sh1): (c_int, c_int) = (0, 0);
    sqlite3_db_status(db, 2, &mut s1, &mut sh1, 0);
    assert!(s1 > s0, "runtime-named table must grow SCHEMA_USED");
    assert_eq!(sh1, 0, "SCHEMA_USED highwater is 0 like C");
    sqlite3_close(db);
} }

#[test] fn anti_cheat_pragma34_runtime_roundtrip() { let _g = lockg(); unsafe {
    // runtime pragma value set/get + runtime table name in the TVF output
    let seed = (std::process::id() % 100000) as i64;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, &format!("PRAGMA user_version = {seed};"));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"PRAGMA user_version".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed, "runtime pragma value must round-trip");
    sqlite3_finalize(st);
    exs(db, &format!("CREATE TABLE rt_{seed}(only_col_{seed});"));
    let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT name FROM pragma_table_xinfo('rt_{seed}')")).unwrap().as_ptr(),
        -1, &mut st2, ptr::null_mut());
    assert_eq!(sqlite3_step(st2), 100);
    assert_eq!(CStr::from_ptr(sqlite3_column_text(st2, 0) as *const c_char).to_string_lossy(),
        format!("only_col_{seed}"), "runtime column name must appear in the TVF");
    sqlite3_finalize(st2);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_status34_badop_and_collation() { let _g = lockg(); unsafe {
    // bad ops still fail (no silent success), and a runtime-registered collation
    // appears at the head of PRAGMA collation_list (live registry, not a canned row set).
    let (mut c, mut hi): (i64, i64) = (0, 0);
    assert_eq!(sqlite3_status64(999, &mut c, &mut hi, 0), 21);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let (mut ci, mut hii): (c_int, c_int) = (0, 0);
    assert_eq!(sqlite3_db_status(db, 999, &mut ci, &mut hii, 0), 1);
    let seed = std::process::id() % 100000;
    let cname = CString::new(format!("rtcoll{seed}")).unwrap();
    unsafe extern "C" fn cmp(_p: *mut c_void, an: c_int, _a: *const c_void, bn: c_int, _b: *const c_void) -> c_int { an - bn }
    sqlite3_create_collation(db, cname.as_ptr(), 1, ptr::null_mut(), Some(cmp));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"PRAGMA collation_list".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), 0);
    assert_eq!(CStr::from_ptr(sqlite3_column_text(st, 1) as *const c_char).to_string_lossy(),
        format!("rtcoll{seed}"), "runtime collation must appear at seq 0");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }
