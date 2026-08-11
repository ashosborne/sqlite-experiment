//! Self-check against the ten HUMAN_ACCEPTED goldens (pack sqlite-experiment-c-to-rust@1).
//!
//! Reads the frozen `*.approved.txt` files (READ-ONLY) and drives the same
//! sequences the C harnesses recorded, asserting every integer/handle
//! observable. `errmsg.text` on error-status-api-001-C001 is wording_deferred
//! and excluded from failure; the NULL-handle `out of memory` string IS a
//! contract and is asserted exactly.
//!
//! This is a branch self-check, NOT factory Verification COMPARE — it never
//! flips parity_green. There is deliberately no test for 002-C003 (BLOCKED).

use sqlite3_rust_spine::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::ptr;

fn golden(rel: &str) -> HashMap<String, String> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // workspace root
    p.push("tests/characterization");
    p.push(rel);
    let text = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {p:?}: {e}"));
    let mut m = HashMap::new();
    for line in text.lines() {
        // OBS <tag> <name> <value...>
        let mut it = line.splitn(4, ' ');
        let (obs, _tag, name) = (it.next(), it.next(), it.next());
        assert_eq!(obs, Some("OBS"), "golden line shape: {line}");
        let value = it.next().unwrap_or("").to_string();
        m.insert(name.unwrap().to_string(), value);
    }
    m
}

fn gi(g: &HashMap<String, String>, key: &str) -> i32 {
    g[key].parse().unwrap_or_else(|_| panic!("golden {key} not int: {}", g[key]))
}

unsafe fn open_mem() -> *mut Sqlite3 {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let name = CString::new(":memory:").unwrap();
    assert_eq!(sqlite3_open(name.as_ptr(), &mut db), 0);
    db
}

unsafe fn cstr<'a>(p: *const c_char) -> &'a str {
    CStr::from_ptr(p).to_str().unwrap()
}

#[test]
fn error_status_api_001_c001_syntax_error_state() {
    let g = golden("error-status-api/cases/error-status-api-001/C001.approved.txt");
    unsafe {
        let db = open_mem();
        let sql = CString::new("SELECTT 1").unwrap();
        let mut stmt: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, ptr::null_mut());
        assert_eq!(rc, gi(&g, "prepare.rc"), "prepare.rc");
        assert_eq!(sqlite3_errcode(db), gi(&g, "errcode.value"), "errcode");
        assert_eq!(sqlite3_extended_errcode(db), gi(&g, "extended_errcode.value"), "extended_errcode");
        // wording_deferred: shape-only — must be non-empty, wording NOT asserted vs golden
        let msg = cstr(sqlite3_errmsg(db));
        assert!(!msg.is_empty(), "errmsg present (wording_deferred, not compared)");
        assert!(stmt.is_null(), "failed prepare yields NULL stmt");
        sqlite3_close(db);
    }
}

#[test]
fn error_status_api_001_c002_null_handle_contract() {
    let g = golden("error-status-api/cases/error-status-api-001/C002.approved.txt");
    unsafe {
        assert_eq!(sqlite3_errcode(ptr::null_mut()), gi(&g, "errcode(NULL).value"));
        // contract string — asserted exactly against the golden bytes
        assert_eq!(cstr(sqlite3_errmsg(ptr::null_mut())), g["errmsg(NULL).text"]);
    }
}

#[test]
fn prepare_statement_api_002_c001_c002_step_machine_with_autoreset() {
    let g1 = golden("prepare-statement-api/cases/prepare-statement-api-002/C001.approved.txt");
    let g2 = golden("prepare-statement-api/cases/prepare-statement-api-002/C002.approved.txt");
    unsafe {
        let db = open_mem();
        let sql = CString::new("SELECT 1").unwrap();
        let mut stmt: *mut Sqlite3Stmt = ptr::null_mut();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, ptr::null_mut()), gi(&g1, "prepare.rc"));
        assert_eq!(sqlite3_step(stmt), gi(&g1, "step1.rc"), "ROW");
        assert_eq!(sqlite3_column_int(stmt, 0), gi(&g1, "column_int.value"));
        assert_eq!(sqlite3_step(stmt), gi(&g1, "step2.rc"), "DONE");
        // C002: third step, no reset -> autoreset ROW (pinned; NOT MISUSE)
        assert_eq!(sqlite3_step(stmt), gi(&g2, "step3_after_done.rc"), "autoreset ROW");
        sqlite3_finalize(stmt);
        sqlite3_close(db);
    }
}

#[test]
fn prepare_statement_api_001_c001_valid_prepare() {
    let g = golden("prepare-statement-api/cases/prepare-statement-api-001/C001.approved.txt");
    unsafe {
        let db = open_mem();
        let sql = CString::new("SELECT 1").unwrap();
        let mut stmt: *mut Sqlite3Stmt = ptr::null_mut();
        let mut tail: *const c_char = ptr::null();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, &mut tail), gi(&g, "prepare.rc"));
        assert_eq!((!stmt.is_null()) as i32, gi(&g, "stmt.nonnull"));
        assert_eq!((!tail.is_null() && *tail == 0) as i32, gi(&g, "pzTail.consumed"));
        sqlite3_finalize(stmt);
        sqlite3_close(db);
    }
}

#[test]
fn prepare_statement_api_001_c002_whitespace_comment_only() {
    let g = golden("prepare-statement-api/cases/prepare-statement-api-001/C002.approved.txt");
    unsafe {
        let db = open_mem();
        let sql = CString::new("  -- just a comment\n  ").unwrap();
        let mut stmt: *mut Sqlite3Stmt = ptr::null_mut();
        let mut tail: *const c_char = ptr::null();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, &mut tail), gi(&g, "prepare.rc"));
        assert_eq!(stmt.is_null() as i32, gi(&g, "stmt.isnull"));
        let rest = if !tail.is_null() && *tail != 0 { cstr(tail).to_string() } else { "(consumed)".to_string() };
        assert_eq!(rest, g["pzTail.rest"]);
        sqlite3_close(db);
    }
}

#[test]
fn prepare_statement_api_003_bind_and_range() {
    let g1 = golden("prepare-statement-api/cases/prepare-statement-api-003/C001.approved.txt");
    let g2 = golden("prepare-statement-api/cases/prepare-statement-api-003/C002.approved.txt");
    unsafe {
        let db = open_mem();
        let sql = CString::new("SELECT ?").unwrap();
        let mut stmt: *mut Sqlite3Stmt = ptr::null_mut();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, ptr::null_mut()), gi(&g1, "prepare.rc"));
        assert_eq!(sqlite3_bind_int(stmt, 1, 7), gi(&g1, "bind.rc"));
        assert_eq!(sqlite3_step(stmt), gi(&g1, "step.rc"), "ROW");
        assert_eq!(sqlite3_column_int(stmt, 0), gi(&g1, "column_int.value"));
        // C002: same stmt after reset, bind index 2 of 1 -> SQLITE_RANGE (pinned 25)
        sqlite3_reset(stmt);
        assert_eq!(sqlite3_bind_int(stmt, 2, 7), gi(&g2, "bind_oor.rc"));
        sqlite3_finalize(stmt);
        sqlite3_close(db);
    }
}

#[test]
fn prepare_statement_api_005_c001_reset_preserves_bindings() {
    let g = golden("prepare-statement-api/cases/prepare-statement-api-005/C001.approved.txt");
    unsafe {
        let db = open_mem();
        let sql = CString::new("SELECT ?").unwrap();
        let mut stmt: *mut Sqlite3Stmt = ptr::null_mut();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, ptr::null_mut()), gi(&g, "prepare.rc"));
        assert_eq!(sqlite3_bind_int(stmt, 1, 42), gi(&g, "bind.rc"));
        assert_eq!(sqlite3_step(stmt), gi(&g, "step_row.rc"), "ROW");
        assert_eq!(sqlite3_step(stmt), gi(&g, "step_done.rc"), "DONE");
        assert_eq!(sqlite3_reset(stmt), gi(&g, "reset.rc"));
        assert_eq!(sqlite3_step(stmt), gi(&g, "post_reset_step.rc"), "ROW after explicit reset, no re-bind");
        assert_eq!(sqlite3_column_int(stmt, 0), gi(&g, "column_after_reset.value"), "binding preserved");
        sqlite3_finalize(stmt);
        sqlite3_close(db);
    }
}

#[test]
fn prepare_statement_api_005_c002_finalize_live_statement() {
    let g = golden("prepare-statement-api/cases/prepare-statement-api-005/C002.approved.txt");
    unsafe {
        let db = open_mem();
        let sql = CString::new("SELECT 1").unwrap();
        let mut stmt: *mut Sqlite3Stmt = ptr::null_mut();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, ptr::null_mut()), gi(&g, "prepare.rc"));
        assert_eq!(sqlite3_step(stmt), gi(&g, "step_row.rc"), "live, mid-row");
        assert_eq!(sqlite3_finalize(stmt), gi(&g, "finalize.rc"));
        // handle is dead — deliberately never touched again (C003 is BLOCKED, no probe exists)
        sqlite3_close(db);
    }
}
