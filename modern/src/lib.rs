//! Rust spine — pack `sqlite-experiment-c-to-rust@1` (BOUND).
//!
//! Implements EXACTLY the ten HUMAN_ACCEPTED characterization cases on the raw
//! C ABI (`sqlite3_*` names, integer codes from `src/sqlite.h.in`, read-only
//! reference). This is a recognizer + statement state machine for the pinned
//! SQL — NOT a general SQL engine, NOT a VDBE, and it never links C.
//!
//! Pinned semantics encoded here (see PACK.yaml):
//! - autoreset: a further `sqlite3_step` after DONE returns SQLITE_ROW (100)
//! - `sqlite3_reset` preserves bindings
//! - bind index out of range -> SQLITE_RANGE (25)
//! - whitespace/comment-only SQL -> SQLITE_OK + NULL stmt, tail consumed
//! - NULL-handle errcode/errmsg -> 7 / "out of memory" (contract string)
//! - errmsg wording for syntax errors is wording_deferred (shape emitted,
//!   never a parity contract)
//!
//! `prepare-statement-api-002-C003` (step after finalize) is BLOCKED/unmapped:
//! there is deliberately no code path, test, or probe for it.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

pub const SQLITE_OK: c_int = 0;
pub const SQLITE_ERROR: c_int = 1;
pub const SQLITE_NOMEM: c_int = 7;
pub const SQLITE_MISUSE: c_int = 21; // internal guard paths only; not a pinned observable
pub const SQLITE_RANGE: c_int = 25;
pub const SQLITE_ROW: c_int = 100;
pub const SQLITE_DONE: c_int = 101;

static OUT_OF_MEMORY: &[u8] = b"out of memory\0"; // contract string (sqlite3ErrStr twin)
static NOT_AN_ERROR: &[u8] = b"not an error\0";

pub struct Sqlite3 {
    errcode: c_int,
    extended: c_int,
    errmsg: Option<CString>,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    SelectOne,   // "SELECT 1"
    SelectParam, // "SELECT ?"
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Ready,
    Row,
    Done,
}

pub struct Sqlite3Stmt {
    kind: Kind,
    state: State,
    bound: Option<i64>, // preserved across reset (pinned)
    param_count: usize,
}

fn db_ok(db: &mut Sqlite3) {
    db.errcode = SQLITE_OK;
    db.extended = SQLITE_OK;
    db.errmsg = None;
}

fn db_syntax_error(db: &mut Sqlite3, token: &str) {
    db.errcode = SQLITE_ERROR;
    db.extended = SQLITE_ERROR;
    // wording_deferred: shape mirrors the legacy message; never a parity contract
    db.errmsg = Some(CString::new(format!("near \"{token}\": syntax error")).unwrap());
}

/// Strip leading whitespace and `--` line comments (the pinned input shapes).
fn skip_ws_and_comments(mut s: &str) -> &str {
    loop {
        s = s.trim_start();
        if let Some(rest) = s.strip_prefix("--") {
            s = match rest.find('\n') {
                Some(i) => &rest[i + 1..],
                None => "",
            };
        } else {
            return s;
        }
    }
}

fn first_token(s: &str) -> &str {
    let end = s
        .find(|c: char| c.is_whitespace() || c == ';')
        .unwrap_or(s.len());
    &s[..end]
}

/// # Safety: C ABI — `pp_db` must be a valid out-pointer.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_open(_filename: *const c_char, pp_db: *mut *mut Sqlite3) -> c_int {
    if pp_db.is_null() {
        return SQLITE_MISUSE;
    }
    let db = Box::new(Sqlite3 { errcode: SQLITE_OK, extended: SQLITE_OK, errmsg: None });
    *pp_db = Box::into_raw(db);
    SQLITE_OK
}

/// # Safety: C ABI — `db` from sqlite3_open or NULL.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_close(db: *mut Sqlite3) -> c_int {
    if !db.is_null() {
        drop(Box::from_raw(db));
    }
    SQLITE_OK
}

/// # Safety: C ABI — NULL db is the pinned guarded path (returns SQLITE_NOMEM).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_errcode(db: *mut Sqlite3) -> c_int {
    if db.is_null() {
        return SQLITE_NOMEM; // pinned: errcode(NULL) == 7
    }
    (*db).errcode
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_extended_errcode(db: *mut Sqlite3) -> c_int {
    if db.is_null() {
        return SQLITE_NOMEM;
    }
    (*db).extended
}

/// # Safety: C ABI — returned pointer valid until the next API call on `db`.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_errmsg(db: *mut Sqlite3) -> *const c_char {
    if db.is_null() {
        return OUT_OF_MEMORY.as_ptr() as *const c_char; // contract string
    }
    match &(*db).errmsg {
        Some(m) => m.as_ptr(),
        None => NOT_AN_ERROR.as_ptr() as *const c_char,
    }
}

/// # Safety: C ABI — mirrors sqlite3_prepare_v2 for the pinned inputs.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare_v2(
    db: *mut Sqlite3,
    z_sql: *const c_char,
    n_byte: c_int,
    pp_stmt: *mut *mut Sqlite3Stmt,
    pz_tail: *mut *const c_char,
) -> c_int {
    if db.is_null() || z_sql.is_null() || pp_stmt.is_null() {
        return SQLITE_MISUSE;
    }
    *pp_stmt = ptr::null_mut();
    let full = CStr::from_ptr(z_sql).to_bytes();
    let full = if n_byte >= 0 && (n_byte as usize) < full.len() {
        &full[..n_byte as usize]
    } else {
        full
    };
    let sql = match std::str::from_utf8(full) {
        Ok(s) => s,
        Err(_) => {
            db_syntax_error(&mut *db, "?");
            return SQLITE_ERROR;
        }
    };
    let end_ptr = z_sql.add(sql.len()); // the terminating NUL / end of input

    let body = skip_ws_and_comments(sql);
    if body.is_empty() {
        // pinned: whitespace/comment-only SQL -> OK + NULL stmt, tail consumed
        db_ok(&mut *db);
        if !pz_tail.is_null() {
            *pz_tail = end_ptr;
        }
        return SQLITE_OK;
    }

    let stmt_text = body.trim_end().trim_end_matches(';').trim_end();
    let kind = if stmt_text.eq_ignore_ascii_case("SELECT 1") {
        Kind::SelectOne
    } else if stmt_text.eq_ignore_ascii_case("SELECT ?") {
        Kind::SelectParam
    } else {
        // recognizer-not-engine (pack known_risk): everything unpinned is a syntax error
        db_syntax_error(&mut *db, first_token(body));
        if !pz_tail.is_null() {
            *pz_tail = z_sql; // unconsumed on error (not a pinned observable)
        }
        return SQLITE_ERROR;
    };

    db_ok(&mut *db);
    let stmt = Box::new(Sqlite3Stmt {
        kind,
        state: State::Ready,
        bound: None,
        param_count: if kind == Kind::SelectParam { 1 } else { 0 },
    });
    *pp_stmt = Box::into_raw(stmt);
    if !pz_tail.is_null() {
        *pz_tail = end_ptr; // single pinned statements consume the whole input
    }
    SQLITE_OK
}

/// # Safety: C ABI — `stmt` must be live (never call after sqlite3_finalize; C003 is BLOCKED).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_step(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() {
        return SQLITE_MISUSE;
    }
    let s = &mut *stmt;
    match s.state {
        State::Ready => {
            s.state = State::Row;
            SQLITE_ROW
        }
        State::Row => {
            s.state = State::Done;
            SQLITE_DONE
        }
        State::Done => {
            // pinned autoreset (OMIT_AUTORESET=off): step after DONE re-runs -> ROW
            s.state = State::Row;
            SQLITE_ROW
        }
    }
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_int(stmt: *mut Sqlite3Stmt, i_col: c_int) -> c_int {
    if stmt.is_null() || i_col != 0 {
        return 0;
    }
    let s = &*stmt;
    if s.state != State::Row {
        return 0;
    }
    match s.kind {
        Kind::SelectOne => 1,
        Kind::SelectParam => s.bound.unwrap_or(0) as c_int, // unbound param evaluates NULL -> 0
    }
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_int(stmt: *mut Sqlite3Stmt, idx: c_int, value: c_int) -> c_int {
    if stmt.is_null() {
        return SQLITE_MISUSE;
    }
    let s = &mut *stmt;
    if idx < 1 || (idx as usize) > s.param_count {
        return SQLITE_RANGE; // pinned: 25
    }
    s.bound = Some(value as i64);
    SQLITE_OK
}

/// # Safety: C ABI — bindings preserved (pinned).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_reset(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() {
        return SQLITE_OK;
    }
    (*stmt).state = State::Ready; // bound value intentionally kept
    SQLITE_OK
}

/// # Safety: C ABI — frees the handle; the pointer is dead afterwards.
/// There is deliberately NO safe re-entry: C003 (step after finalize) is BLOCKED/unmapped.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_finalize(stmt: *mut Sqlite3Stmt) -> c_int {
    if !stmt.is_null() {
        drop(Box::from_raw(stmt));
    }
    SQLITE_OK
}
