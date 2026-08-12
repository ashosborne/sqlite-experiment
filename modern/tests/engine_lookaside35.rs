//! Run-45 lookaside replay — mirrors /tmp/la_harness.c.
//! Plain language: small per-connection allocations (modern: prepared-statement
//! objects) come from a fixed lookaside slot pool before the general heap; db_config
//! knobs follow C's BUSY/normalize/disable rules and LOOKASIDE db_status counters
//! move because of that real pool.
use sqlite3_rust_spine::*;
use std::ffi::CString;
use std::os::raw::c_int;
use std::path::PathBuf;
use std::ptr;

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn seed_table(&mut self) {
        self.ex("CREATE TABLE t(a,b);");
        self.ex("INSERT INTO t VALUES(1,'x'),(2,'y');");
    }
    fn ex(&mut self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn oi(&mut self, label: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, label, v)); }
    fn dst(&mut self, op: c_int, reset: c_int) -> (c_int, c_int) { unsafe {
        let (mut c, mut h): (c_int, c_int) = (0, 0);
        sqlite3_db_status(self.db, op, &mut c, &mut h, reset);
        (c, h)
    } }
    fn cfg(&mut self, sz: i64, cnt: i64) -> c_int { unsafe {
        sqlite3_db_config(self.db, SQLITE_DBCONFIG_LOOKASIDE, 0, sz, cnt)
    } }
    fn prep(&mut self, sql: &str) -> *mut Sqlite3Stmt { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        st
    } }
    fn cycle(&mut self, sql: &str, step: bool) { unsafe {
        let st = self.prep(sql);
        if step { sqlite3_step(st); }
        sqlite3_finalize(st);
    } }
    fn check(self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-lookaside35/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}

// ================= A — enable + config (engine-lookaside35-001) =================

#[test] fn a001() { let mut h = H::new("engine-lookaside35-001-C001");
    let (c, hi) = h.dst(0, 0); h.oi("used_fresh_zero", (c == 0 && hi == 0) as i64);
    let (c, hi) = h.dst(4, 0); h.oi("hit_fresh_zero", (c == 0 && hi == 0) as i64);
    let rc = h.cfg(256, 16); h.oi("cfg_fresh_rc", rc as i64);
    h.check(); }

#[test] fn a002() { let mut h = H::new("engine-lookaside35-001-C002"); h.seed_table();
    let st = h.prep("SELECT a FROM t");
    let (c, _hi) = h.dst(0, 0); h.oi("used_live_pos", (c > 0) as i64);
    let rc = h.cfg(512, 32); h.oi("cfg_busy_rc", rc as i64);
    unsafe { sqlite3_finalize(st); }
    let rc = h.cfg(512, 32); h.oi("cfg_after_fin_rc", rc as i64);
    h.check(); }

#[test] fn a003() { let mut h = H::new("engine-lookaside35-001-C003");
    let rc = h.cfg(-5, 10); h.oi("neg_sz_rc", rc as i64);
    let (c, hi) = h.dst(0, 0); h.oi("used_after_negsz_zero", (c == 0 && hi == 0) as i64);
    let rc = h.cfg(256, -3); h.oi("neg_cnt_rc", rc as i64);
    let (c, hi) = h.dst(0, 0); h.oi("used_after_negcnt_zero", (c == 0 && hi == 0) as i64);
    let rc = h.cfg(100000, 4); h.oi("huge_sz_rc", rc as i64);
    h.check(); }

#[test] fn a004() { let mut h = H::new("engine-lookaside35-001-C004");
    let mut v: c_int = 0;
    let rc = unsafe { sqlite3_db_config(h.db, 9999, &mut v as *mut c_int as i64, 0, 0) };
    h.oi("unknown_op_rc", rc as i64);
    h.check(); }

#[test] fn a005() { let mut h = H::new("engine-lookaside35-001-C005"); h.seed_table();
    let rc = h.cfg(0, 0); h.oi("disable_rc", rc as i64);
    let (c, hi) = h.dst(0, 0); h.oi("used_disabled_zero", (c == 0 && hi == 0) as i64);
    h.dst(4, 1); // clear HIT, then traffic while disabled
    for _ in 0..3 { h.cycle("SELECT a FROM t", false); }
    let (_c, hi) = h.dst(4, 0); h.oi("hit_frozen_while_disabled", (hi == 0) as i64);
    h.check(); }

// ================= B — pool behaviour + fallback (engine-lookaside35-002) =================

#[test] fn b001() { let mut h = H::new("engine-lookaside35-002-C001"); h.seed_table();
    h.cfg(1200, 40);
    h.dst(4, 1);
    for _ in 0..5 { h.cycle("SELECT a, b FROM t WHERE a > 0", true); }
    let (c, hi) = h.dst(4, 0);
    h.oi("hit_grew", (hi > 0) as i64); h.oi("hit_cur_always_zero", (c == 0) as i64);
    let (c, _hi) = h.dst(0, 0); h.oi("used_back_to_zero", (c == 0) as i64);
    h.check(); }

#[test] fn b002() { let mut h = H::new("engine-lookaside35-002-C002"); h.seed_table();
    h.cfg(64, 16);
    h.dst(5, 1);
    for _ in 0..3 { h.cycle("SELECT a, b FROM t WHERE a > 0", true); }
    let (c, hi) = h.dst(5, 0);
    h.oi("miss_size_grew", (hi > 0) as i64); h.oi("miss_size_cur_zero", (c == 0) as i64);
    h.check(); }

#[test] fn b003() { let mut h = H::new("engine-lookaside35-002-C003"); h.seed_table();
    h.cfg(512, 2);
    h.dst(6, 1);
    let sts: Vec<*mut Sqlite3Stmt> = (0..6).map(|_| h.prep("SELECT a, b FROM t WHERE a > 0")).collect();
    let (c, _hi) = h.dst(0, 0); h.oi("used_live_pos", (c > 0) as i64);
    let (_c, hi) = h.dst(6, 0); h.oi("miss_full_grew", (hi > 0) as i64);
    for st in sts { unsafe { sqlite3_finalize(st); } }
    let (c, _hi) = h.dst(0, 0); h.oi("used_drained_zero", (c == 0) as i64);
    h.check(); }

#[test] fn b004() { let mut h = H::new("engine-lookaside35-002-C004"); h.seed_table();
    h.cfg(512, 4);
    h.dst(4, 1);
    h.cycle("SELECT a FROM t", false);
    let (_c, h1) = h.dst(4, 0);
    h.cycle("SELECT b FROM t", false);
    let (_c, h2) = h.dst(4, 0);
    h.oi("first_cycle_hit", (h1 > 0) as i64); h.oi("slot_reused_hit_again", (h2 > h1) as i64);
    let (c, _hi) = h.dst(0, 0); h.oi("used_zero_after_cycles", (c == 0) as i64);
    h.check(); }

#[test] fn b005() { let mut h = H::new("engine-lookaside35-002-C005"); h.seed_table();
    h.cfg(512, 8);
    let sts: Vec<*mut Sqlite3Stmt> = (0..3).map(|_| h.prep("SELECT a FROM t")).collect();
    h.dst(0, 1); // reset USED highwater to current while live
    let (c2, h2) = h.dst(0, 0);
    h.oi("used_reset_hi_eq_cur", (h2 == c2) as i64); h.oi("used_live_pos", (c2 > 0) as i64);
    for st in sts { unsafe { sqlite3_finalize(st); } }
    h.dst(4, 1);
    let (_c, hi) = h.dst(4, 0); h.oi("hit_reset_zero", (hi == 0) as i64);
    h.check(); }

// ================= C — status coupling / regression (engine-lookaside35-003) =================

#[test] fn c001() { let mut h = H::new("engine-lookaside35-003-C001"); h.seed_table();
    // DEFAULT lookaside config — no db_config call at all
    h.dst(4, 1);
    for _ in 0..4 { h.cycle("SELECT a FROM t WHERE a = 1", true); }
    let (_c, hi) = h.dst(4, 0); h.oi("default_pool_hits", (hi > 0) as i64);
    h.check(); }

#[test] fn c002() { let mut h = H::new("engine-lookaside35-003-C002"); h.seed_table();
    let (mut mc, mut mh): (i64, i64) = (0, 0);
    unsafe { sqlite3_status64(0, &mut mc, &mut mh, 0); }
    h.oi("memory_used_sane", (mc >= 0 && mh >= mc) as i64);
    let (c, hi) = h.dst(2, 0);
    h.oi("schema_used_pos", (c > 0) as i64); h.oi("schema_hi_zero", (hi == 0) as i64);
    h.check(); }

#[test] fn c003() { let mut h = H::new("engine-lookaside35-003-C003"); h.seed_table();
    h.cycle("SELECT a FROM t", true);
    let (c, hi) = h.dst(5, 0); h.oi("default_no_miss_size", (c == 0 && hi == 0) as i64);
    let (c, hi) = h.dst(6, 0); h.oi("default_no_miss_full", (c == 0 && hi == 0) as i64);
    h.check(); }

#[test] fn c004() { let mut h = H::new("engine-lookaside35-003-C004"); h.seed_table();
    let st = h.prep("SELECT a FROM t");
    let (c, _hi) = h.dst(4, 0); h.oi("hit_cur_zero_under_traffic", (c == 0) as i64);
    h.dst(0, 0);
    let rc = unsafe {
        let (mut c2, mut h2): (c_int, c_int) = (0, 0);
        sqlite3_db_status(h.db, 9999, &mut c2, &mut h2, 0)
    };
    h.oi("badop_rc_still_error", rc as i64);
    unsafe { sqlite3_finalize(st); }
    h.check(); }

// ================= anti-cheat =================

#[test] fn anti_cheat_lookaside35_runtime_pool() { unsafe {
    // runtime-chosen slot count: with N slots and N+K live statements, exactly N are
    // pool-served (USED current == N) and the overflow misses — impossible to can.
    let seed = (std::process::id() % 5) as i64 + 2; // 2..=6 slots
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_exec(db, c"CREATE TABLE t(a);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    assert_eq!(sqlite3_db_config(db, SQLITE_DBCONFIG_LOOKASIDE, 0, 512, seed), 0);
    let total = seed + 3;
    let sts: Vec<*mut Sqlite3Stmt> = (0..total).map(|_| {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
        st
    }).collect();
    let (mut c, mut hi): (c_int, c_int) = (0, 0);
    sqlite3_db_status(db, 0, &mut c, &mut hi, 0);
    assert_eq!(c as i64, seed, "exactly the runtime slot count is pool-served");
    sqlite3_db_status(db, 6, &mut c, &mut hi, 0);
    assert_eq!(hi as i64, 3, "the overflow beyond the runtime pool misses FULL");
    for st in sts { sqlite3_finalize(st); }
    sqlite3_db_status(db, 0, &mut c, &mut hi, 0);
    assert_eq!(c, 0, "drained pool");
    sqlite3_close(db);
} }

#[test] fn anti_cheat_lookaside35_runtime_size() { unsafe {
    // runtime-chosen slot size below the statement object size: every prepare must
    // MISS_SIZE (heap fallback) and HIT must stay frozen.
    let seed = (std::process::id() % 4) as i64 * 8 + 16; // 16..40 bytes, all too small
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_exec(db, c"CREATE TABLE t(a);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    assert_eq!(sqlite3_db_config(db, SQLITE_DBCONFIG_LOOKASIDE, 0, seed, 8), 0);
    let (mut c, mut hi): (c_int, c_int) = (0, 0);
    sqlite3_db_status(db, 4, &mut c, &mut hi, 1);
    sqlite3_db_status(db, 5, &mut c, &mut hi, 1);
    let reps = std::process::id() % 3 + 2;
    for _ in 0..reps {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
        sqlite3_finalize(st);
    }
    sqlite3_db_status(db, 5, &mut c, &mut hi, 0);
    assert_eq!(hi as u32, reps, "every runtime prepare misses on size");
    sqlite3_db_status(db, 4, &mut c, &mut hi, 0);
    assert_eq!(hi, 0, "no hits with too-small slots");
    sqlite3_close(db);
} }

#[test] fn anti_cheat_lookaside35_bad_forms() { unsafe {
    // bad ops still fail like C — no silent success anywhere in the new paths
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let (mut c, mut hi): (c_int, c_int) = (0, 0);
    assert_eq!(sqlite3_db_status(db, 9999, &mut c, &mut hi, 0), 1);
    assert_eq!(sqlite3_db_config(db, 9999, 0, 0, 0), 1);
    // BUSY protects a live pool from reconfiguration
    sqlite3_exec(db, c"CREATE TABLE t(a);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, c"SELECT a FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_db_config(db, SQLITE_DBCONFIG_LOOKASIDE, 0, 256, 8), 5);
    sqlite3_finalize(st);
    assert_eq!(sqlite3_db_config(db, SQLITE_DBCONFIG_LOOKASIDE, 0, 256, 8), 0);
    sqlite3_close(db);
} }
