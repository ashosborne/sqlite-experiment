//! Run-15 interop tests — the honesty gate (pack v6): a file Rust writes must be
//! readable by the pinned C library, and Rust must read a file C wrote.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::process::Command;
use std::ptr;

fn pin_cli() -> String {
    std::env::var("SQLITE_PIN_BIN").unwrap_or_else(|_| "/tmp/sqlite-build/sqlite3".into())
}

unsafe fn rust_exec(db: *mut Sqlite3, sql: &str) -> i32 {
    let c = CString::new(sql).unwrap();
    sqlite3_exec(db, c.as_ptr(), None, ptr::null_mut(), ptr::null_mut())
}

// MANDATORY (not deferrable): Rust writes a real SQLite file; the pinned C CLI reads it.
#[test]
fn rust_write_c_read() {
    let cli = pin_cli();
    assert!(std::path::Path::new(&cli).exists(), "pinned sqlite3 CLI required at {cli}");
    let n: i64 = 800_000 + (std::process::id() as i64 % 1000); // runtime value, not in script_table
    let path = format!("/tmp/eftest/rwcr_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        let p = CString::new(path.clone()).unwrap();
        assert_eq!(sqlite3_open(p.as_ptr(), &mut db), 0);
        assert_eq!(rust_exec(db, &format!("CREATE TABLE f(a INTEGER); INSERT INTO f VALUES({n});")), 0);
        assert_eq!(sqlite3_close(db), 0); // save-on-close writes the SQLite file
    }
    let out = Command::new(&cli).arg(&path).arg("SELECT a FROM f;").output().expect("run pin cli");
    let got = String::from_utf8_lossy(&out.stdout);
    assert_eq!(got.trim(), n.to_string(), "pinned C must read Rust's file (stderr: {})",
               String::from_utf8_lossy(&out.stderr));
    let _ = std::fs::remove_file(&path);
}

// BEST-EFFORT: C writes a file; Rust reads it back.
#[test]
fn c_write_rust_read() {
    let cli = pin_cli();
    assert!(std::path::Path::new(&cli).exists(), "pinned sqlite3 CLI required at {cli}");
    let v2: i64 = 900_000 + (std::process::id() as i64 % 1000);
    let path = format!("/tmp/eftest/cwrr_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    let st = Command::new(&cli).arg(&path)
        .arg(format!("CREATE TABLE g(a INTEGER); INSERT INTO g VALUES({v2});"))
        .status().expect("run pin cli");
    assert!(st.success());
    unsafe extern "C" fn cb(arg: *mut c_void, argc: c_int, argv: *mut *mut c_char, _az: *mut *mut c_char) -> c_int {
        let out = &mut *(arg as *mut Vec<String>);
        for i in 0..argc as usize {
            let p = *argv.add(i);
            out.push(if p.is_null() { "NULL".into() } else { CStr::from_ptr(p).to_str().unwrap().into() });
        }
        0
    }
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        let p = CString::new(path.clone()).unwrap();
        assert_eq!(sqlite3_open(p.as_ptr(), &mut db), 0); // loads C's file via dbfile::read_db
        let sql = CString::new("SELECT a FROM g;").unwrap();
        let mut got: Vec<String> = Vec::new();
        let rc = sqlite3_exec(db, sql.as_ptr(), Some(cb), &mut got as *mut Vec<String> as *mut c_void, ptr::null_mut());
        assert_eq!(rc, 0);
        assert_eq!(got, vec![v2.to_string()], "Rust must read the file C wrote");
        sqlite3_close(db);
    }
    let _ = std::fs::remove_file(&path);
}

// anti-cheat: not in script_table
#[test]
fn anti_cheat_reopen_runtime() {
    let n: i64 = 850_000 + (std::process::id() as i64 % 1000);
    let path = format!("/tmp/eftest/reopen_{}.db", std::process::id());
    let _ = std::fs::remove_file(&path);
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        let p = CString::new(path.clone()).unwrap();
        assert_eq!(sqlite3_open(p.as_ptr(), &mut db), 0);
        assert_eq!(rust_exec(db, &format!("CREATE TABLE r(a INTEGER); INSERT INTO r VALUES({n});")), 0);
        sqlite3_close(db);
        // reopen a fresh connection on the same path
        let mut db2: *mut Sqlite3 = ptr::null_mut();
        assert_eq!(sqlite3_open(p.as_ptr(), &mut db2), 0);
        unsafe extern "C" fn cb(arg: *mut c_void, argc: c_int, argv: *mut *mut c_char, _az: *mut *mut c_char) -> c_int {
            let out = &mut *(arg as *mut Vec<String>);
            for i in 0..argc as usize {
                let pp = *argv.add(i);
                out.push(if pp.is_null() { "NULL".into() } else { CStr::from_ptr(pp).to_str().unwrap().into() });
            }
            0
        }
        let sql = CString::new("SELECT a FROM r;").unwrap();
        let mut got: Vec<String> = Vec::new();
        sqlite3_exec(db2, sql.as_ptr(), Some(cb), &mut got as *mut Vec<String> as *mut c_void, ptr::null_mut());
        assert_eq!(got, vec![n.to_string()]);
        sqlite3_close(db2);
    }
    let _ = std::fs::remove_file(&path);
}
