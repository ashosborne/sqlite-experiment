//! Run-15 engine-files golden replays: drive each durable round-trip through the
//! Rust file path and assert the OBS equals the frozen C golden (read-only).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

fn golden(cnum: &str) -> String {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.push(format!("tests/characterization/engine-files/cases/engine-files-001/{cnum}.approved.txt"));
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("golden {p:?}: {e}"))
}

fn tmp(tag: &str) -> String {
    let p = format!("/tmp/eftest/fc_{}_{}.db", tag, std::process::id());
    let _ = std::fs::remove_file(&p);
    p
}

unsafe fn wr(lines: &mut Vec<String>, cid: &str, label: &str, path: &str, sql: &str) {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let p = CString::new(path).unwrap();
    let mut rc = sqlite3_open(p.as_ptr(), &mut db);
    if rc == 0 {
        let c = CString::new(sql).unwrap();
        rc = sqlite3_exec(db, c.as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    }
    sqlite3_close(db);
    lines.push(format!("OBS {cid} {label} {rc}"));
}

struct Cap<'a> { cid: &'a str, rows: i32, lines: &'a mut Vec<String> }
unsafe extern "C" fn cb(arg: *mut c_void, argc: c_int, argv: *mut *mut c_char, _az: *mut *mut c_char) -> c_int {
    let cap = &mut *(arg as *mut Cap);
    for i in 0..argc as usize {
        let p = *argv.add(i);
        let v = if p.is_null() { "NULL".to_string() } else { CStr::from_ptr(p).to_str().unwrap().to_string() };
        cap.lines.push(format!("OBS {} row{}.col{} {}", cap.cid, cap.rows, i, v));
    }
    cap.rows += 1;
    0
}

unsafe fn rd(lines: &mut Vec<String>, cid: &str, path: &str, sql: &str) {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let p = CString::new(path).unwrap();
    let orc = sqlite3_open(p.as_ptr(), &mut db);
    lines.push(format!("OBS {cid} reopen.rc {orc}"));
    let mut cap = Cap { cid, rows: 0, lines };
    let c = CString::new(sql).unwrap();
    let rc = sqlite3_exec(db, c.as_ptr(), Some(cb), &mut cap as *mut Cap as *mut c_void, ptr::null_mut());
    let rows = cap.rows;
    lines.push(format!("OBS {cid} read.rc {rc}"));
    lines.push(format!("OBS {cid} cb.rows {rows}"));
    sqlite3_close(db);
}

#[test]
fn engine_files_c001() {
    let cid = "engine-files-001-C001"; let path = tmp("c001"); let mut l = Vec::new();
    unsafe {
        wr(&mut l, cid, "write.rc", &path, "CREATE TABLE f(a INTEGER); INSERT INTO f VALUES(7);");
        rd(&mut l, cid, &path, "SELECT a FROM f;");
    }
    assert_eq!(l.join("\n") + "\n", golden("C001"));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn engine_files_c002() {
    let cid = "engine-files-001-C002"; let path = tmp("c002"); let mut l = Vec::new();
    unsafe {
        wr(&mut l, cid, "write.rc", &path,
           "CREATE TABLE f2(a INTEGER, b TEXT); INSERT INTO f2 VALUES(1,'x'); INSERT INTO f2 VALUES(2,'y');");
        rd(&mut l, cid, &path, "SELECT a,b FROM f2 ORDER BY a;");
    }
    assert_eq!(l.join("\n") + "\n", golden("C002"));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn engine_files_c003() {
    let cid = "engine-files-001-C003"; let path = tmp("c003"); let mut l = Vec::new();
    unsafe {
        wr(&mut l, cid, "write.rc", &path, "CREATE TABLE f3(a INTEGER); INSERT INTO f3 VALUES(1),(2),(3);");
        wr(&mut l, cid, "write2.rc", &path, "UPDATE f3 SET a=a+10 WHERE a=2; DELETE FROM f3 WHERE a=1;");
        rd(&mut l, cid, &path, "SELECT a FROM f3 ORDER BY a;");
    }
    assert_eq!(l.join("\n") + "\n", golden("C003"));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn engine_files_c004() {
    let cid = "engine-files-001-C004"; let path = tmp("c004"); let mut l = Vec::new();
    unsafe {
        wr(&mut l, cid, "write.rc", &path, "CREATE TABLE f4(a INTEGER); INSERT INTO f4 VALUES(777001);");
        rd(&mut l, cid, &path, "SELECT a FROM f4;");
    }
    assert_eq!(l.join("\n") + "\n", golden("C004"));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn engine_files_c005() {
    let cid = "engine-files-001-C005"; let path = tmp("c005"); let mut l = Vec::new();
    unsafe {
        wr(&mut l, cid, "write.rc", &path, "CREATE TABLE f5(a INTEGER); INSERT INTO f5 VALUES(424243);");
        rd(&mut l, cid, &path, "SELECT a FROM f5;");
    }
    assert_eq!(l.join("\n") + "\n", golden("C005"));
    let _ = std::fs::remove_file(&path);
}
