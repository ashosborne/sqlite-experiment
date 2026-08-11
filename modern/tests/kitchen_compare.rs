//! Run-13 kitchen tests (pack v4): the five kitchen goldens + the two RE-HOMED
//! cases (ddl-schema-001-C001, dml-codegen-001-C001) — all answered by the REAL
//! in-memory store (their SQL is deliberately absent from script_table.rs),
//! plus the anti-cheat insert/select.
mod util;
use util::compare_script;

#[test]
fn engine_kitchen_001_c001() {
    compare_script("engine-kitchen", "engine-kitchen-001", "C001",
        "CREATE TABLE k(a INTEGER); INSERT INTO k VALUES(7); SELECT a FROM k;");
}
#[test]
fn engine_kitchen_001_c002() {
    compare_script("engine-kitchen", "engine-kitchen-001", "C002",
        "CREATE TABLE k(a INTEGER, b TEXT); INSERT INTO k VALUES(1,'x'); INSERT INTO k VALUES(2,'y'); SELECT a,b FROM k ORDER BY a;");
}
#[test]
fn engine_kitchen_001_c003() {
    compare_script("engine-kitchen", "engine-kitchen-001", "C003",
        "CREATE TABLE k(a INTEGER); INSERT INTO k VALUES(10); UPDATE k SET a=11; SELECT a FROM k;");
}
#[test]
fn engine_kitchen_001_c004() {
    compare_script("engine-kitchen", "engine-kitchen-001", "C004",
        "CREATE TABLE k(a INTEGER); INSERT INTO k VALUES(1),(2); DELETE FROM k WHERE a=1; SELECT a FROM k;");
}
#[test]
fn engine_kitchen_001_c005() {
    // fresh integer chosen at RECORD time precisely because it appears nowhere else
    compare_script("engine-kitchen", "engine-kitchen-001", "C005",
        "CREATE TABLE k(a INTEGER); INSERT INTO k VALUES(424242); SELECT a FROM k;");
}

// RE-HOMED (pack v4 kitchen_path_cases): same goldens, now served by the store.
#[test]
fn rehomed_ddl_schema_001_c001_via_store() {
    compare_script("ddl-schema", "ddl-schema-001", "C001",
        "CREATE TABLE t1(a INTEGER PRIMARY KEY, b TEXT); SELECT count(*) FROM sqlite_master WHERE name='t1'; DROP TABLE t1; SELECT count(*) FROM sqlite_master;");
}
#[test]
fn rehomed_dml_codegen_001_c001_via_store() {
    compare_script("dml-codegen", "dml-codegen-001", "C001",
        "CREATE TABLE d(a); INSERT INTO d VALUES(1),(2); UPDATE d SET a=a+10 WHERE a=2; DELETE FROM d WHERE a=1; SELECT a, changes(), total_changes() FROM d;");
}

// Anti-cheat: the inserted value is chosen at RUNTIME, so no lookup table can
// contain the answer — only a live store satisfies this.
#[test]
fn anti_cheat_runtime_value_round_trip() {
    use sqlite3_rust_spine::*;
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int, c_void};
    use std::ptr;

    // anti-cheat: value not in script_table
    let n: i64 = 600_000 + (std::process::id() as i64 % 1000);

    struct Cap(Vec<String>);
    unsafe extern "C" fn cb(arg: *mut c_void, argc: c_int, argv: *mut *mut c_char, _az: *mut *mut c_char) -> c_int {
        let cap = &mut *(arg as *mut Cap);
        for i in 0..argc as usize {
            let p = *argv.add(i);
            cap.0.push(if p.is_null() { "NULL".into() } else { CStr::from_ptr(p).to_str().unwrap().into() });
        }
        0
    }
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        let name = CString::new(":memory:").unwrap();
        assert_eq!(sqlite3_open(name.as_ptr(), &mut db), 0);
        let sql = CString::new(format!(
            "CREATE TABLE ac(v INTEGER); INSERT INTO ac VALUES({n}); UPDATE ac SET v=v+1; SELECT v FROM ac;"
        )).unwrap();
        let mut cap = Cap(Vec::new());
        let rc = sqlite3_exec(db, sql.as_ptr(), Some(cb), &mut cap as *mut Cap as *mut c_void, ptr::null_mut());
        assert_eq!(rc, 0);
        assert_eq!(cap.0, vec![(n + 1).to_string()], "store must return the runtime value + 1");
        sqlite3_close(db);
    }
}
