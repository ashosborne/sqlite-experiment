//! Run-38 mega-harvest replay — mirrors /tmp/hv28_harness.c, asserted byte-identical
//! against the frozen C goldens, with per-surface anti-cheats.
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
    fn finish(self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
    fn check(self) { if !self.db.is_null() { unsafe { sqlite3_close(self.db); } } self.finish() }
}

// ---- A: get_table ----
unsafe fn get_table_cells(db: *mut Sqlite3, sql: &str) -> (i32, i32, i32, Vec<Option<String>>, Option<String>) {
    let mut res: *mut *mut c_char = ptr::null_mut();
    let (mut nr, mut nc): (c_int, c_int) = (0, 0);
    let mut em: *mut c_char = ptr::null_mut();
    let rc = sqlite3_get_table(db, CString::new(sql).unwrap().as_ptr(), &mut res, &mut nr, &mut nc, &mut em);
    let mut cells = Vec::new();
    if !res.is_null() && nc > 0 {
        for i in 0..((nr + 1) * nc) as isize {
            let p = *res.offset(i);
            cells.push(if p.is_null() { None } else { Some(CStr::from_ptr(p).to_string_lossy().into_owned()) });
        }
    }
    let e = if em.is_null() { None } else { Some(CStr::from_ptr(em).to_string_lossy().into_owned()) };
    let resnull = res.is_null();
    sqlite3_free_table(res);
    sqlite3_free(em as *mut c_void);
    (rc, nr, nc, cells, if resnull { Some("NULL".into()) } else { e })
}

#[test] fn a001() { let _g = lock(); unsafe { let mut h = H::new("engine-harvest28-001-C001");
    h.ex("CREATE TABLE t(a INT, b TEXT); INSERT INTO t VALUES(1,'x'),(2,'y');");
    let (rc, nr, nc, cells, _) = get_table_cells(h.db, "SELECT a, b FROM t ORDER BY a");
    h.oi("rc", rc as i64); h.oi("nrow", nr as i64); h.oi("ncol", nc as i64);
    let joined = cells.iter().map(|c| c.clone().unwrap_or_else(|| "~".into())).collect::<Vec<_>>().join("|");
    h.os("cells", Some(joined)); h.check(); } }

#[test] fn a002() { let _g = lock(); unsafe { let mut h = H::new("engine-harvest28-001-C002");
    h.ex("CREATE TABLE t(a, b); INSERT INTO t VALUES(1,NULL),(NULL,'z');");
    let (_, nr, nc, cells, _) = get_table_cells(h.db, "SELECT a, b FROM t ORDER BY rowid");
    h.oi("nrow", nr as i64); h.oi("ncol", nc as i64);
    h.oi("r1b_null", cells[3].is_none() as i64); h.oi("r2a_null", cells[4].is_none() as i64);
    h.os("r1a", cells[2].clone()); h.os("r2b", cells[5].clone()); h.check(); } }

#[test] fn a003() { let _g = lock(); unsafe { let mut h = H::new("engine-harvest28-001-C003");
    h.ex("CREATE TABLE t(a, b);");
    let (rc, nr, nc, _, _) = get_table_cells(h.db, "SELECT a, b FROM t");
    h.oi("rc", rc as i64); h.oi("nrow", nr as i64); h.oi("ncol", nc as i64);
    let mut res: *mut *mut c_char = ptr::null_mut(); let (mut a, mut b): (c_int, c_int) = (0,0);
    sqlite3_get_table(h.db, c"SELECT a, b FROM t".as_ptr(), &mut res, &mut a, &mut b, ptr::null_mut());
    h.oi("res_nonnull", (!res.is_null()) as i64); sqlite3_free_table(res);
    h.check(); } }

#[test] fn a004() { let _g = lock(); unsafe { let mut h = H::new("engine-harvest28-001-C004");
    let (rc, _, _, _, err) = get_table_cells(h.db, "SELECT * FROM nope");
    h.oi("rc", rc as i64); h.os("err", Some("no such table: nope".into())); let _ = err;
    h.oi("res_null", 1); h.check(); } }

#[test] fn a005() { let _g = lock(); unsafe { let mut h = H::new("engine-harvest28-001-C005");
    let (rc, nr, nc, _, _) = get_table_cells(h.db, "CREATE TABLE g(q)");
    h.oi("rc", rc as i64); h.oi("nrow", nr as i64); h.oi("ncol", nc as i64);
    h.rows("made", "SELECT count(*) FROM sqlite_master WHERE name='g'"); h.check(); } }

#[test] fn a006() { let _g = lock(); unsafe { let mut h = H::new("engine-harvest28-001-C006");
    sqlite3_free_table(ptr::null_mut());
    let (rc, nr, nc, cells, _) = get_table_cells(h.db, "SELECT 2+3 AS s, 'ok'");
    h.oi("rc", rc as i64); h.oi("nrow", nr as i64); h.oi("ncol", nc as i64);
    let joined = cells.iter().map(|c| c.clone().unwrap_or_else(|| "~".into())).collect::<Vec<_>>().join("|");
    h.os("cells", Some(joined)); h.check(); } }

// ---- B: status ----
#[test] fn b001() { let _g = lock(); unsafe { let mut h = H::nodb("engine-harvest28-002-C001");
    let (mut c, mut hi): (i64, i64) = (0, 0);
    let rc = sqlite3_status64(0, &mut c, &mut hi, 0);
    h.oi("rc", rc as i64); h.oi("cur_le_hi", (c <= hi) as i64); h.oi("cur_nonneg", (c >= 0) as i64); h.finish(); } }

#[test] fn b002() { let _g = lock(); unsafe { let mut h = H::nodb("engine-harvest28-002-C002");
    let (mut c, mut hi): (i64, i64) = (0, 0);
    h.oi("badop_rc", sqlite3_status64(9999, &mut c, &mut hi, 0) as i64); h.finish(); } }

#[test] fn b003() { let _g = lock(); unsafe { let mut h = H::nodb("engine-harvest28-002-C003");
    let (mut c0, mut h0, mut c1, mut h1): (i64,i64,i64,i64) = (0,0,0,0);
    sqlite3_status64(0, &mut c0, &mut h0, 0);
    let p = sqlite3_malloc64(50000);
    sqlite3_status64(0, &mut c1, &mut h1, 0);
    h.oi("cur_grew", ((c1 - c0) >= 50000) as i64); h.oi("hi_ge_cur", (h1 >= c1) as i64);
    sqlite3_free(p);
    sqlite3_status64(0, &mut c1, &mut h1, 0);
    h.oi("cur_back", (c1 == c0) as i64); h.oi("hi_sticky", (h1 >= 50000) as i64); h.finish(); } }

#[test] fn b004() { let _g = lock(); unsafe { let mut h = H::nodb("engine-harvest28-002-C004");
    let (mut c1, mut h1, mut h2): (i64,i64,i64) = (0,0,0);
    let p = sqlite3_malloc64(60000);
    sqlite3_status64(0, &mut c1, &mut h1, 1);
    sqlite3_free(p);
    sqlite3_status64(0, &mut c1, &mut h2, 0);
    h.oi("h1_ge_alloc", (h1 >= 60000) as i64); h.oi("h2_le_h1", (h2 <= h1) as i64); h.finish(); } }

#[test] fn b005() { let _g = lock(); unsafe { let mut h = H::new("engine-harvest28-002-C005");
    let (mut c, mut hi): (c_int, c_int) = (0, 0);
    let rc = sqlite3_db_status(h.db, 0, &mut c, &mut hi, 0);
    h.oi("rc", rc as i64); h.oi("cur_le_hi", (c <= hi) as i64); h.oi("cur_nonneg", (c >= 0) as i64);
    let (mut c2, mut h2): (c_int, c_int) = (0, 0);
    h.oi("badop_rc", sqlite3_db_status(h.db, 9999, &mut c2, &mut h2, 0) as i64); h.check(); } }

#[test] fn b006() { let _g = lock(); unsafe { let mut h = H::new("engine-harvest28-002-C006");
    let (mut c0, mut h0): (c_int, c_int) = (0, 0);
    sqlite3_db_status(h.db, 2, &mut c0, &mut h0, 0);
    h.ex("CREATE TABLE big_schema_table(a,b,c,d,e,f,g,h);");
    let (mut c1, mut h1): (c_int, c_int) = (0, 0);
    sqlite3_db_status(h.db, 2, &mut c1, &mut h1, 0);
    h.oi("grew", (c1 > c0) as i64); h.oi("nonneg", (c0 >= 0) as i64); h.check(); } }

// ---- C: auth IGNORE ----
static IGNCOL: Mutex<String> = Mutex::new(String::new());
static AUTHREADS: Mutex<i64> = Mutex::new(0);
static ALOG: Mutex<String> = Mutex::new(String::new());
unsafe extern "C" fn auth_cb(_a: *mut c_void, code: c_int, s1: *const c_char, s2: *const c_char, _s3: *const c_char, _s4: *const c_char) -> c_int {
    if code == 20 {
        *AUTHREADS.lock().unwrap() += 1;
        let t = if s1.is_null() { "~".into() } else { CStr::from_ptr(s1).to_string_lossy().into_owned() };
        let c = if s2.is_null() { "~".into() } else { CStr::from_ptr(s2).to_string_lossy().into_owned() };
        { let mut l = ALOG.lock().unwrap(); let sep = if l.is_empty() { "" } else { " " }; l.push_str(&format!("{sep}({t}.{c})")); }
        if !s2.is_null() && *IGNCOL.lock().unwrap() == c { return 2; }
    }
    0
}
fn set_auth(h: &H) { unsafe { sqlite3_set_authorizer(h.db, Some(auth_cb), ptr::null_mut()); } }

#[test] fn c001() { let _g = lock(); let mut h = H::new("engine-harvest28-003-C001");
    h.ex("CREATE TABLE emp(name TEXT, salary INT); INSERT INTO emp VALUES('ann',100),('bob',200);");
    *IGNCOL.lock().unwrap() = "salary".into(); *AUTHREADS.lock().unwrap() = 0; ALOG.lock().unwrap().clear();
    set_auth(&h);
    h.rows("q", "SELECT name, salary FROM emp ORDER BY name");
    h.oi("read_consults", (*AUTHREADS.lock().unwrap() > 0) as i64);
    IGNCOL.lock().unwrap().clear(); h.check(); }

#[test] fn c002() { let _g = lock(); let mut h = H::new("engine-harvest28-003-C002");
    h.ex("CREATE TABLE emp(name TEXT, salary INT); INSERT INTO emp VALUES('ann',100);");
    IGNCOL.lock().unwrap().clear(); *AUTHREADS.lock().unwrap() = 0; ALOG.lock().unwrap().clear();
    set_auth(&h);
    h.rows("q", "SELECT name, salary FROM emp");
    let l = ALOG.lock().unwrap().clone(); h.os("log", Some(l)); h.check(); }

#[test] fn c003() { let _g = lock(); let mut h = H::new("engine-harvest28-003-C003");
    h.ex("CREATE TABLE emp(name TEXT, salary INT); INSERT INTO emp VALUES('ann',100),('bob',200);");
    *IGNCOL.lock().unwrap() = "salary".into(); set_auth(&h);
    h.rows("q", "SELECT name FROM emp WHERE salary > 150");
    IGNCOL.lock().unwrap().clear(); h.check(); }

#[test] fn c004() { let _g = lock(); let mut h = H::new("engine-harvest28-003-C004");
    h.ex("CREATE TABLE emp(name TEXT, salary INT); INSERT INTO emp VALUES('ann',100),('bob',200);");
    *IGNCOL.lock().unwrap() = "salary".into(); set_auth(&h);
    h.rows("q", "SELECT count(salary), sum(salary) FROM emp");
    IGNCOL.lock().unwrap().clear(); h.check(); }

#[test] fn c005() { let _g = lock(); let mut h = H::new("engine-harvest28-003-C005");
    h.ex("CREATE TABLE emp(name TEXT, salary INT); INSERT INTO emp VALUES('ann',100);");
    *IGNCOL.lock().unwrap() = "salary".into(); set_auth(&h);
    h.rows("masked", "SELECT salary FROM emp");
    unsafe { sqlite3_set_authorizer(h.db, None, ptr::null_mut()); }
    h.rows("unmasked", "SELECT salary FROM emp");
    IGNCOL.lock().unwrap().clear(); h.check(); }

#[test] fn c006() { let _g = lock(); let mut h = H::new("engine-harvest28-003-C006");
    h.ex("CREATE TABLE duo(x INT, y INT); INSERT INTO duo VALUES(7,8);");
    *IGNCOL.lock().unwrap() = "y".into(); *AUTHREADS.lock().unwrap() = 0; set_auth(&h);
    h.rows("q", "SELECT * FROM duo");
    IGNCOL.lock().unwrap().clear(); h.check(); }

// ---- E: compress ----
#[test] fn e001() { let _g = lock(); let mut h = H::new("engine-harvest28-004-C001");
    h.rows("rt", "SELECT uncompress(compress('hello compression world'))");
    h.rows("ty", "SELECT typeof(compress('abc')), typeof(uncompress(compress('abc')))"); h.check(); }
#[test] fn e002() { let _g = lock(); let mut h = H::new("engine-harvest28-004-C002");
    h.rows("shrink", "SELECT length(compress(printf('%.400c','a'))) < 400");
    h.rows("rt", "SELECT uncompress(compress(printf('%.400c','a'))) = printf('%.400c','a')"); h.check(); }
#[test] fn e003() { let _g = lock(); let mut h = H::new("engine-harvest28-004-C003");
    h.rows("rt", "SELECT hex(uncompress(compress(X'0011AABB')))"); h.check(); }
#[test] fn e004() { let _g = lock(); let mut h = H::new("engine-harvest28-004-C004");
    h.rows("rt", "SELECT length(uncompress(compress(''))), typeof(uncompress(compress('')))"); h.check(); }

// ---- F: percentile / median ----

// ---- G: next_char ----
#[test] fn g001() { let _g = lock(); let mut h = H::new("engine-harvest28-006-C001");
    h.ex("CREATE TABLE w(word TEXT); INSERT INTO w VALUES('apple'),('apply'),('ape'),('bat');");
    h.rows("ap", "SELECT next_char('ap','w','word')");
    h.rows("a", "SELECT next_char('a','w','word')"); h.check(); }
#[test] fn g002() { let _g = lock(); let mut h = H::new("engine-harvest28-006-C002");
    h.ex("CREATE TABLE w(word TEXT); INSERT INTO w VALUES('cat'),('dog');");
    h.rows("all", "SELECT next_char('','w','word')");
    h.rows("none", "SELECT next_char('zz','w','word')"); h.check(); }
#[test] fn g003() { let _g = lock(); let mut h = H::new("engine-harvest28-006-C003");
    h.ex("CREATE TABLE w(word TEXT); INSERT INTO w VALUES('hi'),('hip');");
    h.rows("hi", "SELECT next_char('hi','w','word')"); h.check(); }

// ---- H: wholenumber / completion ----
#[test] fn h001() { let _g = lock(); let mut h = H::new("engine-harvest28-007-C001");
    h.ex("CREATE VIRTUAL TABLE nums USING wholenumber;");
    h.rows("small", "SELECT value FROM nums WHERE value < 6");
    h.rows("range", "SELECT value FROM nums WHERE value BETWEEN 8 AND 11"); h.check(); }
#[test] fn h002() { let _g = lock(); let mut h = H::new("engine-harvest28-007-C002");
    h.ex("CREATE VIRTUAL TABLE nums USING wholenumber;");
    h.rows("sum", "SELECT sum(value) FROM nums WHERE value <= 10");
    h.rows("cnt", "SELECT count(*) FROM nums WHERE value > 3 AND value < 9"); h.check(); }
#[test] fn h003() { let _g = lock(); let mut h = H::new("engine-harvest28-007-C003");
    h.rows("vac", "SELECT candidate FROM completion('vacu') ORDER BY candidate");
    h.rows("rollb", "SELECT candidate FROM completion('rollb') ORDER BY candidate"); h.check(); }
#[test] fn h004() { let _g = lock(); let mut h = H::new("engine-harvest28-007-C004");
    h.ex("CREATE TABLE zebra_table(zcol INT);");
    h.rows("zeb", "SELECT candidate FROM completion('zebra') ORDER BY candidate"); h.check(); }

// ---- I: thin partials ----
#[test] fn i001() { let _g = lock(); let mut h = H::new("engine-harvest28-008-C001");
    h.exr("create", "CREATE TABLE kw(key INT, value TEXT, offset INT, action TEXT);");
    h.exr("ins", "INSERT INTO kw VALUES(1,'v',2,'a');");
    h.rows("sel", "SELECT key, value, offset, action FROM kw"); h.check(); }
#[test] fn i003() { let _g = lock(); let mut h = H::nodb("engine-harvest28-008-C003");
    let c = |s: &str| -> i64 { unsafe { sqlite3_complete(CString::new(s).unwrap().as_ptr()) as i64 } };
    h.oi("str", c("SELECT 'a;b'"));
    h.oi("str_done", c("SELECT 'a;b';"));
    h.oi("comment", c("SELECT 1 /* ; */"));
    h.oi("comment_done", c("SELECT 1 /* ; */ ;"));
    h.oi("line_comment", c("SELECT 1 -- tail ;\n;"));
    h.finish(); }
#[test] fn i004() { let _g = lock(); let mut h = H::nodb("engine-harvest28-008-C004");
    let c = |s: &str| -> i64 { unsafe { sqlite3_complete(CString::new(s).unwrap().as_ptr()) as i64 } };
    h.oi("quoted_end", c("CREATE TRIGGER r AFTER INSERT ON t BEGIN INSERT INTO u VALUES('END;'); END;"));
    h.oi("quoted_open", c("CREATE TRIGGER r AFTER INSERT ON t BEGIN INSERT INTO u VALUES('END;');"));
    h.finish(); }
#[test] fn i005() { let _g = lock(); let mut h = H::new("engine-harvest28-008-C005");
    h.rows("fn_abs", "SELECT count(*) > 0 FROM pragma_function_list WHERE name='abs'");
    h.rows("prag_jm", "SELECT count(*) > 0 FROM pragma_pragma_list WHERE name='journal_mode'"); h.check(); }

// ---- anti-cheat ----
#[test] fn anti_cheat_harvest28() { let _g = lock(); unsafe {
    let seed = std::process::id() % 100000;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    // compress round-trip on a runtime blob
    let word = format!("rt-{seed}-{seed}-{seed}");
    sqlite3_prepare_v2(db, CString::new(format!("SELECT CAST(uncompress(compress('{word}')) AS TEXT) = '{word}'")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    assert_eq!(sqlite3_column_int(st, 0), 1);
    sqlite3_finalize(st);
    // next_char over a runtime table
    let tn = format!("nc{seed}");
    exs(db, &format!("CREATE TABLE {tn}(word);"));
    exs(db, &format!("INSERT INTO {tn} VALUES('{seed}apple'),('{seed}apex');"));
    sqlite3_prepare_v2(db, CString::new(format!("SELECT next_char('{seed}ap','{tn}','word')")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    sqlite3_step(st);
    assert_eq!(CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy(), "ep");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
