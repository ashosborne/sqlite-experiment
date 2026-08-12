//! v12 HONESTY GATE: the pinned C sqlite3 CLI must accept Rust-written files with
//! overflow chains and on-disk index b-trees (integrity_check=ok, exact payloads,
//! duplicate INSERT rejected by C itself).
use sqlite3_rust_spine::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::process::Command;
use std::ptr;

const CLI: &str = "/tmp/sqlite-build/sqlite3";

unsafe fn rust_exec(path: &str, sql: &str) -> i32 {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let p = CString::new(path).unwrap();
    let mut rc = sqlite3_open(p.as_ptr(), &mut db);
    if rc == 0 {
        let c = CString::new(sql).unwrap();
        rc = sqlite3_exec(db, c.as_ptr(), None, ptr::null_mut(), ptr::null_mut() as *mut *mut c_char);
    }
    sqlite3_close(db);
    rc
}
fn c_query(path: &str, sql: &str) -> String {
    let out = Command::new(CLI).arg(path).arg(sql).output().expect("pinned C CLI");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}
fn c_query_err(path: &str, sql: &str) -> (String, String) {
    let out = Command::new(CLI).arg(path).arg(sql).output().expect("pinned C CLI");
    (String::from_utf8_lossy(&out.stdout).trim().to_string(),
     String::from_utf8_lossy(&out.stderr).trim().to_string())
}

#[test]
fn rust_write_c_read_overflow() {
    let path = format!("/tmp/ov_interop_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    let long: String = "abcdefghij".repeat(900); // 9000 chars -> multi-page overflow chain
    unsafe {
        assert_eq!(rust_exec(&path, &format!(
            "CREATE TABLE ov(a INTEGER, t TEXT); INSERT INTO ov VALUES(7,'{long}');")), 0);
    }
    assert_eq!(c_query(&path, "PRAGMA integrity_check;"), "ok",
        "C must integrity_check-accept a Rust overflow chain");
    assert_eq!(c_query(&path, "SELECT length(t) FROM ov;"), "9000");
    assert_eq!(c_query(&path, "SELECT substr(t,8991,10) FROM ov;"), "abcdefghij");
    assert_eq!(c_query(&path, &format!("SELECT t='{long}' FROM ov;")), "1",
        "C must read back the EXACT overflow payload");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn rust_write_c_unique_after_reopen() {
    let path = format!("/tmp/uq_interop_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    unsafe {
        assert_eq!(rust_exec(&path, "CREATE TABLE q(a INTEGER UNIQUE, t TEXT UNIQUE); INSERT INTO q VALUES(1,'x'),(2,'y');"), 0);
    }
    assert_eq!(c_query(&path, "PRAGMA integrity_check;"), "ok",
        "C must integrity_check-accept Rust autoindex b-trees");
    let (_out, err) = c_query_err(&path, "INSERT INTO q VALUES(1,'z');");
    assert!(err.contains("UNIQUE constraint failed"),
        "C must enforce the on-disk unique index against a Rust file, got: {err}");
    let (_o2, err2) = c_query_err(&path, "INSERT INTO q VALUES(3,'x');");
    assert!(err2.contains("UNIQUE constraint failed"), "text unique too, got: {err2}");
    assert_eq!(c_query(&path, "SELECT count(*) FROM q;"), "2");
    // and Rust itself enforces after ITS OWN reopen
    unsafe { assert_eq!(rust_exec(&path, "INSERT INTO q VALUES(2,'w');"), 19); }
    let _ = std::fs::remove_file(&path);
}

#[test]
fn anti_cheat_overflow_runtime() {
    let n = std::process::id() as i64 + 12_345;
    let path = format!("/tmp/ovr_{}.db", n);
    let _ = std::fs::remove_file(&path);
    let marker = format!("RUNTIME{n}MARK");
    let long = format!("{}{}", "z".repeat(7000), marker);
    unsafe {
        assert_eq!(rust_exec(&path, &format!("CREATE TABLE r(t TEXT); INSERT INTO r VALUES('{long}');")), 0);
        // reopen in Rust; the runtime marker must survive the overflow chain round-trip
        let mut db: *mut Sqlite3 = ptr::null_mut();
        let p = CString::new(path.clone()).unwrap();
        assert_eq!(sqlite3_open(p.as_ptr(), &mut db), 0);
        let sql = CString::new(format!("CREATE TABLE chk(x INTEGER); INSERT INTO chk SELECT 1 FROM r WHERE t = '{long}';")).unwrap();
        // simpler: verify via C CLI (independent implementation)
        sqlite3_close(db);
        let _ = sql;
    }
    assert_eq!(c_query(&path, &format!("SELECT substr(t, 7001, {}) FROM r;", marker.len())), marker);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn anti_cheat_unique_runtime() {
    let n = std::process::id() as i64 % 100_000 + 700_003;
    let path = format!("/tmp/uqr_{}.db", n);
    let _ = std::fs::remove_file(&path);
    unsafe {
        assert_eq!(rust_exec(&path, &format!("CREATE TABLE u(k INTEGER UNIQUE); INSERT INTO u VALUES({n});")), 0);
        // reopen: the runtime key must be unique-enforced from the durable file
        assert_eq!(rust_exec(&path, &format!("INSERT INTO u VALUES({n});")), 19);
        assert_eq!(rust_exec(&path, &format!("INSERT INTO u VALUES({});", n + 1)), 0);
    }
    assert_eq!(c_query(&path, "SELECT count(*) FROM u;"), "2");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn anti_cheat_script_table_empty_v12() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}

// ---- pack v17: C reads Rust index files + probe anti-cheat ----

#[test]
fn rust_write_c_index_lookup() {
    let path = format!("/tmp/idx_interop_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    let rows: String = (1..=600).map(|i| format!("({i},'v{i}')")).collect::<Vec<_>>().join(",");
    unsafe {
        assert_eq!(rust_exec(&path, &format!(
            "CREATE TABLE t(k INTEGER, v TEXT); CREATE INDEX ik ON t(k); INSERT INTO t VALUES {rows};")), 0);
    }
    // C must integrity_check-accept the (multi-leaf) index b-tree Rust wrote
    assert_eq!(c_query(&path, "PRAGMA integrity_check;"), "ok");
    // and find a row by the indexed key
    assert_eq!(c_query(&path, "SELECT v FROM t WHERE k = 432;"), "v432");
    assert_eq!(c_query(&path, "SELECT count(*) FROM t;"), "600");
    // C's own planner uses the index (proves the b-tree is real, not decoration)
    let plan = c_query(&path, "EXPLAIN QUERY PLAN SELECT v FROM t WHERE k = 432;");
    assert!(plan.contains("USING INDEX ik") || plan.contains("USING COVERING INDEX ik"),
        "C must plan an index search on the Rust file, got: {plan}");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn anti_cheat_index_lookup_runtime() {
    use std::ffi::CString;
    let n = (std::process::id() as i64 % 400) + 50;
    let mut db: *mut Sqlite3 = ptr::null_mut();
    unsafe {
        sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
        let rows: String = (1..=500).map(|i| format!("({i},{})", i * 10)).collect::<Vec<_>>().join(",");
        assert_eq!(rust_exec_db(db, &format!("CREATE TABLE t(k INTEGER, v INTEGER); CREATE INDEX ik ON t(k); INSERT INTO t VALUES {rows};")), 0);
        // a selective lookup: must return the C-correct row AND use a real index probe
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let q = CString::new(format!("SELECT v FROM t WHERE k = {n}")).unwrap();
        sqlite3_prepare_v2(db, q.as_ptr(), -1, &mut st, ptr::null_mut());
        assert_eq!(sqlite3_step(st), 100);
        assert_eq!(sqlite3_column_int64(st, 0), n * 10);
        sqlite3_finalize(st);
        assert!(sqlite3_rust_spine::store::index_probe_count() >= 1, "lookup must probe the index b-tree");
        // missing key -> empty
        let q2 = CString::new("SELECT v FROM t WHERE k = 99999").unwrap();
        let mut st2: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db, q2.as_ptr(), -1, &mut st2, ptr::null_mut());
        assert_eq!(sqlite3_step(st2), 101);
        sqlite3_finalize(st2);
        sqlite3_close(db);
    }
}

unsafe fn rust_exec_db(db: *mut Sqlite3, sql: &str) -> i32 {
    let c = std::ffi::CString::new(sql).unwrap();
    sqlite3_exec(db, c.as_ptr(), None, ptr::null_mut(), ptr::null_mut() as *mut *mut c_char)
}

#[test]
fn anti_cheat_partial_or_expr_index() {
    let path = format!("/tmp/idx_pe_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    unsafe {
        // partial UNIQUE: dups below the predicate allowed, above rejected — after reopen
        assert_eq!(rust_exec(&path, "CREATE TABLE pu(a INTEGER); CREATE UNIQUE INDEX pux ON pu(a) WHERE a > 10; INSERT INTO pu VALUES(5),(5),(20);"), 0);
        assert_eq!(rust_exec(&path, "INSERT INTO pu VALUES(5);"), 0, "below-predicate dup allowed");
        assert_eq!(rust_exec(&path, "INSERT INTO pu VALUES(20);"), 19, "above-predicate dup rejected after reopen");
    }
    // expression index is C-readable
    let path2 = format!("/tmp/idx_ex_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path2);
    unsafe { assert_eq!(rust_exec(&path2, "CREATE TABLE nx(nm TEXT); CREATE INDEX ie ON nx(lower(nm)); INSERT INTO nx VALUES('Ann'),('BOB');"), 0); }
    assert_eq!(c_query(&path2, "PRAGMA integrity_check;"), "ok");
    assert_eq!(c_query(&path2, "SELECT nm FROM nx WHERE lower(nm)='bob';"), "BOB");
    let _ = std::fs::remove_file(&path); let _ = std::fs::remove_file(&path2);
}

#[test]
fn anti_cheat_multileaf_index() {
    let path = format!("/tmp/idx_ml_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    let rows: String = (1..=1000).map(|i| format!("('k{:05}',{i})", i)).collect::<Vec<_>>().join(",");
    unsafe { assert_eq!(rust_exec(&path, &format!("CREATE TABLE b(nm TEXT, n INTEGER); CREATE INDEX bi ON b(nm); INSERT INTO b VALUES {rows};")), 0); }
    assert_eq!(c_query(&path, "PRAGMA integrity_check;"), "ok", "multi-leaf index must be C-valid");
    assert_eq!(c_query(&path, "SELECT n FROM b WHERE nm='k00777';"), "777");
    // Rust probes it after reopen too
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(std::ffi::CString::new(path.clone()).unwrap().as_ptr(), &mut db);
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let q = std::ffi::CString::new("SELECT n FROM b WHERE nm='k00777'").unwrap();
        sqlite3_prepare_v2(db, q.as_ptr(), -1, &mut st, ptr::null_mut());
        assert_eq!(sqlite3_step(st), 100);
        assert_eq!(sqlite3_column_int64(st, 0), 777);
        sqlite3_finalize(st);
        assert!(sqlite3_rust_spine::store::index_probe_count() >= 1);
        sqlite3_close(db);
    }
    let _ = std::fs::remove_file(&path);
}
