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
