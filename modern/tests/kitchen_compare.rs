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

// ===== run-14 re-homed (pack v5 kitchen_path_cases) — same goldens, store-served =====
#[test]
fn rehomed_name_resolution_001_c001() {
    compare_script("name-resolution", "name-resolution-001", "C001",
        "CREATE TABLE n1(a); INSERT INTO n1 VALUES(5); SELECT n1.a, a, rowid FROM n1;");
}
#[test]
fn rehomed_ddl_schema_002_c001() {
    compare_script("ddl-schema", "ddl-schema-002", "C001",
        "CREATE TABLE t2(a); CREATE UNIQUE INDEX i2 ON t2(a); INSERT INTO t2 VALUES(1); INSERT OR IGNORE INTO t2 VALUES(1); SELECT count(*) FROM t2;");
}
#[test]
fn rehomed_dml_codegen_002_c001() {
    compare_script("dml-codegen", "dml-codegen-002", "C001",
        "CREATE TABLE u(a UNIQUE); INSERT INTO u VALUES(1); INSERT OR REPLACE INTO u VALUES(1); INSERT OR IGNORE INTO u VALUES(1); SELECT count(*) FROM u;");
}
#[test]
fn rehomed_ddl_schema_003_c001() {
    compare_script("ddl-schema", "ddl-schema-003", "C001",
        "CREATE TABLE t3(a); ALTER TABLE t3 RENAME TO t3x; ALTER TABLE t3x ADD COLUMN b DEFAULT 5; INSERT INTO t3x(a) VALUES(9); SELECT a,b FROM t3x;");
}
#[test]
fn rehomed_upsert_001_c001() {
    compare_script("upsert", "upsert-001", "C001",
        "CREATE TABLE up(a INTEGER PRIMARY KEY, b); INSERT INTO up VALUES(1,'x'); INSERT INTO up VALUES(1,'y') ON CONFLICT(a) DO NOTHING; SELECT b, count(*) FROM up;");
}
#[test]
fn rehomed_upsert_002_c001() {
    compare_script("upsert", "upsert-002", "C001",
        "CREATE TABLE up2(a INTEGER PRIMARY KEY, b); INSERT INTO up2 VALUES(1,'x'); INSERT INTO up2 VALUES(1,'y') ON CONFLICT(a) DO UPDATE SET b=excluded.b; SELECT b FROM up2;");
}
#[test]
fn rehomed_foreign_keys_001_c001() {
    compare_script("foreign-keys", "foreign-keys-001", "C001",
        "PRAGMA foreign_keys=ON; CREATE TABLE par(id INTEGER PRIMARY KEY); CREATE TABLE chi(pid REFERENCES par(id)); INSERT INTO chi VALUES(1);");
}
#[test]
fn rehomed_foreign_keys_002_c001() {
    compare_script("foreign-keys", "foreign-keys-002", "C001",
        "PRAGMA foreign_keys=ON; CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE CASCADE); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1); DELETE FROM p2; SELECT count(*) FROM c2;");
}
#[test]
fn rehomed_foreign_keys_003_c001() {
    compare_script("foreign-keys", "foreign-keys-003", "C001",
        "PRAGMA foreign_keys=ON; CREATE TABLE p3(id INTEGER PRIMARY KEY); CREATE TABLE c3(pid REFERENCES p3(id)); INSERT INTO p3 VALUES(1); INSERT INTO c3 VALUES(1); DROP TABLE p3;");
}
#[test]
fn rehomed_triggers_001_c001() {
    compare_script("triggers", "triggers-001", "C001",
        "CREATE TABLE tr(a); CREATE TABLE tlog(v); CREATE TRIGGER trg AFTER INSERT ON tr BEGIN INSERT INTO tlog VALUES(new.a); END; SELECT count(*) FROM sqlite_master WHERE type='trigger';");
}
#[test]
fn rehomed_triggers_002_c001() {
    compare_script("triggers", "triggers-002", "C001",
        "CREATE TABLE tr2(a); CREATE TABLE tlog2(v); CREATE TRIGGER trg2 AFTER INSERT ON tr2 BEGIN INSERT INTO tlog2 VALUES(new.a*2); END; INSERT INTO tr2 VALUES(7); SELECT v FROM tlog2;");
}

// anti-cheat: runtime FK round-trip, value not in script_table
#[test]
fn anti_cheat_runtime_fk_round_trip() {
    use sqlite3_rust_spine::*;
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int, c_void};
    use std::ptr;

    let n: i64 = 700_000 + (std::process::id() as i64 % 1000);

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
        let setup = CString::new(format!(
            "PRAGMA foreign_keys=ON; CREATE TABLE parent(id INTEGER PRIMARY KEY); \
             CREATE TABLE child(pid REFERENCES parent(id)); \
             INSERT INTO parent VALUES({n}); INSERT INTO child VALUES({n}); SELECT pid FROM child;"
        )).unwrap();
        let mut cap = Cap(Vec::new());
        let rc = sqlite3_exec(db, setup.as_ptr(), Some(cb), &mut cap as *mut Cap as *mut c_void, ptr::null_mut());
        assert_eq!(rc, 0);
        assert_eq!(cap.0, vec![n.to_string()], "store must return the runtime FK value");
        // now a missing parent key -> rc 19 (FK violation), straight from the store rules
        let bad = CString::new(format!("INSERT INTO child VALUES({});", n + 1)).unwrap();
        let mut err: *mut c_char = ptr::null_mut();
        let rc2 = sqlite3_exec(db, bad.as_ptr(), None, ptr::null_mut(), &mut err);
        assert_eq!(rc2, 19, "missing parent key must fail with SQLITE_CONSTRAINT");
        assert!(!err.is_null() && *err != 0);
        sqlite3_free(err as *mut c_void);
        sqlite3_close(db);
    }
}
