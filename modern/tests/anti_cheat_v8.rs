//! pack v8 anti-cheat: answers must be COMPUTED. Each test feeds a runtime-varying
//! value through sqlite3_exec; a whole-script string pin cannot exist for it.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

fn exec_collect(sql: &str) -> (i32, Vec<Vec<Option<String>>>) {
    unsafe extern "C" fn cb(arg: *mut c_void, n: c_int, vals: *mut *mut c_char, _c: *mut *mut c_char) -> c_int {
        let out = unsafe { &mut *(arg as *mut Vec<Vec<Option<String>>>) };
        let mut row = Vec::new();
        for i in 0..n as isize {
            let p = unsafe { *vals.offset(i) };
            row.push(if p.is_null() { None } else { Some(unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned()) });
        }
        out.push(row);
        0
    }
    unsafe {
        let mut db: *mut Sqlite3 = std::ptr::null_mut();
        assert_eq!(sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db), 0);
        let mut rows: Vec<Vec<Option<String>>> = Vec::new();
        let rc = sqlite3_exec(db, CString::new(sql).unwrap().as_ptr(), Some(cb),
                              &mut rows as *mut _ as *mut c_void, std::ptr::null_mut());
        sqlite3_close(db);
        (rc, rows)
    }
}

fn runtime_int() -> i64 {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    (t.subsec_nanos() as i64 % 100_000) + (std::process::id() as i64 % 1000) * 100_000 + 3
}

#[test]
fn anti_cheat_expr_runtime() {
    let n = runtime_int();
    let (rc, rows) = exec_collect(&format!("SELECT {n} + 2, {n} * 3, upper(printf('v%d', {n}));"));
    assert_eq!(rc, 0);
    assert_eq!(rows[0][0].as_deref(), Some((n + 2).to_string().as_str()));
    assert_eq!(rows[0][1].as_deref(), Some((n * 3).to_string().as_str()));
    assert_eq!(rows[0][2].as_deref(), Some(format!("V{n}").as_str()));
}

#[test]
fn anti_cheat_pragma_roundtrip() {
    let n = runtime_int() % 100_000;
    let (rc, rows) = exec_collect(&format!("PRAGMA user_version={n}; PRAGMA user_version; PRAGMA application_id={}; PRAGMA application_id;", n + 7));
    assert_eq!(rc, 0);
    assert_eq!(rows[0][0].as_deref(), Some(n.to_string().as_str()));
    assert_eq!(rows[1][0].as_deref(), Some((n + 7).to_string().as_str()));
}

#[test]
fn anti_cheat_json_or_scalar() {
    let n = runtime_int();
    let (rc, rows) = exec_collect(&format!(
        "SELECT json_extract('{{\"k\":{n}}}', '$.k'), json_set('{{}}','$.v',{n}), length(printf('%d', {n}));"));
    assert_eq!(rc, 0);
    assert_eq!(rows[0][0].as_deref(), Some(n.to_string().as_str()));
    assert_eq!(rows[0][1].as_deref(), Some(format!("{{\"v\":{n}}}").as_str()));
    assert_eq!(rows[0][2].as_deref(), Some(n.to_string().len().to_string().as_str()));
}

#[test]
fn anti_cheat_script_table_is_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0,
        "pack v8 law: no behavioural pins may exist");
}

#[test]
fn anti_cheat_unknown_sql_fails_honestly() {
    let (rc, rows) = exec_collect("SELECT frobnicate_no_such_fn(1);");
    assert_ne!(rc, 0, "unknown SQL must not invent success");
    assert!(rows.is_empty());
}

// ---- pack v9 anti-cheat: joins + scalar subqueries computed from runtime values ----

#[test]
fn anti_cheat_join_runtime() {
    let k = runtime_int();
    let (rc, rows) = exec_collect(&format!(
        "CREATE TABLE ja(k INTEGER, n TEXT); INSERT INTO ja VALUES({k},'left'),({},'other'); \
         CREATE TABLE jb(k INTEGER, t TEXT); INSERT INTO jb VALUES({k},'hit'); \
         SELECT ja.n, jb.t FROM ja JOIN jb ON ja.k = jb.k;", k + 1));
    assert_eq!(rc, 0);
    assert_eq!(rows, vec![vec![Some("left".to_string()), Some("hit".to_string())]]);
}

#[test]
fn anti_cheat_scalar_subquery_runtime() {
    let n = runtime_int();
    let (rc, rows) = exec_collect(&format!(
        "CREATE TABLE sq(v INTEGER); INSERT INTO sq VALUES({n}),({}); \
         SELECT (SELECT max(v) FROM sq), (SELECT v FROM sq ORDER BY v LIMIT 1), \
                (SELECT {n} + 1);", n - 5));
    assert_eq!(rc, 0);
    assert_eq!(rows[0][0].as_deref(), Some((n).to_string().as_str()));
    assert_eq!(rows[0][1].as_deref(), Some((n - 5).to_string().as_str()));
    assert_eq!(rows[0][2].as_deref(), Some((n + 1).to_string().as_str()));
}

#[test]
fn anti_cheat_script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0,
        "pack v9 law: SCRIPT_TABLE must stay at zero behavioural pins");
}

// ---- pack v10 anti-cheat: date/time + set-op/CHECK computed from runtime values ----

#[test]
fn anti_cheat_datetime_runtime() {
    // runtime-chosen day-of-month: engine must compute, not match
    let d = (runtime_int() % 27 + 1) as i64;
    let (rc, rows) = exec_collect(&format!(
        "SELECT strftime('%d','2026-04-{d:02}'), date('2026-04-{d:02}','+1 day'), unixepoch('2026-04-{d:02}') - unixepoch('2026-04-01');"));
    assert_eq!(rc, 0);
    assert_eq!(rows[0][0].as_deref(), Some(format!("{d:02}").as_str()));
    assert_eq!(rows[0][2].as_deref(), Some(((d - 1) * 86400).to_string().as_str()));
}

#[test]
fn anti_cheat_setop_and_check_runtime() {
    let n = runtime_int();
    let (rc, rows) = exec_collect(&format!(
        "CREATE TABLE ac(v INTEGER CHECK(v < {n})); INSERT INTO ac VALUES({}); \
         SELECT v FROM ac INTERSECT SELECT {};", n - 1, n - 1));
    assert_eq!(rc, 0);
    assert_eq!(rows[0][0].as_deref(), Some((n - 1).to_string().as_str()));
    let (rc2, rows2) = exec_collect(&format!(
        "CREATE TABLE ac(v INTEGER CHECK(v < {n})); INSERT INTO ac VALUES({n});"));
    assert_eq!(rc2, 19, "runtime CHECK violation must be computed");
    assert!(rows2.is_empty());
}

// ---- pack v11 anti-cheat: HAVING/DISTINCT + thin-misc over runtime values ----

#[test]
fn anti_cheat_having_distinct_runtime() {
    let n = runtime_int();
    let (rc, rows) = exec_collect(&format!(
        "CREATE TABLE hd(k TEXT, v INTEGER); INSERT INTO hd VALUES('x',{n}),('x',{n}),('y',1); \
         SELECT k, count(DISTINCT v), sum(v) FROM hd GROUP BY k HAVING sum(v) > {} ORDER BY k;", n));
    assert_eq!(rc, 0);
    assert_eq!(rows, vec![vec![Some("x".into()), Some("1".into()), Some((n * 2).to_string())]]);
}

#[test]
fn anti_cheat_thin_misc_runtime() {
    let n = runtime_int();
    // sha1_query over a runtime-varying statement: computed via the real protocol + digest
    let (rc, rows) = exec_collect(&format!("SELECT sha1_query('SELECT {n}'), sha1_query('SELECT {}');", n + 1));
    assert_eq!(rc, 0);
    let (a, b) = (rows[0][0].clone().unwrap(), rows[0][1].clone().unwrap());
    assert_eq!(a.len(), 40);
    assert_ne!(a, b, "different queries must hash differently (computed, not pinned)");
    // base85 round-trip of runtime bytes
    let (rc2, rows2) = exec_collect(&format!(
        "SELECT hex(base85(base85(ieee754_to_blob({n}.5))));"));
    assert_eq!(rc2, 0);
    assert_eq!(rows2[0][0].as_deref().map(|h| h.to_lowercase()),
               Some(format!("{:016x}", (n as f64 + 0.5).to_bits())));
}

// ---- pack v13 anti-cheat: prepared statements compute with runtime binds ----

#[test]
fn anti_cheat_prepare_runtime_bind() {
    unsafe {
        let mut db: *mut Sqlite3 = std::ptr::null_mut();
        sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
        let mut st: *mut Sqlite3Stmt = std::ptr::null_mut();
        let sql = CString::new("SELECT ?1 + ?2").unwrap();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, std::ptr::null_mut()), 0);
        let (a, b) = (runtime_int(), runtime_int() / 3 + 11);
        sqlite3_bind_int64(st, 1, a);
        sqlite3_bind_int64(st, 2, b);
        assert_eq!(sqlite3_step(st), 100);
        assert_eq!(sqlite3_column_int64(st, 0), a + b, "bound values must drive the computation");
        sqlite3_finalize(st);
        sqlite3_close(db);
    }
}

#[test]
fn anti_cheat_prepare_dml_visible() {
    unsafe {
        let mut db: *mut Sqlite3 = std::ptr::null_mut();
        sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
        let k = runtime_int();
        let (rc0, _rows) = (sqlite3_exec(db, CString::new("CREATE TABLE ac(v);").unwrap().as_ptr(), None, std::ptr::null_mut(), std::ptr::null_mut()), ());
        assert_eq!(rc0, 0);
        let mut st: *mut Sqlite3Stmt = std::ptr::null_mut();
        let ins = CString::new("INSERT INTO ac VALUES(?1)").unwrap();
        assert_eq!(sqlite3_prepare_v2(db, ins.as_ptr(), -1, &mut st, std::ptr::null_mut()), 0);
        sqlite3_bind_int64(st, 1, k);
        assert_eq!(sqlite3_step(st), 101);
        sqlite3_finalize(st);
        let sel = CString::new(format!("SELECT v FROM ac WHERE v = {k}")).unwrap();
        let mut st2: *mut Sqlite3Stmt = std::ptr::null_mut();
        assert_eq!(sqlite3_prepare_v2(db, sel.as_ptr(), -1, &mut st2, std::ptr::null_mut()), 0);
        assert_eq!(sqlite3_step(st2), 100, "prepared DML effect must be visible");
        assert_eq!(sqlite3_column_int64(st2, 0), k);
        sqlite3_finalize(st2);
        sqlite3_close(db);
    }
}

#[test]
fn anti_cheat_column_types() {
    unsafe {
        let mut db: *mut Sqlite3 = std::ptr::null_mut();
        sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
        let n = runtime_int();
        let mut st: *mut Sqlite3Stmt = std::ptr::null_mut();
        let sql = CString::new(format!("SELECT {n}, {n}.5, 'v{n}', NULL")).unwrap();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, std::ptr::null_mut()), 0);
        assert_eq!(sqlite3_step(st), 100);
        assert_eq!(sqlite3_column_type(st, 0), 1);
        assert_eq!(sqlite3_column_type(st, 1), 2);
        assert_eq!(sqlite3_column_type(st, 2), 3);
        assert_eq!(sqlite3_column_type(st, 3), 5);
        assert_eq!(sqlite3_column_int64(st, 0), n);
        let p = sqlite3_column_text(st, 2);
        assert_eq!(CStr::from_ptr(p as *const std::os::raw::c_char).to_str().unwrap(), format!("v{n}"));
        sqlite3_finalize(st);
        sqlite3_close(db);
    }
}
