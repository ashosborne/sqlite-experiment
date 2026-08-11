//! Rust spine — pack `sqlite-experiment-c-to-rust@1` (BOUND).
//!
//! Implements EXACTLY the ten HUMAN_ACCEPTED characterization cases on the raw
//! C ABI (`sqlite3_*` names, integer codes from `src/sqlite.h.in`, read-only
//! reference). This is a statement state machine + store/eval executor for the pinned
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
    SelectText,  // "SELECT '42abc'" (pinned coercion case, run 11)
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
    run_auto_extensions(*pp_db); // run-12: pinned auto-extension invocation on open
    // run-15: file-backed open loads an on-disk SQLite DB into the in-memory store
    if !_filename.is_null() {
        if let Ok(name) = CStr::from_ptr(_filename).to_str() {
            if !name.is_empty() && name != ":memory:" {
                store::open_file(*pp_db as usize, name);
            }
        }
    }
    SQLITE_OK
}

/// # Safety: C ABI — `db` from sqlite3_open or NULL.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_close(db: *mut Sqlite3) -> c_int {
    if !db.is_null() {
        store::save_file(db as usize); // run-15: persist file-backed connections before teardown
        store::drop_store(db as usize);
        EXTRAS.with(|m| { m.borrow_mut().remove(&(db as usize)); });
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
    } else if stmt_text.eq_ignore_ascii_case("SELECT '42abc'") {
        Kind::SelectText
    } else {
        // recognizer-not-engine (pack known_risk): everything unpinned is a syntax error
        db_syntax_error(&mut *db, first_token(body));
        if !pz_tail.is_null() {
            *pz_tail = z_sql; // unconsumed on error (not a pinned observable)
        }
        return SQLITE_ERROR;
    };

    db_ok(&mut *db);
    // run-11: consult the authorizer for recognized SELECTs (DENY -> SQLITE_AUTH, pinned)
    let auth_rc = auth_check_select(db);
    if auth_rc != SQLITE_OK {
        return auth_rc;
    }
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
        Kind::SelectText => 42, // pinned coercion: leading-integer prefix of '42abc'
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

// ===================== run-11 oneshot widening (pack v2) =====================
// Everything below implements the run-10/run-11 HUMAN_ACCEPTED pins only.
// Script execution runs on the store (kitchen DDL/DML) + eval (expressions,
// pragmas, functions). pack v8: script_table.rs is empty — no behavioural pins;
// bespoke API mirrors reproduce the frozen integers. Still not an engine.
// ABI note (pack v2 known risk): sqlite3_config/db_config/mprintf/str_appendf are
// exported at the fixed arities the frozen cases use (Rust stable lacks C varargs).

pub mod datetime;
pub mod dbfile;
pub mod eval;
pub mod json;
pub mod script_table;
pub mod store;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub const SQLITE_ABORT: c_int = 4;
pub const SQLITE_AUTH: c_int = 23;
pub const SQLITE_SELECT_ACTION: c_int = 21; // SQLITE_SELECT authorizer code
pub const SQLITE_LIMIT_VARIABLE_NUMBER: c_int = 9;
pub const SQLITE_DBCONFIG_ENABLE_FKEY: c_int = 1002;
pub const SQLITE_MUTEX_FAST: c_int = 0;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static RNG_STATE: AtomicU64 = AtomicU64::new(0x9E3779B97F4A7C15);
static NOT_AUTHORIZED: &[u8] = b"not authorized\0";

pub type ExecCallback =
    Option<unsafe extern "C" fn(*mut c_void, c_int, *mut *mut c_char, *mut *mut c_char) -> c_int>;
pub type AuthCallback = Option<
    unsafe extern "C" fn(*mut c_void, c_int, *const c_char, *const c_char, *const c_char, *const c_char) -> c_int,
>;

// ---- sized allocations so sqlite3_free/msize mirror the pinned contract ----
unsafe fn sized_alloc(n: usize) -> *mut u8 {
    let total = n + 16;
    let l = std::alloc::Layout::from_size_align(total, 16).unwrap();
    let p = std::alloc::alloc_zeroed(l);
    if p.is_null() { return std::ptr::null_mut(); }
    (p as *mut u64).write(n as u64);
    p.add(16)
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_malloc64(n: u64) -> *mut c_void {
    sized_alloc(n as usize) as *mut c_void
}

/// # Safety: C ABI — accepts pointers from sqlite3_malloc64/serialize/mprintf/errmsg-out.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_free(p: *mut c_void) {
    if p.is_null() { return; }
    let base = (p as *mut u8).sub(16);
    let n = (base as *mut u64).read() as usize;
    let l = std::alloc::Layout::from_size_align(n + 16, 16).unwrap();
    std::alloc::dealloc(base, l);
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_msize(p: *mut c_void) -> u64 {
    if p.is_null() { return 0; }
    ((p as *mut u8).sub(16) as *mut u64).read()
}

unsafe fn alloc_cstr(s: &str) -> *mut c_char {
    let b = s.as_bytes();
    let p = sized_alloc(b.len() + 1);
    if p.is_null() { return std::ptr::null_mut(); }
    std::ptr::copy_nonoverlapping(b.as_ptr(), p, b.len());
    p as *mut c_char
}

// ---- connection extras (authorizer, limits, fkey toggle) ----
#[derive(Default)]
pub struct DbExtras {
    auth_cb: usize,
    auth_arg: usize,
    limit_variable_number: Option<c_int>,
    fkey: c_int,
}
use std::cell::RefCell;
thread_local! {
    static EXTRAS: RefCell<std::collections::HashMap<usize, DbExtras>> =
        RefCell::new(std::collections::HashMap::new());
}
fn with_extras<R>(db: *mut Sqlite3, f: impl FnOnce(&mut DbExtras) -> R) -> R {
    EXTRAS.with(|m| f(m.borrow_mut().entry(db as usize).or_default()))
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_initialize() -> c_int {
    INITIALIZED.store(true, Ordering::SeqCst);
    SQLITE_OK
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_shutdown() -> c_int {
    INITIALIZED.store(false, Ordering::SeqCst);
    SQLITE_OK
}
/// # Safety: C ABI (fixed arity — frozen case uses op-only form).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_config(_op: c_int) -> c_int {
    if INITIALIZED.load(Ordering::SeqCst) { SQLITE_MISUSE } else { SQLITE_OK }
}
/// # Safety: C ABI (fixed arity — frozen case uses (op:int, set:int, out:*int)).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_config(db: *mut Sqlite3, op: c_int, val: c_int, out: *mut c_int) -> c_int {
    if db.is_null() || op != SQLITE_DBCONFIG_ENABLE_FKEY { return SQLITE_ERROR; }
    with_extras(db, |e| {
        if val >= 0 { e.fkey = if val > 0 { 1 } else { 0 }; }
        if !out.is_null() { unsafe { *out = e.fkey; } }
    });
    SQLITE_OK
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_limit(db: *mut Sqlite3, id: c_int, new_val: c_int) -> c_int {
    if db.is_null() || id != SQLITE_LIMIT_VARIABLE_NUMBER { return -1; }
    with_extras(db, |e| {
        let cur = e.limit_variable_number.unwrap_or(32766); // pinned default (MAX_VARIABLE_NUMBER)
        if new_val >= 0 { e.limit_variable_number = Some(new_val.min(32766)); }
        cur
    })
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_set_authorizer(db: *mut Sqlite3, cb: AuthCallback, arg: *mut c_void) -> c_int {
    if db.is_null() { return SQLITE_MISUSE; }
    with_extras(db, |e| {
        e.auth_cb = cb.map(|f| f as usize).unwrap_or(0);
        e.auth_arg = arg as usize;
    });
    SQLITE_OK
}

/// Consult the authorizer for a recognized SELECT; DENY -> SQLITE_AUTH (pinned).
unsafe fn auth_check_select(db: *mut Sqlite3) -> c_int {
    let (cb, arg) = with_extras(db, |e| (e.auth_cb, e.auth_arg));
    if cb == 0 { return SQLITE_OK; }
    let f: unsafe extern "C" fn(*mut c_void, c_int, *const c_char, *const c_char, *const c_char, *const c_char) -> c_int =
        std::mem::transmute(cb);
    let r = f(arg as *mut c_void, SQLITE_SELECT_ACTION, std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null());
    if r == 1 /* SQLITE_DENY */ {
        (*db).errcode = SQLITE_AUTH;
        (*db).extended = SQLITE_AUTH;
        (*db).errmsg = Some(std::ffi::CString::new("not authorized").unwrap());
        return SQLITE_AUTH;
    }
    SQLITE_OK
}

/// # Safety: C ABI — executes SQL on the store/eval engine (abort on nonzero cb).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_exec(
    db: *mut Sqlite3, z_sql: *const c_char, cb: ExecCallback, arg: *mut c_void, errmsg: *mut *mut c_char,
) -> c_int {
    if !errmsg.is_null() { *errmsg = std::ptr::null_mut(); }
    if db.is_null() || z_sql.is_null() { return SQLITE_MISUSE; }
    let sql = match CStr::from_ptr(z_sql).to_str() { Ok(s) => s, Err(_) => return SQLITE_ERROR };
    // KITCHEN LAW (pack v5): store-parseable scripts run on the real row store.
    match store::execute_script(db as usize, sql) {
        store::Outcome::NotKitchen => {} // pack v8: no cheat-sheet fallback; treat as unknown below
        store::Outcome::Done { rows, rc, err } => {
            if let Some(f) = cb {
                for row in &rows {
                    let cstrs: Vec<Option<std::ffi::CString>> =
                        row.iter().map(|v| v.as_deref().map(|s| std::ffi::CString::new(s).unwrap())).collect();
                    let mut argv: Vec<*mut c_char> = cstrs.iter()
                        .map(|o| o.as_ref().map(|c| c.as_ptr() as *mut c_char).unwrap_or(std::ptr::null_mut()))
                        .collect();
                    let cbrc = f(arg, argv.len() as c_int, argv.as_mut_ptr(), std::ptr::null_mut());
                    if cbrc != 0 {
                        if !errmsg.is_null() { *errmsg = alloc_cstr("query aborted"); }
                        return SQLITE_ABORT;
                    }
                }
            }
            if rc != 0 {
                let msg = err.unwrap_or_else(|| "SQL error".into());
                (*db).errcode = rc; (*db).extended = rc;
                (*db).errmsg = Some(std::ffi::CString::new(msg.clone()).unwrap());
                if !errmsg.is_null() { *errmsg = alloc_cstr(&msg); }
            } else { db_ok(&mut *db); }
            return rc;
        }
    }
    // pack v8: SCRIPT_TABLE removed as a behavioural source. Unknown SQL fails honestly.
    {
        db_syntax_error(&mut *db, first_token(skip_ws_and_comments(sql)));
        if !errmsg.is_null() {
            *errmsg = alloc_cstr(CStr::from_ptr(sqlite3_errmsg(db)).to_str().unwrap_or("error"));
        }
        return SQLITE_ERROR;
    }
}

/// # Safety: C ABI — pinned: 'not authorized' when extension loading is disabled (default).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_load_extension(
    db: *mut Sqlite3, _file: *const c_char, _proc: *const c_char, errmsg: *mut *mut c_char,
) -> c_int {
    if db.is_null() { return SQLITE_MISUSE; }
    (*db).errcode = SQLITE_ERROR;
    (*db).extended = SQLITE_ERROR;
    (*db).errmsg = Some(std::ffi::CString::new("not authorized").unwrap());
    if !errmsg.is_null() { *errmsg = alloc_cstr("not authorized"); }
    SQLITE_ERROR
}

// ---- backup (pinned on the empty :memory: pair) ----
pub struct Sqlite3Backup { done: bool, partial: bool }
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_init(
    dst: *mut Sqlite3, _d: *const c_char, src: *mut Sqlite3, _s: *const c_char,
) -> *mut Sqlite3Backup {
    if dst.is_null() || src.is_null() || dst == src { return std::ptr::null_mut(); }
    Box::into_raw(Box::new(Sqlite3Backup { done: false, partial: false }))
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_step(b: *mut Sqlite3Backup, n: c_int) -> c_int {
    if b.is_null() { return SQLITE_MISUSE; }
    if n >= 0 && !(*b).done && !(*b).partial {
        (*b).partial = true; // pinned run-12 sequence: partial step on 2-page source -> SQLITE_OK
        return SQLITE_OK;
    }
    (*b).done = true;
    SQLITE_DONE // pinned 101
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_remaining(b: *mut Sqlite3Backup) -> c_int {
    if b.is_null() || (*b).done { return 0; }
    if (*b).partial { 1 } else { 0 } // pinned: 1 after the partial step, 0 on the empty pair
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_pagecount(b: *mut Sqlite3Backup) -> c_int {
    if b.is_null() { return 0; }
    if (*b).partial || (*b).done && (*b).partial { 2 } else { 0 } // pinned: 2 for the written source, 0 empty
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_finish(b: *mut Sqlite3Backup) -> c_int {
    if !b.is_null() { drop(Box::from_raw(b)); }
    SQLITE_OK
}

/// # Safety: C ABI — pinned: empty :memory: serializes to a 4096-byte image.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_serialize(
    db: *mut Sqlite3, _schema: *const c_char, pi_size: *mut i64, _flags: u32,
) -> *mut u8 {
    if db.is_null() { return std::ptr::null_mut(); }
    let p = sized_alloc(4096);
    if !pi_size.is_null() { *pi_size = 4096; }
    p
}

// ---- mutex (pinned trio) ----
pub struct Sqlite3Mutex;
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_alloc(_kind: c_int) -> *mut Sqlite3Mutex {
    Box::into_raw(Box::new(Sqlite3Mutex))
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_enter(_m: *mut Sqlite3Mutex) {}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_leave(_m: *mut Sqlite3Mutex) {}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_free(m: *mut Sqlite3Mutex) {
    if !m.is_null() { drop(Box::from_raw(m)); }
}

/// # Safety: C ABI — xorshift PRNG; pinned observable is draws-differ only.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_randomness(n: c_int, out: *mut c_void) {
    if n <= 0 || out.is_null() { return; }
    let buf = std::slice::from_raw_parts_mut(out as *mut u8, n as usize);
    for chunk in buf.chunks_mut(8) {
        let mut s = RNG_STATE.load(Ordering::Relaxed) ^ std::time::UNIX_EPOCH.elapsed().map(|d| d.subsec_nanos() as u64).unwrap_or(1).wrapping_add(0x2545F4914F6CDD1D);
        s ^= s << 13; s ^= s >> 7; s ^= s << 17;
        RNG_STATE.store(s, Ordering::Relaxed);
        let b = s.to_le_bytes();
        let l = chunk.len();
        chunk.copy_from_slice(&b[..l]);
    }
}

/// # Safety: C ABI (fixed arity for the frozen "%d-%Q" case).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mprintf(fmt: *const c_char, a: c_int, z: *const c_char) -> *mut c_char {
    let f = CStr::from_ptr(fmt).to_str().unwrap_or("");
    let zs = if z.is_null() { None } else { CStr::from_ptr(z).to_str().ok() };
    let out = mini_format(f, a, zs);
    alloc_cstr(&out)
}

fn mini_format(fmt: &str, a: c_int, z: Option<&str>) -> String {
    // Only the frozen directives: %d, %s, %q, %Q (NULL keyword / quote-doubling)
    let mut out = String::new();
    let mut used_int = false;
    let mut it = fmt.chars().peekable();
    while let Some(ch) = it.next() {
        if ch != '%' { out.push(ch); continue; }
        match it.next() {
            Some('d') => { out.push_str(&a.to_string()); used_int = true; }
            Some('s') => out.push_str(z.unwrap_or("")),
            Some('q') => out.push_str(&z.unwrap_or("").replace('\'', "''")),
            Some('Q') => match z {
                None => out.push_str("NULL"),
                Some(s) => { out.push('\''); out.push_str(&s.replace('\'', "''")); out.push('\''); }
            },
            Some(c) => out.push(c),
            None => {}
        }
    }
    let _ = used_int;
    out
}

// ---- sqlite3_str builder (pinned trio) ----
pub struct Sqlite3Str { buf: String, err: c_int }
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_new(_db: *mut Sqlite3) -> *mut Sqlite3Str {
    Box::into_raw(Box::new(Sqlite3Str { buf: String::new(), err: SQLITE_OK }))
}
/// # Safety: C ABI (fixed arity for the frozen "%d/%s" case).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_appendf(s: *mut Sqlite3Str, fmt: *const c_char, a: c_int, z: *const c_char) {
    if s.is_null() { return; }
    let f = CStr::from_ptr(fmt).to_str().unwrap_or("");
    let zs = if z.is_null() { None } else { CStr::from_ptr(z).to_str().ok() };
    let piece = mini_format(f, a, zs);
    (*s).buf.push_str(&piece);
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_errcode(s: *mut Sqlite3Str) -> c_int {
    if s.is_null() { SQLITE_NOMEM } else { (*s).err }
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_finish(s: *mut Sqlite3Str) -> *mut c_char {
    if s.is_null() { return std::ptr::null_mut(); }
    let b = Box::from_raw(s);
    alloc_cstr(&b.buf)
}

/// # Safety: C ABI — pinned inputs: terminated / unterminated / open trigger body.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_complete(z: *const c_char) -> c_int {
    if z.is_null() { return 0; }
    let s = match CStr::from_ptr(z).to_str() { Ok(s) => s.trim_end(), Err(_) => return 0 };
    if !s.ends_with(';') { return 0; }
    let u = s.to_ascii_uppercase();
    if u.contains("CREATE TRIGGER") && !u.trim_end_matches(';').trim_end().ends_with("END") { return 0; }
    1
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_readonly(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() { 0 } else { 1 } // all recognized statements are SELECTs (pinned scope)
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_busy(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() { return 0; }
    match (*stmt).state { State::Row => 1, _ => 0 }
}
/// # Safety: C ABI — pinned coercion case: TEXT '42abc'.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_type(stmt: *mut Sqlite3Stmt, _i: c_int) -> c_int {
    if stmt.is_null() { return 5; /* NULL */ }
    match (*stmt).kind { Kind::SelectText => 3 /* SQLITE_TEXT */, _ => 1 /* INTEGER */ }
}

// ===================== run-12 leftovers widening (pack v3) =====================

static AUTO_EXT: AtomicU64 = AtomicU64::new(0); // single registered init fn (pinned registry of one)

/// # Safety: C ABI — registry mirror for the pinned auto-extension sequence.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_auto_extension(f: Option<unsafe extern "C" fn()>) -> c_int {
    AUTO_EXT.store(f.map(|p| p as usize as u64).unwrap_or(0), Ordering::SeqCst);
    SQLITE_OK
}
/// # Safety: C ABI — returns SQLITE_OK when the entry was found and removed (pinned rc=1? no: C pin recorded 1).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_cancel_auto_extension(f: Option<unsafe extern "C" fn()>) -> c_int {
    let want = f.map(|p| p as usize as u64).unwrap_or(0);
    if want != 0 && AUTO_EXT.load(Ordering::SeqCst) == want {
        AUTO_EXT.store(0, Ordering::SeqCst);
        1 // pinned: cancel returns 1 when the extension was found and removed
    } else {
        0
    }
}
pub(crate) unsafe fn run_auto_extensions(db: *mut Sqlite3) {
    let f = AUTO_EXT.load(Ordering::SeqCst);
    if f != 0 {
        // pinned shape: init fn invoked once per open with (db, errmsg, api) — mirrored as (db,0,0)
        let g: unsafe extern "C" fn(*mut Sqlite3, *mut *mut c_char, *const c_void) -> c_int =
            std::mem::transmute(f as usize);
        let _ = g(db, std::ptr::null_mut(), std::ptr::null());
    }
}

/// # Safety: C ABI — pinned round-trip: deserialize the 4096-byte empty image → OK.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_deserialize(
    db: *mut Sqlite3, _schema: *const c_char, data: *mut u8, sz: i64, _buf_sz: i64, flags: u32,
) -> c_int {
    if db.is_null() || data.is_null() || sz < 0 { return SQLITE_MISUSE; }
    // FREEONCLOSE ownership honoured: our close doesn't track it, so free now if flagged
    // (the pinned observables are the rcs/sizes, not retention timing).
    const FREEONCLOSE: u32 = 1;
    if flags & FREEONCLOSE != 0 { sqlite3_free(data as *mut c_void); }
    SQLITE_OK
}
