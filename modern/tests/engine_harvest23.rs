//! Run-33 thin-gap harvest replay — mirrors /tmp/hv_harness.c, asserted
//! byte-identical against the frozen C goldens. A file-level mutex serializes
//! the tests: malloc accounting counters are process-global like C's.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lock() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn nodb(cid: &'static str) -> H { H { cid, lines: Vec::new(), db: ptr::null_mut() } }
    fn ex(&mut self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn exr(&mut self, label: &str, s: &str) { unsafe {
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let e = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={}", self.cid, label, rc, e));
        sqlite3_free(em as *mut c_void);
    } }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn os(&mut self, n: &str, v: Option<String>) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v.unwrap_or_else(|| "NULL".into()))); }
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
        if !self.db.is_null() { unsafe { sqlite3_close(self.db); } }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}

// ---- auto-extension callbacks ----
static GA: Mutex<i64> = Mutex::new(0);
static GB: Mutex<i64> = Mutex::new(0);
unsafe extern "C" fn ext_a(_db: *mut Sqlite3, _e: *mut *mut c_char, _api: *const c_void) -> c_int { *GA.lock().unwrap() += 1; 0 }
unsafe extern "C" fn ext_b(_db: *mut Sqlite3, _e: *mut *mut c_char, _api: *const c_void) -> c_int { *GB.lock().unwrap() += 1; 0 }
type PlainFn = Option<unsafe extern "C" fn()>;
fn fa() -> PlainFn { unsafe { Some(std::mem::transmute(ext_a as unsafe extern "C" fn(*mut Sqlite3, *mut *mut c_char, *const c_void) -> c_int)) } }
fn fb() -> PlainFn { unsafe { Some(std::mem::transmute(ext_b as unsafe extern "C" fn(*mut Sqlite3, *mut *mut c_char, *const c_void) -> c_int)) } }
fn open_close() { unsafe {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    sqlite3_close(db);
} }

#[test] fn a_auto_extension_registry() { let _g = lock(); unsafe {
    // C001..C003 share global registry state — run as one serialized sequence
    *GA.lock().unwrap() = 0; *GB.lock().unwrap() = 0;
    sqlite3_reset_auto_extension();
    let mut h = H::nodb("engine-harvest23-001-C001");
    sqlite3_auto_extension(fa()); sqlite3_auto_extension(fb());
    open_close();
    h.oi("a_calls", *GA.lock().unwrap()); h.oi("b_calls", *GB.lock().unwrap());
    h.check();
    let mut h = H::nodb("engine-harvest23-001-C002");
    h.oi("cancel_a", sqlite3_cancel_auto_extension(fa()) as i64);
    h.oi("cancel_a_again", sqlite3_cancel_auto_extension(fa()) as i64);
    open_close();
    h.oi("a_calls", *GA.lock().unwrap()); h.oi("b_calls", *GB.lock().unwrap());
    h.check();
    let mut h = H::nodb("engine-harvest23-001-C003");
    sqlite3_reset_auto_extension();
    open_close();
    h.oi("a_calls", *GA.lock().unwrap()); h.oi("b_calls", *GB.lock().unwrap());
    sqlite3_auto_extension(fa()); sqlite3_auto_extension(fa());
    open_close();
    h.oi("a_after_dup", *GA.lock().unwrap());
    sqlite3_reset_auto_extension();
    h.check();
} }

#[test] fn b_malloc_accounting() { let _g = lock(); unsafe {
    let mut h = H::nodb("engine-harvest23-002-C001");
    let u0 = sqlite3_memory_used();
    let p = sqlite3_malloc64(3000);
    let u1 = sqlite3_memory_used();
    sqlite3_free(p);
    let u2 = sqlite3_memory_used();
    h.oi("grew", ((u1 - u0) >= 3000) as i64);
    h.oi("back", (u2 == u0) as i64);
    h.check();
    let mut h = H::nodb("engine-harvest23-002-C002");
    sqlite3_memory_highwater(1);
    let p = sqlite3_malloc64(5000);
    let hw = sqlite3_memory_highwater(0);
    sqlite3_free(p);
    let hw2 = sqlite3_memory_highwater(0);
    h.oi("peak_ge", (hw >= 5000) as i64);
    h.oi("sticky", (hw2 == hw) as i64);
    h.check();
    let mut h = H::nodb("engine-harvest23-002-C003");
    let p = sqlite3_malloc64(2000);
    let prior = sqlite3_memory_highwater(1);
    let after = sqlite3_memory_highwater(0);
    h.oi("prior_ge", (prior >= 2000) as i64);
    h.oi("after_le_prior", (after <= prior) as i64);
    sqlite3_free(p);
    h.check();
} }

#[test] fn c_snprintf_str_append() { let _g = lock(); unsafe {
    let mut h = H::nodb("engine-harvest23-003-C001");
    let mut b = [0u8; 64];
    sqlite3_snprintf(64, b.as_mut_ptr() as *mut c_char, c"%d-%s".as_ptr(), 42, c"forty-two".as_ptr());
    h.os("out", Some(CStr::from_ptr(b.as_ptr() as *const c_char).to_string_lossy().into_owned()));
    h.check();
    let mut h = H::nodb("engine-harvest23-003-C002");
    let mut b = [b'X'; 8];
    sqlite3_snprintf(5, b.as_mut_ptr() as *mut c_char, c"%d-%s".as_ptr(), 1234, c"abcdef".as_ptr());
    h.os("out", Some(CStr::from_ptr(b.as_ptr() as *const c_char).to_string_lossy().into_owned()));
    h.oi("nul_at_4", (b[4] == 0) as i64);
    h.check();
    let mut h = H::nodb("engine-harvest23-003-C003");
    let mut b = [0u8; 64];
    sqlite3_snprintf(64, b.as_mut_ptr() as *mut c_char, c"%d %Q".as_ptr(), 7, ptr::null());
    h.os("null_q", Some(CStr::from_ptr(b.as_ptr() as *const c_char).to_string_lossy().into_owned()));
    let mut c2 = [0u8; 64];
    sqlite3_snprintf(64, c2.as_mut_ptr() as *mut c_char, c"%d %q".as_ptr(), 7, c"o'brien".as_ptr());
    h.os("quoted", Some(CStr::from_ptr(c2.as_ptr() as *const c_char).to_string_lossy().into_owned()));
    h.check();
    let mut h = H::nodb("engine-harvest23-003-C004");
    let mut b = *b"keep\0\0\0\0";
    let r = sqlite3_snprintf(0, b.as_mut_ptr() as *mut c_char, c"%d-%s".as_ptr(), 9, c"z".as_ptr());
    h.os("buf", Some(CStr::from_ptr(b.as_ptr() as *const c_char).to_string_lossy().into_owned()));
    h.oi("ret_is_buf", (r == b.as_mut_ptr() as *mut c_char) as i64);
    h.check();
    let mut h = H::nodb("engine-harvest23-003-C005");
    let s = sqlite3_str_new(ptr::null_mut());
    sqlite3_str_append(s, c"hello world".as_ptr(), 5);
    sqlite3_str_append(s, c"!!".as_ptr(), 2);
    let v = sqlite3_str_finish(s);
    h.os("out", Some(CStr::from_ptr(v).to_string_lossy().into_owned()));
    sqlite3_free(v as *mut c_void);
    h.check();
    let mut h = H::nodb("engine-harvest23-003-C006");
    let s = sqlite3_str_new(ptr::null_mut());
    sqlite3_str_appendchar(s, 3, b'=' as c_char);
    sqlite3_str_append(s, c"abcdef".as_ptr(), 3);
    h.oi("len", sqlite3_str_length(s) as i64);
    let v = sqlite3_str_finish(s);
    h.os("out", Some(CStr::from_ptr(v).to_string_lossy().into_owned()));
    sqlite3_free(v as *mut c_void);
    h.check();
} }

fn wdb() -> H {
    let mut h = H::new("x");
    h.ex("CREATE TABLE w(g TEXT, v INT); INSERT INTO w VALUES('a',10),('a',20),('a',30),('b',5),('b',15);");
    h
}

#[test] fn d_window_leftovers() { let _g = lock();
    let mut h = wdb(); h.cid = "engine-harvest23-004-C001";
    h.rows("first_value", "SELECT g, v, first_value(v) OVER (PARTITION BY g ORDER BY v) FROM w ORDER BY g, v");
    h.check();
    let mut h = wdb(); h.cid = "engine-harvest23-004-C002";
    h.rows("last_value", "SELECT g, v, last_value(v) OVER (PARTITION BY g ORDER BY v ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) FROM w ORDER BY g, v");
    h.rows("last_dflt", "SELECT v, last_value(v) OVER (ORDER BY v) FROM w ORDER BY v");
    h.check();
    let mut h = wdb(); h.cid = "engine-harvest23-004-C003";
    h.rows("nth_2", "SELECT g, v, nth_value(v,2) OVER (PARTITION BY g ORDER BY v ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) FROM w ORDER BY g, v");
    h.rows("nth_oor", "SELECT v, nth_value(v,9) OVER (ORDER BY v ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) FROM w ORDER BY v");
    h.check();
    let mut h = wdb(); h.cid = "engine-harvest23-004-C004";
    h.rows("ntile2", "SELECT v, ntile(2) OVER (ORDER BY v) FROM w ORDER BY v");
    h.rows("ntile3", "SELECT v, ntile(3) OVER (ORDER BY v) FROM w ORDER BY v");
    h.check();
    let mut h = wdb(); h.cid = "engine-harvest23-004-C005";
    h.rows("pct_rank", "SELECT v, percent_rank() OVER (ORDER BY v) FROM w ORDER BY v");
    h.check();
    let mut h = wdb(); h.cid = "engine-harvest23-004-C006";
    h.rows("cume_dist", "SELECT v, cume_dist() OVER (ORDER BY v) FROM w ORDER BY v");
    h.rows("combo", "SELECT g, v, first_value(v) OVER pw, cume_dist() OVER pw FROM w WINDOW pw AS (PARTITION BY g ORDER BY v) ORDER BY g, v");
    h.check();
}

#[test] fn e_limit_matrix() { let _g = lock(); unsafe {
    let mut h = H::new("engine-harvest23-005-C001");
    h.oi("length", sqlite3_limit(h.db, 0, -1) as i64);
    h.oi("column", sqlite3_limit(h.db, 2, -1) as i64);
    h.oi("function_arg", sqlite3_limit(h.db, 6, -1) as i64);
    h.oi("attached", sqlite3_limit(h.db, 7, -1) as i64);
    h.oi("variable_number", sqlite3_limit(h.db, 9, -1) as i64);
    let db = h.db; h.db = ptr::null_mut(); h.check();
    let mut h = H { cid: "engine-harvest23-005-C002", lines: Vec::new(), db };
    h.oi("prior_col", sqlite3_limit(h.db, 2, 50) as i64);
    h.oi("get_col", sqlite3_limit(h.db, 2, -1) as i64);
    h.oi("prior_clamp", sqlite3_limit(h.db, 2, 999999) as i64);
    h.oi("get_clamped", sqlite3_limit(h.db, 2, -1) as i64);
    h.oi("att_clamp_prior", sqlite3_limit(h.db, 7, 50) as i64);
    h.oi("att_clamped", sqlite3_limit(h.db, 7, -1) as i64);
    let db = h.db; h.db = ptr::null_mut(); h.check();
    let mut h = H { cid: "engine-harvest23-005-C003", lines: Vec::new(), db };
    sqlite3_limit(h.db, 9, 3);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare_v2(h.db, c"SELECT ?4".as_ptr(), -1, &mut st, ptr::null_mut());
    let e = CStr::from_ptr(sqlite3_errmsg(h.db)).to_string_lossy().into_owned();
    h.lines.push(format!("OBS {} prep rc={} err={}", h.cid, rc, e));
    sqlite3_finalize(st);
    let rc = sqlite3_prepare_v2(h.db, c"SELECT ?3".as_ptr(), -1, &mut st, ptr::null_mut());
    h.oi("ok3", rc as i64);
    sqlite3_finalize(st);
    h.check();
} }

#[test] fn f_errstr_extended() { let _g = lock(); unsafe {
    let mut h = H::nodb("engine-harvest23-006-C001");
    for (n, rc) in [("e0",0),("e1",1),("e5",5),("e14",14),("e19",19),("e21",21),("e23",23),("e25",25),("e100",100),("e101",101),("e787",787),("e2067",2067)] {
        h.os(n, Some(CStr::from_ptr(sqlite3_errstr(rc)).to_string_lossy().into_owned()));
    }
    h.check();
    let mut h = H::new("engine-harvest23-006-C002");
    h.ex("CREATE TABLE t(a UNIQUE, b NOT NULL DEFAULT 1, c CHECK(c>0)); INSERT INTO t VALUES(1,1,1);");
    h.exr("uniq", "INSERT INTO t VALUES(1,1,1);"); let x = sqlite3_extended_errcode(h.db); h.oi("uniq_ext", x as i64);
    h.exr("notnull", "INSERT INTO t(a,b,c) VALUES(2,NULL,1);"); let x = sqlite3_extended_errcode(h.db); h.oi("nn_ext", x as i64);
    h.exr("check", "INSERT INTO t VALUES(3,1,-5);"); let x = sqlite3_extended_errcode(h.db); h.oi("ck_ext", x as i64);
    h.check();
    let mut h = H::new("engine-harvest23-006-C003");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(x REFERENCES p(id));");
    h.exr("fk", "INSERT INTO c VALUES(9);"); let x = sqlite3_extended_errcode(h.db); h.oi("fk_ext", x as i64);
    h.check();
} }

#[test] fn g_deferred_fk() { let _g = lock(); unsafe {
    let mut h = H::new("engine-harvest23-007-C001");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY);CREATE TABLE c(x REFERENCES p(id) DEFERRABLE INITIALLY DEFERRED);");
    h.exr("begin", "BEGIN;");
    h.exr("orphan", "INSERT INTO c VALUES(5);");
    h.exr("commit1", "COMMIT;");
    h.oi("still_in_txn", (sqlite3_get_autocommit(h.db) == 0) as i64);
    h.exr("parent", "INSERT INTO p VALUES(5);");
    h.exr("commit2", "COMMIT;");
    h.rows("final", "SELECT (SELECT count(*) FROM p), (SELECT count(*) FROM c)");
    h.check();
    let mut h = H::new("engine-harvest23-007-C002");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY);CREATE TABLE c(x REFERENCES p(id) DEFERRABLE INITIALLY DEFERRED);");
    h.ex("BEGIN; INSERT INTO c VALUES(7);");
    h.exr("fix", "DELETE FROM c WHERE x=7;");
    h.exr("commit", "COMMIT;");
    h.rows("final", "SELECT count(*) FROM c");
    h.check();
    let mut h = H::new("engine-harvest23-007-C003");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(x REFERENCES p(id));");
    h.ex("BEGIN;");
    h.ex("PRAGMA defer_foreign_keys=1;");
    h.exr("orphan", "INSERT INTO c VALUES(3);");
    h.exr("parent", "INSERT INTO p VALUES(3);");
    h.exr("commit", "COMMIT;");
    h.rows("pragma_after", "PRAGMA defer_foreign_keys");
    h.rows("final", "SELECT count(*) FROM c");
    h.check();
    let mut h = H::new("engine-harvest23-007-C004");
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY);CREATE TABLE c(x REFERENCES p(id) DEFERRABLE INITIALLY DEFERRED);");
    h.exr("orphan", "INSERT INTO c VALUES(4);");
    h.rows("cnt", "SELECT count(*) FROM c");
    h.check();
} }

// ---- authorizer ----
static DENYCODE: Mutex<i32> = Mutex::new(-1);
unsafe extern "C" fn auth_cb(_a: *mut c_void, code: c_int, _s1: *const c_char, _s2: *const c_char, _s3: *const c_char, _s4: *const c_char) -> c_int {
    if code == *DENYCODE.lock().unwrap() { 1 /* SQLITE_DENY */ } else { 0 }
}

#[test] fn h_auth_codes() { let _g = lock(); unsafe {
    let mut h = H::new("engine-harvest23-008-C001");
    h.ex("CREATE TABLE t(a);");
    *DENYCODE.lock().unwrap() = 18; // SQLITE_INSERT
    sqlite3_set_authorizer(h.db, Some(auth_cb), ptr::null_mut());
    h.exr("ins", "INSERT INTO t VALUES(1);");
    *DENYCODE.lock().unwrap() = -1;
    h.exr("ins_ok", "INSERT INTO t VALUES(1);");
    h.rows("cnt", "SELECT count(*) FROM t");
    h.check();
    let mut h = H::new("engine-harvest23-008-C002");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1);");
    sqlite3_set_authorizer(h.db, Some(auth_cb), ptr::null_mut());
    *DENYCODE.lock().unwrap() = 23; h.exr("upd", "UPDATE t SET a=2;");
    *DENYCODE.lock().unwrap() = 9;  h.exr("del", "DELETE FROM t;");
    *DENYCODE.lock().unwrap() = -1; h.rows("row", "SELECT a FROM t");
    h.check();
    let mut h = H::new("engine-harvest23-008-C003");
    sqlite3_set_authorizer(h.db, Some(auth_cb), ptr::null_mut());
    *DENYCODE.lock().unwrap() = 2;  h.exr("crt", "CREATE TABLE nope(a);");
    *DENYCODE.lock().unwrap() = 19; h.exr("prag", "PRAGMA cache_size;");
    *DENYCODE.lock().unwrap() = -1; h.exr("crt_ok", "CREATE TABLE yes(a);");
    h.check();
} }

#[test] fn i_complete_nesting() { let _g = lock(); unsafe {
    let c = |s: &str| -> i64 { sqlite3_complete(CString::new(s).unwrap().as_ptr()) as i64 };
    let mut h = H::nodb("engine-harvest23-009-C001");
    h.oi("open_case", c("CREATE TRIGGER r AFTER INSERT ON t BEGIN SELECT CASE WHEN 1 THEN 1 END;"));
    h.oi("closed", c("CREATE TRIGGER r AFTER INSERT ON t BEGIN SELECT CASE WHEN 1 THEN 1 END; END;"));
    h.oi("plain_body", c("CREATE TRIGGER r AFTER INSERT ON t BEGIN UPDATE t SET a=1; END;"));
    h.check();
    let mut h = H::nodb("engine-harvest23-009-C002");
    h.oi("two_stmts_open", c("CREATE TRIGGER r BEFORE DELETE ON t BEGIN DELETE FROM u; INSERT INTO v VALUES(1);"));
    h.oi("two_stmts_done", c("CREATE TRIGGER r BEFORE DELETE ON t BEGIN DELETE FROM u; INSERT INTO v VALUES(1); END;"));
    h.oi("no_trigger", c("SELECT 1;"));
    h.check();
} }

// ---- run-33 anti-cheat: runtime values through the new surfaces ----
#[test] fn anti_cheat_harvest_runtime() { let _g = lock(); unsafe {
    let seed = (std::process::id() % 769 + 31) as i64;
    // window functions over runtime rows
    let mut h = H::new("x");
    h.ex(&format!("CREATE TABLE r(v INT); INSERT INTO r VALUES({}),({}),({});", seed, seed * 2, seed * 3));
    h.rows("q", "SELECT v, first_value(v) OVER (ORDER BY v), ntile(2) OVER (ORDER BY v) FROM r ORDER BY v");
    assert_eq!(h.lines.pop().unwrap(),
        format!("OBS x q {s},{s},1|{d},{s},1|{t},{s},2", s = seed, d = seed * 2, t = seed * 3));
    // limit enforcement with a runtime bound
    let lim = (seed % 5 + 2) as i32;
    sqlite3_limit(h.db, 9, lim);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let rc = sqlite3_prepare_v2(h.db, CString::new(format!("SELECT ?{}", lim + 1)).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(rc, 1);
    let e = CStr::from_ptr(sqlite3_errmsg(h.db)).to_string_lossy().into_owned();
    assert_eq!(e, format!("variable number must be between ?1 and ?{lim}"));
    // deferred FK with a runtime key
    h.ex("PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(x REFERENCES p(id) DEFERRABLE INITIALLY DEFERRED);");
    h.ex(&format!("BEGIN; INSERT INTO c VALUES({seed});"));
    h.exr("cm", "COMMIT;");
    assert!(h.lines.pop().unwrap().contains("rc=19"));
    h.ex(&format!("INSERT INTO p VALUES({seed});"));
    h.exr("cm2", "COMMIT;");
    assert!(h.lines.pop().unwrap().contains("rc=0"));
    sqlite3_close(h.db);
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
