//! Run-42 compile-option diagnostics replay — mirrors /tmp/copt_harness.c.
//! Plain language: ask whether a compile flag is on (sqlite3_compileoption_used, C + SQL)
//! and list the flags this build knows (sqlite3_compileoption_get). Modern answers from
//! the pinned bare-amalgamation fingerprint (38 entries; ADR 0030).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn used(&mut self, label: &str, opt: &str) { unsafe {
        let v = sqlite3_compileoption_used(CString::new(opt).unwrap().as_ptr());
        self.lines.push(format!("OBS {} {} used={}", self.cid, label, v));
    } }
    fn get1(&mut self, label: &str, n: c_int) { unsafe {
        let z = sqlite3_compileoption_get(n);
        let s = if z.is_null() { "NULL".to_string() } else { CStr::from_ptr(z).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} get={}", self.cid, label, s));
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
    fn obs(&mut self, tail: String) { self.lines.push(format!("OBS {} {}", self.cid, tail)); }
    fn check(self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-compile32/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}

// ================= A — diagnostics API (engine-compile32-001) =================

#[test] fn a001() { let mut h = H::new("engine-compile32-001-C001");
    h.used("plain", "THREADSAFE");
    h.used("prefixed", "SQLITE_THREADSAFE");
    h.used("valued", "THREADSAFE=1");
    h.used("wrong_value", "THREADSAFE=0");
    h.check(); }

#[test] fn a002() { let mut h = H::new("engine-compile32-001-C002");
    h.used("unknown", "NOT_A_REAL_OPTION_XYZ");
    h.used("empty", "");
    h.used("prefix_only", "SQLITE_");
    h.check(); }

#[test] fn a003() { let mut h = H::new("engine-compile32-001-C003");
    h.get1("first", 0);
    h.get1("second", 1);
    h.get1("last", 37);
    h.get1("past_end", 38);
    h.get1("far", 999);
    h.get1("negative", -1);
    h.check(); }

#[test] fn a004() { let mut h = H::new("engine-compile32-001-C004"); unsafe {
    let mut list: Vec<String> = Vec::new();
    let mut i: c_int = 0;
    loop {
        let z = sqlite3_compileoption_get(i);
        if z.is_null() { break; }
        list.push(CStr::from_ptr(z).to_string_lossy().into_owned());
        i += 1;
    }
    h.obs(format!("all n={} list={}", i, list.join("|")));
    } h.check(); }

#[test] fn a005() { let mut h = H::new("engine-compile32-001-C005");
    h.rows("sql_used", "SELECT sqlite_compileoption_used('THREADSAFE'), sqlite_compileoption_used('NOT_A_REAL_OPTION_XYZ')");
    h.check(); }

#[test] fn a006() { let mut h = H::new("engine-compile32-001-C006");
    h.rows("sql_get", "SELECT sqlite_compileoption_get(0), sqlite_compileoption_get(37), sqlite_compileoption_get(99)");
    h.check(); }

#[test] fn a007() { let mut h = H::new("engine-compile32-001-C007");
    h.rows("sql_prefix", "SELECT sqlite_compileoption_used('SQLITE_THREADSAFE'), sqlite_compileoption_used('SQLITE_THREADSAFE=1'), sqlite_compileoption_used('SQLITE_NOT_REAL')");
    h.check(); }

#[test] fn a008() { let mut h = H::new("engine-compile32-001-C008");
    h.used("max_attached", "MAX_ATTACHED");
    h.used("max_attached_val", "MAX_ATTACHED=10");
    h.used("max_var", "MAX_VARIABLE_NUMBER=32766");
    h.used("temp_store", "TEMP_STORE=1");
    h.check(); }

#[test] fn a009() { let mut h = H::new("engine-compile32-001-C009");
    h.used("lower", "threadsafe");
    h.used("mixed", "ThreadSafe=1");
    h.used("lower_prefixed", "sqlite_threadsafe");
    h.check(); }

#[test] fn a011() { let mut h = H::new("engine-compile32-001-C011");
    h.rows("sql_typeof", "SELECT typeof(sqlite_compileoption_used('THREADSAFE')), typeof(sqlite_compileoption_get(0)), typeof(sqlite_compileoption_get(999))");
    h.check(); }

#[test] fn a012() { let mut h = H::new("engine-compile32-001-C012");
    h.used("bare_gate", "DEFAULT_AUTOVACUUM");
    h.used("bare_gate_valued", "DEFAULT_AUTOVACUUM=1");
    h.used("compiler_row", "COMPILER=gcc-13.3.0");
    h.used("compiler_bare", "COMPILER");
    h.check(); }

// ================= B — OMIT census (engine-compile32-002) =================

#[test] fn b001() { let mut h = H::new("engine-compile32-002-C001");
    h.used("load_ext", "OMIT_LOAD_EXTENSION");
    h.used("wal", "OMIT_WAL");
    h.used("vtab", "OMIT_VIRTUALTABLE");
    h.used("trigger", "OMIT_TRIGGER");
    h.check(); }

#[test] fn b002() { let mut h = H::new("engine-compile32-002-C002");
    h.used("autoreset", "OMIT_AUTORESET");
    h.used("diags", "OMIT_COMPILEOPTION_DIAGS");
    h.check(); }

#[test] fn b003() { let mut h = H::new("engine-compile32-002-C003"); unsafe {
    let mut c = 0; let mut i: c_int = 0;
    loop {
        let z = sqlite3_compileoption_get(i);
        if z.is_null() { break; }
        if CStr::from_ptr(z).to_string_lossy().starts_with("OMIT_") { c += 1; }
        i += 1;
    }
    h.obs(format!("omit_prefix_count {c}"));
    } h.check(); }

#[test] fn b004() { let mut h = H::new("engine-compile32-002-C004");
    h.used("attach", "OMIT_ATTACH");
    h.used("subquery", "OMIT_SUBQUERY");
    h.used("view", "OMIT_VIEW");
    h.rows("sql_omit", "SELECT sqlite_compileoption_used('OMIT_WAL'), sqlite_compileoption_used('OMIT_VIRTUALTABLE')");
    h.check(); }

// ================= C — ENABLE census (engine-compile32-003) =================

#[test] fn c001() { let mut h = H::new("engine-compile32-003-C001");
    h.used("fts5", "ENABLE_FTS5");
    h.used("fts3", "ENABLE_FTS3");
    h.used("rtree", "ENABLE_RTREE");
    h.used("geopoly", "ENABLE_GEOPOLY");
    h.check(); }

#[test] fn c002() { let mut h = H::new("engine-compile32-003-C002");
    h.used("stat4", "ENABLE_STAT4");
    h.used("api_armor", "ENABLE_API_ARMOR");
    h.used("unlock_notify", "ENABLE_UNLOCK_NOTIFY");
    h.used("session", "ENABLE_SESSION");
    h.check(); }

#[test] fn c003() { let mut h = H::new("engine-compile32-003-C003"); unsafe {
    let mut c = 0; let mut i: c_int = 0;
    loop {
        let z = sqlite3_compileoption_get(i);
        if z.is_null() { break; }
        if CStr::from_ptr(z).to_string_lossy().starts_with("ENABLE_") { c += 1; }
        i += 1;
    }
    h.obs(format!("enable_prefix_count {c}"));
    } h.check(); }

#[test] fn c004() { let mut h = H::new("engine-compile32-003-C004");
    h.rows("sql_enable", "SELECT sqlite_compileoption_used('ENABLE_FTS5'), sqlite_compileoption_used('ENABLE_STAT4')");
    h.check(); }

// ================= D — anti-cheat =================

#[test] fn anti_cheat_compile32_runtime_option() { unsafe {
    // runtime-generated option name: no canned table can know it; must report 0
    // through BOTH the C API and the SQL twin.
    let seed = std::process::id() % 100000;
    let fake = format!("TOTALLY_FAKE_OPTION_{seed}");
    assert_eq!(sqlite3_compileoption_used(CString::new(fake.clone()).unwrap().as_ptr()), 0);
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT sqlite_compileoption_used('{fake}')")).unwrap().as_ptr(),
        -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), 0);
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_compile32_enumeration_roundtrip() { unsafe {
    // C-API enumeration and the SQL twin must agree entry-for-entry at runtime,
    // terminate NULL at the same index, and every enumerated entry must report used()=1.
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let mut i: c_int = 0;
    loop {
        let z = sqlite3_compileoption_get(i);
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db, CString::new(format!("SELECT sqlite_compileoption_get({i})")).unwrap().as_ptr(),
            -1, &mut st, ptr::null_mut());
        assert_eq!(sqlite3_step(st), 100);
        let sql_z = sqlite3_column_text(st, 0);
        if z.is_null() {
            assert!(sql_z.is_null(), "SQL twin must terminate at the same index {i}");
            sqlite3_finalize(st);
            break;
        }
        let copt = CStr::from_ptr(z).to_string_lossy().into_owned();
        assert_eq!(CStr::from_ptr(sql_z as *const c_char).to_string_lossy(), copt, "entry {i}");
        assert_eq!(sqlite3_compileoption_used(z), 1, "enumerated entry must be used(): {copt}");
        sqlite3_finalize(st);
        i += 1;
    }
    assert_eq!(i, 38, "pinned fingerprint length");
    sqlite3_close(db);
} }
