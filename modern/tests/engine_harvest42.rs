//! Run-52 recursive-CTE replay — mirrors /tmp/h42.c.
//! Non-recursive WITH; WITH RECURSIVE UNION ALL as C's Queue/Current FIFO;
//! SQLITE_RECURSIVE (33) scan-gated; UNION-distinct cycle; LIMIT stop; errors.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }
static ALOG: Mutex<String> = Mutex::new(String::new());
static DENY_CODE: Mutex<i32> = Mutex::new(0);

fn fs(p: *const c_char) -> String {
    if p.is_null() { "~".into() }
    else {
        let s = unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() };
        if s.is_empty() { "{}".into() } else { s }
    }
}
unsafe extern "C" fn auth_cb(_c: *mut c_void, code: c_int, s1: *const c_char, s2: *const c_char,
        s3: *const c_char, s4: *const c_char) -> c_int {
    ALOG.lock().unwrap_or_else(|e| e.into_inner())
        .push_str(&format!("[{}|{}|{}|{}|{}]", code, fs(s1), fs(s2), fs(s3), fs(s4)));
    if code == *DENY_CODE.lock().unwrap_or_else(|e| e.into_inner()) { return 1; }
    0
}
fn take_log() -> String { std::mem::take(&mut *ALOG.lock().unwrap_or_else(|e| e.into_inner())) }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        take_log();
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 {
            self.lines.push(format!("OBS {} {} prep.rc={} err={} LOG={}", self.cid, label, rc,
                CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy(), take_log()));
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
        self.lines.push(format!("OBS {} {} rows={} LOG={}", self.cid, label, buf, take_log()));
    } }
    fn cols(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={}", self.cid, label, rc)); return; }
        let mut buf = String::new();
        for i in 0..sqlite3_column_count(st) {
            if i > 0 { buf.push(','); }
            buf.push_str(&CStr::from_ptr(sqlite3_column_name(st, i as c_int)).to_string_lossy());
        }
        sqlite3_finalize(st);
        self.lines.push(format!("OBS {} {} {}", self.cid, label, buf));
    } }
    fn check_keep(&mut self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-harvest42/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
}
fn setup(h: &mut H) {
    h.ex("CREATE TABLE t(a,b); INSERT INTO t VALUES(1,'x'),(2,'y'),(3,'z');");
    h.ex("CREATE TABLE edges(p,c); INSERT INTO edges VALUES(1,2),(1,3),(2,4),(2,5),(3,6),(3,7);");
    h.ex("CREATE TABLE g(a,b); INSERT INTO g VALUES(1,2),(2,3),(3,1);");
}

#[test] fn h001_plain_with() { let _g = lockg();
    let mut h = H::new("engine-harvest42-001-C001");
    setup(&mut h);
    h.rows("w_lit", "WITH x(a) AS (SELECT 1) SELECT * FROM x");
    h.cols("w_lit_cols", "WITH x(a) AS (SELECT 1) SELECT * FROM x");
    h.rows("w_tab", "WITH big(a,b) AS (SELECT a, b FROM t WHERE a > 1) SELECT a, b FROM big");
    h.rows("w_noname", "WITH x AS (SELECT a AS q FROM t) SELECT q FROM x WHERE q < 3");
    h.check_keep();
    h.cid = "engine-harvest42-001-C002";
    h.rows("w_two", "WITH one(v) AS (SELECT 10), two(w) AS (SELECT v+5 FROM one) SELECT v, w FROM one, two");
    h.rows("w_mat", "WITH x(a) AS MATERIALIZED (SELECT 7) SELECT a FROM x");
    h.rows("w_notmat", "WITH x(a) AS NOT MATERIALIZED (SELECT 8) SELECT a FROM x");
    h.check();
}

#[test] fn h002_recursive_ints() { let _g = lockg();
    let mut h = H::new("engine-harvest42-002-C001");
    setup(&mut h);
    h.rows("r_int", "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM c WHERE x<5) SELECT x FROM c");
    h.rows("r_expr", "WITH RECURSIVE c(x,y) AS (SELECT 1, 'a' UNION ALL SELECT x+1, y||'b' FROM c WHERE x<3) SELECT x, y FROM c");
    h.check_keep();
    h.cid = "engine-harvest42-002-C002";
    h.rows("r_two_seed", "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT 10 UNION ALL SELECT x+1 FROM c WHERE x%10 < 3) SELECT x FROM c");
    h.check();
}

#[test] fn h003_table_walk() { let _g = lockg();
    let mut h = H::new("engine-harvest42-003-C001");
    setup(&mut h);
    h.rows("r_bfs", "WITH RECURSIVE walk(n) AS (SELECT 1 UNION ALL SELECT e.c FROM edges e, walk w WHERE e.p = w.n) SELECT n FROM walk");
    h.rows("r_bfs2", "WITH RECURSIVE walk(n) AS (SELECT 2 UNION ALL SELECT e.c FROM edges e JOIN walk ON e.p = walk.n) SELECT n FROM walk");
    h.check();
}

#[test] fn h004_auth_33() { let _g = lockg(); unsafe {
    let mut h = H::new("engine-harvest42-004-C001");
    setup(&mut h);
    sqlite3_set_authorizer(h.db, Some(auth_cb), ptr::null_mut());
    h.rows("a_unused", "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM c WHERE x<3) SELECT 42");
    h.rows("a_scanned", "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM c WHERE x<3) SELECT x FROM c");
    h.rows("a_nonrec", "WITH x(a) AS (SELECT 1) SELECT a FROM x");
    h.check_keep();
    h.cid = "engine-harvest42-004-C002";
    *DENY_CODE.lock().unwrap_or_else(|e| e.into_inner()) = 33;
    h.rows("a_deny", "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM c WHERE x<3) SELECT x FROM c");
    *DENY_CODE.lock().unwrap_or_else(|e| e.into_inner()) = 0;
    sqlite3_set_authorizer(h.db, None, ptr::null_mut());
    h.check();
} }

#[test] fn h005_distinct_limit() { let _g = lockg();
    let mut h = H::new("engine-harvest42-005-C001");
    setup(&mut h);
    h.rows("r_union_distinct", "WITH RECURSIVE r(n) AS (SELECT 1 UNION SELECT g.b FROM g, r WHERE g.a = r.n) SELECT n FROM r");
    h.check_keep();
    h.cid = "engine-harvest42-005-C002";
    h.rows("r_limit", "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM c) SELECT x FROM c LIMIT 4");
    h.rows("r_orderby", "WITH RECURSIVE c(x) AS (SELECT 3 UNION ALL SELECT x-1 FROM c WHERE x>1) SELECT x FROM c ORDER BY x");
    h.rows("r_sub", "SELECT (SELECT count(*) FROM (WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM c WHERE x<4) SELECT x FROM c))");
    h.check();
}

#[test] fn h006_errors() { let _g = lockg();
    let mut h = H::new("engine-harvest42-006-C001");
    setup(&mut h);
    h.rows("e_colcount", "WITH RECURSIVE c(x,y) AS (SELECT 1 UNION ALL SELECT x+1 FROM c WHERE x<3) SELECT x FROM c");
    h.rows("e_seedcols", "WITH x(a,b) AS (SELECT 1) SELECT * FROM x");
    h.check_keep();
    h.cid = "engine-harvest42-006-C002";
    h.rows("e_circular", "WITH a AS (SELECT * FROM b), b AS (SELECT * FROM a) SELECT * FROM a");
    h.rows("e_multi_rec", "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM c, c AS d WHERE x<3) SELECT x FROM c");
    h.rows("e_agg", "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT max(x)+1 FROM c WHERE x<3) SELECT x FROM c");
    h.check();
}

// ================= anti-cheat =================

#[test] fn anti_cheat_h42_runtime_bound() { let _g = lockg(); unsafe {
    // a runtime stop value must change how many rows the machine produces
    let n = (std::process::id() % 5) as i64 + 3;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let sql = CString::new(format!(
        "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM c WHERE x<{n}) SELECT x FROM c")).unwrap();
    assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut()), 0);
    let mut got: Vec<i64> = Vec::new();
    while sqlite3_step(st) == 100 { got.push(sqlite3_column_int64(st, 0)); }
    sqlite3_finalize(st);
    assert_eq!(got, (1..=n).collect::<Vec<_>>(), "runtime bound {n} must drive the row count");
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h42_runtime_cte_name() { let _g = lockg(); unsafe {
    // a runtime CTE name must appear in the code-33 s4 slot when scanned
    let seed = std::process::id() % 100000;
    let cname = format!("rtc{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_set_authorizer(db, Some(auth_cb), ptr::null_mut());
    take_log();
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let sql = CString::new(format!(
        "WITH RECURSIVE {cname}(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM {cname} WHERE x<2) SELECT x FROM {cname}")).unwrap();
    assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut()), 0);
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    let log = take_log();
    assert!(log.contains(&format!("[33|~|~|~|{cname}]")), "33 must carry the runtime CTE name: {log}");
    sqlite3_set_authorizer(db, None, ptr::null_mut());
    sqlite3_close(db);
} }
