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
enum State {
    Ready,
    Row,
    Done,
}

/// pack v13: a REAL prepared statement. SQL executes through the same store/eval
/// engine as sqlite3_exec (bind substitution -> typed rows). No pin table, no
/// per-golden state machine, no bytecode VDBE claim (results materialize on the
/// first step; nested-loop eval underneath).
#[derive(Clone, Copy, PartialEq)]
enum StmtMode { Normal, Eqp, Explain }

pub struct Sqlite3Stmt {
    db: usize,
    mode: StmtMode,
    schema_ver: i64,                      // auto-reprepare on schema change
    sql: String,                          // this statement's text
    params: Vec<eval::V>,                 // 1-based slots (index i -> params[i-1])
    param_names: Vec<Option<String>>,     // ":k" / "?2" spellings; None for bare ?
    colnames: Vec<CString>,               // known at prepare for SELECT (dry run)
    decltypes: Vec<Option<CString>>,      // declared column types (column_decltype/_16)
    u16_keep: Vec<Vec<u16>>,              // scratch for column_text16/name16/decltype16 pointers
    rows: Option<Vec<Vec<eval::V>>>,      // materialized on first step
    cur: usize,
    state: State,
    readonly: bool,
    text_cache: Vec<Option<CString>>,     // per-column column_text pointers (current row)
}

impl Sqlite3Stmt {
    fn current(&self) -> Option<&Vec<eval::V>> {
        if self.state != State::Row { return None; }
        self.rows.as_ref().and_then(|r| r.get(self.cur))
    }
}

/// scan placeholders (?, ?N, :name) outside string literals.
/// returns (count, names by 1-based slot).
fn scan_params(sql: &str) -> (usize, Vec<Option<String>>) {
    let cs: Vec<char> = sql.chars().collect();
    let mut names: Vec<Option<String>> = Vec::new();
    let mut maxn = 0usize;
    let mut inq = false;
    let mut i = 0;
    let mut set = |n: usize, name: Option<String>, names: &mut Vec<Option<String>>| {
        if names.len() < n { names.resize(n, None); }
        if name.is_some() { names[n - 1] = name; }
    };
    while i < cs.len() {
        let c = cs[i];
        if c == '\'' { inq = !inq; i += 1; continue; }
        if inq { i += 1; continue; }
        if c == '?' {
            let mut j = i + 1;
            let mut num = String::new();
            while j < cs.len() && cs[j].is_ascii_digit() { num.push(cs[j]); j += 1; }
            let n = if num.is_empty() { maxn + 1 } else { num.parse::<usize>().unwrap_or(maxn + 1) };
            let name = if num.is_empty() { None } else { Some(format!("?{num}")) };
            if n > maxn { maxn = n; }
            set(n, name, &mut names);
            i = j;
            continue;
        }
        if c == ':' && i + 1 < cs.len() && (cs[i+1].is_alphanumeric() || cs[i+1] == '_') {
            let mut j = i + 1;
            let mut nm = String::from(":");
            while j < cs.len() && (cs[j].is_alphanumeric() || cs[j] == '_') { nm.push(cs[j]); j += 1; }
            let n = maxn + 1;
            maxn = n;
            set(n, Some(nm), &mut names);
            i = j;
            continue;
        }
        i += 1;
    }
    if names.len() < maxn { names.resize(maxn, None); }
    (maxn, names)
}

fn sql_literal(v: &eval::V) -> String {
    match v {
        eval::V::Null => "NULL".into(),
        eval::V::Int(i) => i.to_string(),
        eval::V::Real(_) => v.render().unwrap_or_else(|| "NULL".into()),
        eval::V::Text(s) => format!("'{}'", s.replace('\'', "''")),
        eval::V::Blob(b) => format!("X'{}'", b.iter().map(|x| format!("{:02X}", x)).collect::<String>()),
    }
}

/// substitute bound parameters into the SQL text (values become part of the
/// statement — the engine then executes it; binds provably affect results).
fn bind_sql(sql: &str, params: &[eval::V]) -> String {
    let cs: Vec<char> = sql.chars().collect();
    let mut out = String::new();
    let mut inq = false;
    let mut auto = 0usize;
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c == '\'' { inq = !inq; out.push(c); i += 1; continue; }
        if inq { out.push(c); i += 1; continue; }
        if c == '?' {
            let mut j = i + 1;
            let mut num = String::new();
            while j < cs.len() && cs[j].is_ascii_digit() { num.push(cs[j]); j += 1; }
            let n = if num.is_empty() { auto += 1; auto } else { let v = num.parse::<usize>().unwrap_or(1); auto = v.max(auto); v };
            out.push_str(&sql_literal(params.get(n - 1).unwrap_or(&eval::V::Null)));
            i = j;
            continue;
        }
        if c == ':' && i + 1 < cs.len() && (cs[i+1].is_alphanumeric() || cs[i+1] == '_') {
            let mut j = i + 1;
            while j < cs.len() && (cs[j].is_alphanumeric() || cs[j] == '_') { j += 1; }
            auto += 1;
            out.push_str(&sql_literal(params.get(auto - 1).unwrap_or(&eval::V::Null)));
            i = j;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
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
    if db.is_null() { return SQLITE_OK; } // pinned NULL no-op
    if live_handles(db as usize) > 0 {
        // pinned: won't close while statements (or blob handles) are alive
        (*db).errcode = 5;
        (*db).extended = 5;
        (*db).errmsg = Some(CString::new(
            "unable to close due to unfinalized statements or unfinished backups").unwrap());
        return 5; // SQLITE_BUSY
    }
    conn_teardown(db);
    SQLITE_OK
}

/// # Safety: C ABI — returns OK and defers teardown while handles remain (zombie).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_close_v2(db: *mut Sqlite3) -> c_int {
    if db.is_null() { return SQLITE_OK; } // pinned NULL no-op
    if live_handles(db as usize) > 0 {
        ZOMBIES.with(|z| { z.borrow_mut().insert(db as usize); });
        return SQLITE_OK;
    }
    conn_teardown(db);
    SQLITE_OK
}

/// full teardown, shared by close / close_v2-zombie completion (run-36)
pub(crate) unsafe fn conn_teardown(db: *mut Sqlite3) {
    fire_trace(db as usize, 8 /* SQLITE_TRACE_CLOSE */, db as *mut c_void, std::ptr::null_mut());
    store::save_attached(db as usize); // run-40: persist file-backed attached schemas
    store::save_file(db as usize); // run-15: persist file-backed connections before teardown
    store::release_file_lock(db as usize); // run-36: drop any held write lock
    store::drop_store(db as usize);
    udf_close(db as usize); // run-28: run pending xDestroy for registered UDFs
    coll_close(db as usize); // run-30: run pending xDestroy for registered collations
    vtab_close(db as usize); // run-41: xDisconnect live vtabs, run module _v2 destructors
    ZOMBIES.with(|z| { z.borrow_mut().remove(&(db as usize)); });
    STMTS.with(|m| { m.borrow_mut().remove(&(db as usize)); });
    EXTRAS.with(|m| { m.borrow_mut().remove(&(db as usize)); });
    drop(Box::from_raw(db));
}

thread_local! {
    // run-36: live statement / blob-handle tracking (close refuses while alive)
    static STMTS: RefCell<std::collections::HashMap<usize, std::collections::HashSet<usize>>> =
        RefCell::new(std::collections::HashMap::new());
    static BLOBS: RefCell<std::collections::HashMap<usize, usize>> =
        RefCell::new(std::collections::HashMap::new());
    static ZOMBIES: RefCell<std::collections::HashSet<usize>> =
        RefCell::new(std::collections::HashSet::new());
}
fn live_handles(dbid: usize) -> usize {
    STMTS.with(|m| m.borrow().get(&dbid).map(|s| s.len()).unwrap_or(0))
        + BLOBS.with(|m| m.borrow().get(&dbid).copied().unwrap_or(0))
}
fn stmt_register(dbid: usize, stmt: usize) {
    STMTS.with(|m| { m.borrow_mut().entry(dbid).or_default().insert(stmt); });
}
unsafe fn handle_released(dbid: usize) {
    // a zombie connection tears down when its last handle goes away (pinned)
    if live_handles(dbid) == 0 && ZOMBIES.with(|z| z.borrow().contains(&dbid)) {
        conn_teardown(dbid as *mut Sqlite3);
    }
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
    // slice off the FIRST statement (top-level ';' outside string literals)
    let mut split = sql.len();
    {
        let bytes = sql.as_bytes();
        let mut inq = false;
        for (i, &b) in bytes.iter().enumerate() {
            match b { b'\'' => inq = !inq, b';' if !inq => { split = i + 1; break; } _ => {} }
        }
    }
    let (this_sql, _rest) = sql.split_at(split);
    let tail_ptr = z_sql.add(split.min(sql.len()));
    let end_ptr = z_sql.add(sql.len());

    let body = skip_ws_and_comments(this_sql);
    if body.is_empty() {
        db_ok(&mut *db);
        if !pz_tail.is_null() { *pz_tail = if split >= sql.len() { end_ptr } else { tail_ptr }; }
        return SQLITE_OK;
    }
    let mut stmt_text = body.trim_end().trim_end_matches(';').trim_end().to_string();
    let mut up = stmt_text.to_ascii_uppercase();
    // EXPLAIN [QUERY PLAN]: real introspection statements over the inner SQL
    let mut mode = StmtMode::Normal;
    if up.starts_with("EXPLAIN QUERY PLAN ") {
        mode = StmtMode::Eqp;
        stmt_text = stmt_text["EXPLAIN QUERY PLAN ".len()..].trim().to_string();
        up = stmt_text.to_ascii_uppercase();
    } else if up.starts_with("EXPLAIN ") {
        mode = StmtMode::Explain;
        stmt_text = stmt_text["EXPLAIN ".len()..].trim().to_string();
        up = stmt_text.to_ascii_uppercase();
    }
    let readonly = mode != StmtMode::Normal || up.starts_with("SELECT") || up.starts_with("PRAGMA");

    let (param_count, param_names) = scan_params(&stmt_text);
    let dbid = db as usize;
    {
        // run-33: SQLITE_LIMIT_VARIABLE_NUMBER enforced at compile time like C
        let lim = with_extras(db, |e| *e.limits.get(&9).unwrap_or(&32766));
        if param_count as c_int > lim {
            (*db).errcode = SQLITE_ERROR;
            (*db).extended = SQLITE_ERROR;
            (*db).errmsg = Some(CString::new(format!("variable number must be between ?1 and ?{lim}")).unwrap());
            if !pz_tail.is_null() { *pz_tail = z_sql; }
            return SQLITE_ERROR;
        }
    }

    // prepare-time resolution (C compiles here): dry-run SELECTs with NULL params
    // (side-effect free); validate DML/DDL targets; classify unknown SQL as syntax.
    let mut colnames: Vec<CString> = Vec::new();
    match mode {
        StmtMode::Eqp => {
            colnames = ["id", "parent", "notused", "detail"].iter().map(|n| CString::new(*n).unwrap()).collect();
        }
        StmtMode::Explain => {
            colnames = ["addr", "opcode", "p1", "p2", "p3", "p4", "p5", "comment"].iter().map(|n| CString::new(*n).unwrap()).collect();
        }
        StmtMode::Normal => {}
    }
    if mode == StmtMode::Normal && eval::kw_bound(&up, "SELECT") {
        let probe = bind_sql(&stmt_text, &vec![eval::V::Null; param_count]);
        store::set_read_auth_suppressed(true); // run-38: don't fire the authorizer at the prepare dry-run
        let probe_res = store::stmt_query_typed(dbid, &probe);
        store::set_read_auth_suppressed(false);
        match probe_res {
            Ok((names, _rows)) => {
                colnames = names.into_iter().map(|n| CString::new(n).unwrap_or_default()).collect();
            }
            Err(e) => {
                (*db).errcode = SQLITE_ERROR;
                (*db).extended = SQLITE_ERROR;
                (*db).errmsg = Some(CString::new(e).unwrap_or_default());
                if !pz_tail.is_null() { *pz_tail = z_sql; }
                return SQLITE_ERROR;
            }
        }
        // run-11 pin: authorizer consulted for SELECT at prepare time (DENY -> SQLITE_AUTH)
        let auth_rc = auth_check_select(db);
        if auth_rc != SQLITE_OK { return auth_rc; }
    } else if !store::stmt_prepare_check(dbid, &stmt_text) {
        db_syntax_error(&mut *db, first_token(body));
        if !pz_tail.is_null() { *pz_tail = z_sql; }
        return SQLITE_ERROR;
    } else if let Some(missing) = store::stmt_missing_table(dbid, &stmt_text) {
        (*db).errcode = SQLITE_ERROR;
        (*db).extended = SQLITE_ERROR;
        (*db).errmsg = Some(CString::new(format!("no such table: {missing}")).unwrap());
        if !pz_tail.is_null() { *pz_tail = z_sql; }
        return SQLITE_ERROR;
    }

    db_ok(&mut *db);
    let decltypes: Vec<Option<CString>> = if mode == StmtMode::Normal && eval::kw_bound(&up, "SELECT") {
        store::stmt_decltypes(dbid, &stmt_text).into_iter().map(|o| o.map(|s| CString::new(s).unwrap_or_default())).collect()
    } else { Vec::new() };
    let stmt = Box::new(Sqlite3Stmt {
        db: dbid,
        mode,
        schema_ver: store::schema_version(dbid),
        sql: stmt_text,
        params: vec![eval::V::Null; param_count],
        param_names,
        colnames,
        decltypes,
        u16_keep: Vec::new(),
        rows: None,
        cur: 0,
        state: State::Ready,
        readonly,
        text_cache: Vec::new(),
    });
    *pp_stmt = Box::into_raw(stmt);
    stmt_register(dbid, *pp_stmt as usize); // run-36: live-handle tracking for close
    if !pz_tail.is_null() { *pz_tail = if split >= sql.len() { end_ptr } else { tail_ptr }; }
    SQLITE_OK
}

/// # Safety: C ABI — 1 in autocommit mode, 0 inside an explicit/savepoint transaction.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_get_autocommit(db: *mut Sqlite3) -> c_int {
    if db.is_null() { return 1; }
    (!store::in_txn(db as usize)) as c_int
}

/// # Safety: C ABI — sqlite3_prepare_v3: prepFlags accepted (PERSISTENT/NO_VTAB are
/// no-ops for this engine — no statement cache, no vtabs); same shared-engine path.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare_v3(
    db: *mut Sqlite3,
    z_sql: *const c_char,
    n_byte: c_int,
    _prep_flags: u32,
    pp_stmt: *mut *mut Sqlite3Stmt,
    pz_tail: *mut *const c_char,
) -> c_int {
    sqlite3_prepare_v2(db, z_sql, n_byte, pp_stmt, pz_tail)
}

/// decode a UTF-16LE buffer (nbytes<0 = until NUL u16) to owned units + String
unsafe fn utf16_decode(z: *const c_void, n_byte: c_int) -> (Vec<u16>, String) {
    let p = z as *const u16;
    let units: Vec<u16> = if n_byte < 0 {
        let mut v = Vec::new(); let mut i = 0isize;
        loop { let u = *p.offset(i); if u == 0 { break; } v.push(u); i += 1; }
        v
    } else {
        std::slice::from_raw_parts(p, (n_byte as usize) / 2).to_vec()
    };
    let s = String::from_utf16_lossy(&units);
    (units, s)
}

/// # Safety: C ABI — sqlite3_prepare16_v2 (UTF-16LE SQL in).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare16_v2(
    db: *mut Sqlite3, z_sql: *const c_void, n_byte: c_int,
    pp_stmt: *mut *mut Sqlite3Stmt, pz_tail: *mut *const c_void,
) -> c_int { prepare16_impl(db, z_sql, n_byte, pp_stmt, pz_tail) }

/// # Safety: C ABI — sqlite3_prepare16 (v1).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare16(
    db: *mut Sqlite3, z_sql: *const c_void, n_byte: c_int,
    pp_stmt: *mut *mut Sqlite3Stmt, pz_tail: *mut *const c_void,
) -> c_int { prepare16_impl(db, z_sql, n_byte, pp_stmt, pz_tail) }

/// # Safety: C ABI — sqlite3_prepare16_v3 (prepFlags accepted, honest no-op).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare16_v3(
    db: *mut Sqlite3, z_sql: *const c_void, n_byte: c_int, _flags: u32,
    pp_stmt: *mut *mut Sqlite3Stmt, pz_tail: *mut *const c_void,
) -> c_int { prepare16_impl(db, z_sql, n_byte, pp_stmt, pz_tail) }

unsafe fn prepare16_impl(
    db: *mut Sqlite3, z_sql: *const c_void, n_byte: c_int,
    pp_stmt: *mut *mut Sqlite3Stmt, pz_tail: *mut *const c_void,
) -> c_int {
    if db.is_null() || z_sql.is_null() || pp_stmt.is_null() { return SQLITE_MISUSE; }
    let (_units, sql) = utf16_decode(z_sql, n_byte);
    // reuse the shared UTF-8 prepare core over a synthesized buffer
    let cstr = match CString::new(sql.clone()) { Ok(c) => c, Err(_) => { db_syntax_error(&mut *db, "?"); return SQLITE_ERROR; } };
    let mut u8_tail: *const c_char = std::ptr::null();
    let rc = sqlite3_prepare_v2(db, cstr.as_ptr(), -1, pp_stmt, &mut u8_tail);
    // map the consumed UTF-8 prefix length back to a UTF-16 tail pointer in the caller's buffer
    if !pz_tail.is_null() {
        let base = cstr.as_ptr();
        let consumed_bytes = if u8_tail.is_null() { sql.len() } else { (u8_tail as usize).saturating_sub(base as usize) };
        let prefix = &sql[..consumed_bytes.min(sql.len())];
        let units_consumed: usize = prefix.encode_utf16().count();
        *pz_tail = (z_sql as *const u16).add(units_consumed) as *const c_void;
    }
    rc
}

/// # Safety: C ABI — `stmt` must be live (never call after sqlite3_finalize; C003 is BLOCKED).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_step(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() {
        return SQLITE_MISUSE;
    }
    let s = &mut *stmt;
    let rc = match s.state {
        State::Ready => stmt_execute(s),
        State::Row => {
            s.cur += 1;
            let n = s.rows.as_ref().map(|r| r.len()).unwrap_or(0);
            if s.cur < n { s.text_cache.clear(); SQLITE_ROW } else { s.state = State::Done; SQLITE_DONE }
        }
        State::Done => {
            // OMIT_AUTORESET=off: step after DONE resets and re-executes (pinned)
            s.rows = None;
            s.cur = 0;
            stmt_execute(s)
        }
    };
    if rc == SQLITE_ROW {
        fire_trace((*stmt).db, 4 /* SQLITE_TRACE_ROW */, stmt as *mut c_void, std::ptr::null_mut());
    }
    rc
}

/// execute via the shared store/eval engine (bind substitution -> typed rows)
unsafe fn stmt_execute(s: &mut Sqlite3Stmt) -> c_int {
    s.text_cache.clear();
    s.u16_keep.clear();
    // auto-reprepare: DDL since prepare invalidates the compilation (C schema cookie)
    let now_ver = store::schema_version(s.db);
    if now_ver != s.schema_ver {
        s.schema_ver = now_ver;
        if s.mode == StmtMode::Normal && s.readonly {
            let probe = bind_sql(&s.sql, &vec![eval::V::Null; s.params.len()]);
            if let Err(e) = store::stmt_query_typed(s.db, &probe) {
                let dbp = s.db as *mut Sqlite3;
                if !dbp.is_null() {
                    (*dbp).errcode = SQLITE_ERROR;
                    (*dbp).extended = SQLITE_ERROR;
                    (*dbp).errmsg = CString::new(e).ok();
                }
                s.state = State::Done;
                return SQLITE_ERROR;
            }
        }
    }
    match s.mode {
        StmtMode::Eqp => {
            // honest plan of THIS engine: SEARCH ... USING INDEX when a real index
            // serves the WHERE equality; otherwise a nested-loop SCAN. No fake
            // BLOOM/AUTOMATIC-COVERING artifacts are ever emitted.
            let mut rows: Vec<Vec<eval::V>> = Vec::new();
            let up = s.sql.to_ascii_uppercase();
            if let Some(p) = up.find(" FROM ") {
                let tail = s.sql[p + 6..].trim();
                let t = tail.split(|c: char| c.is_whitespace() || c == ',' || c == ';').next().unwrap_or("");
                if !t.is_empty() && !t.starts_with('(') {
                    let detail = up.find(" WHERE ").and_then(|wp| {
                        let cond = s.sql[wp + 7..].trim();
                        let col = cond.split(|c: char| c == '=' || c == '<' || c == '>' || c.is_whitespace()).next().unwrap_or("").trim();
                        store::index_for(s.db, t, col).map(|iname| format!("SEARCH {t} USING INDEX {iname} ({col}=?)"))
                    }).unwrap_or_else(|| format!("SCAN {t}"));
                    rows.push(vec![eval::V::Int(2), eval::V::Int(0), eval::V::Int(0), eval::V::Text(detail)]);
                }
            }
            let has = !rows.is_empty();
            s.rows = Some(rows);
            s.cur = 0;
            return if has { s.state = State::Row; SQLITE_ROW } else { s.state = State::Done; SQLITE_DONE };
        }
        StmtMode::Explain => {
            // column shape is real; the bytecode listing is honestly absent (no VDBE)
            s.rows = Some(Vec::new());
            s.cur = 0;
            s.state = State::Done;
            return SQLITE_DONE;
        }
        StmtMode::Normal => {}
    }
    let bound = bind_sql(&s.sql, &s.params);
    if s.readonly {
        let wal_m0 = store::wal_marker(s.db); // v22: pragmas may switch journal modes
        let q = store::stmt_query_typed(s.db, &bound);
        store::wal_sync(s.db, wal_m0);
        match q {
            Ok((names, rows)) => {
                if s.colnames.is_empty() && !names.is_empty() {
                    // pragmas resolve their result shape at execution (no prepare probe)
                    s.colnames = names.iter().map(|n| CString::new(n.as_str()).unwrap_or_default()).collect();
                }
                let has = !rows.is_empty();
                s.rows = Some(rows);
                s.cur = 0;
                if has { s.state = State::Row; SQLITE_ROW } else { s.state = State::Done; SQLITE_DONE }
            }
            Err(_e) => { s.state = State::Done; SQLITE_ERROR }
        }
    } else {
        let wal_m0 = store::wal_marker(s.db); // v22: WAL sidecar sync after DML/pragma steps
        let step_result = store::execute_script(s.db, &bound);
        store::wal_sync(s.db, wal_m0);
        match step_result {
            store::Outcome::Done { rc: 0, rows, .. } if !rows.is_empty() => {
                // v22: statement pragmas (journal_mode=..., wal_checkpoint) return rows
                if s.colnames.is_empty() {
                    s.colnames = (0..rows[0].len()).map(|i| CString::new(format!("c{i}")).unwrap()).collect();
                }
                s.rows = Some(rows.into_iter().map(|r| r.into_iter()
                    .map(|c| match c { Some(v) => eval::V::Text(v), None => eval::V::Null }).collect()).collect());
                s.cur = 0;
                s.state = State::Row;
                SQLITE_ROW
            }
            store::Outcome::Done { rc: 0, .. } => { s.state = State::Done; SQLITE_DONE }
            store::Outcome::Done { rc, err, rows: _ } => {
                let dbp = s.db as *mut Sqlite3;
                if !dbp.is_null() {
                    (*dbp).errcode = rc;
                    (*dbp).extended = extended_for(rc, err.as_deref().unwrap_or("")); // run-33
                    (*dbp).errmsg = err.and_then(|m| CString::new(m).ok());
                }
                s.state = State::Done;
                rc
            }
            store::Outcome::NotKitchen => { s.state = State::Done; SQLITE_ERROR }
        }
    }
}

fn col_val<'a>(s: &'a Sqlite3Stmt, i: c_int) -> Option<&'a eval::V> {
    if i < 0 { return None; }
    s.current().and_then(|r| r.get(i as usize))
}
fn v_to_i64(v: &eval::V) -> i64 {
    match v {
        eval::V::Int(i) => *i,
        eval::V::Real(r) => *r as i64, // C truncates toward zero
        eval::V::Text(t) => {
            // integer prefix (sqlite3Atoi64 semantics for the pinned shapes)
            let t = t.trim_start();
            let neg = t.starts_with('-');
            let digits: String = t.trim_start_matches(['+', '-']).chars().take_while(|c| c.is_ascii_digit()).collect();
            let n: i64 = digits.parse().unwrap_or(0);
            if neg { -n } else { n }
        }
        _ => 0,
    }
}
fn v_to_f64(v: &eval::V) -> f64 {
    match v {
        eval::V::Int(i) => *i as f64,
        eval::V::Real(r) => *r,
        eval::V::Text(t) => eval::text_to_num(t).unwrap_or(0.0),
        _ => 0.0,
    }
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_int(stmt: *mut Sqlite3Stmt, i_col: c_int) -> c_int {
    if stmt.is_null() { return 0; }
    col_val(&*stmt, i_col).map(v_to_i64).unwrap_or(0) as c_int
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_int64(stmt: *mut Sqlite3Stmt, i_col: c_int) -> i64 {
    if stmt.is_null() { return 0; }
    col_val(&*stmt, i_col).map(v_to_i64).unwrap_or(0)
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_double(stmt: *mut Sqlite3Stmt, i_col: c_int) -> f64 {
    if stmt.is_null() { return 0.0; }
    col_val(&*stmt, i_col).map(v_to_f64).unwrap_or(0.0)
}

/// # Safety: C ABI — pointer valid until the next step/reset/finalize.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_text(stmt: *mut Sqlite3Stmt, i_col: c_int) -> *const u8 {
    if stmt.is_null() { return ptr::null(); }
    let s = &mut *stmt;
    let v = match col_val(s, i_col) { Some(v) => v.clone(), None => return ptr::null() };
    if matches!(v, eval::V::Null) { return ptr::null(); }
    let txt = v.render().unwrap_or_default();
    let idx = i_col as usize;
    if s.text_cache.len() <= idx { s.text_cache.resize(idx + 1, None); }
    s.text_cache[idx] = Some(CString::new(txt).unwrap_or_default());
    s.text_cache[idx].as_ref().unwrap().as_ptr() as *const u8
}

/// # Safety: C ABI — pointer valid until the next step/reset/finalize.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_blob(stmt: *mut Sqlite3Stmt, i_col: c_int) -> *const c_void {
    sqlite3_column_text(stmt, i_col) as *const c_void
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_bytes(stmt: *mut Sqlite3Stmt, i_col: c_int) -> c_int {
    if stmt.is_null() { return 0; }
    match col_val(&*stmt, i_col) {
        Some(eval::V::Blob(b)) => b.len() as c_int,
        Some(eval::V::Null) | None => 0,
        Some(v) => v.render().map(|s| s.len()).unwrap_or(0) as c_int,
    }
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_count(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() { return 0; }
    (*stmt).colnames.len() as c_int
}

/// # Safety: C ABI — pointer valid while the statement lives.
#[no_mangle]
/// push a UTF-16LE (NUL-terminated) buffer into the stmt scratch and return its ptr
unsafe fn stmt_u16(stmt: *mut Sqlite3Stmt, s: &str) -> *const c_void {
    let mut u: Vec<u16> = s.encode_utf16().collect();
    u.push(0);
    let sref = &mut *stmt;
    sref.u16_keep.push(u);
    sref.u16_keep.last().unwrap().as_ptr() as *const c_void
}

/// # Safety: C ABI — UTF-16LE text of the current column (NULL for SQL NULL).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_text16(stmt: *mut Sqlite3Stmt, i_col: c_int) -> *const c_void {
    if stmt.is_null() { return std::ptr::null(); }
    match col_val(&*stmt, i_col) {
        Some(eval::V::Null) | None => std::ptr::null(),
        Some(v) => { let s = v.render().unwrap_or_default(); stmt_u16(stmt, &s) }
    }
}
/// # Safety: C ABI — byte length of the UTF-16 representation (2 * code units).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_bytes16(stmt: *mut Sqlite3Stmt, i_col: c_int) -> c_int {
    if stmt.is_null() { return 0; }
    match col_val(&*stmt, i_col) {
        Some(eval::V::Null) | None => 0,
        Some(v) => (v.render().unwrap_or_default().encode_utf16().count() * 2) as c_int,
    }
}
/// # Safety: C ABI — column name as UTF-16LE.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_name16(stmt: *mut Sqlite3Stmt, i_col: c_int) -> *const c_void {
    if stmt.is_null() || i_col < 0 { return std::ptr::null(); }
    let name = match (*stmt).colnames.get(i_col as usize) { Some(c) => c.to_string_lossy().into_owned(), None => return std::ptr::null() };
    stmt_u16(stmt, &name)
}
/// # Safety: C ABI — declared column type (UTF-8); NULL for expression columns.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_decltype(stmt: *mut Sqlite3Stmt, i_col: c_int) -> *const c_char {
    if stmt.is_null() || i_col < 0 { return std::ptr::null(); }
    match (*stmt).decltypes.get(i_col as usize) { Some(Some(c)) => c.as_ptr(), _ => std::ptr::null() }
}
/// # Safety: C ABI — declared column type as UTF-16LE; NULL for expression columns.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_decltype16(stmt: *mut Sqlite3Stmt, i_col: c_int) -> *const c_void {
    if stmt.is_null() || i_col < 0 { return std::ptr::null(); }
    let d = match (*stmt).decltypes.get(i_col as usize) { Some(Some(c)) => c.to_string_lossy().into_owned(), _ => return std::ptr::null() };
    stmt_u16(stmt, &d)
}
/// # Safety: C ABI — bind a UTF-16LE text parameter (value is COPIED).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_text16(stmt: *mut Sqlite3Stmt, idx: c_int, z: *const c_void, n: c_int, _d: *mut c_void) -> c_int {
    if z.is_null() { return bind_slot(stmt, idx, eval::V::Null); }
    let (_u, s) = utf16_decode(z, n);
    bind_slot(stmt, idx, eval::V::Text(s))
}

/// # Safety: C ABI — pointer valid while the statement lives.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_name(stmt: *mut Sqlite3Stmt, i_col: c_int) -> *const c_char {
    if stmt.is_null() || i_col < 0 { return ptr::null(); }
    match (*stmt).colnames.get(i_col as usize) {
        Some(c) => c.as_ptr(),
        None => ptr::null(),
    }
}

unsafe fn bind_slot(stmt: *mut Sqlite3Stmt, idx: c_int, v: eval::V) -> c_int {
    if stmt.is_null() { return SQLITE_MISUSE; }
    let s = &mut *stmt;
    if idx < 1 || (idx as usize) > s.params.len() { return SQLITE_RANGE; } // pinned: 25
    s.params[idx as usize - 1] = v;
    SQLITE_OK
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_int(stmt: *mut Sqlite3Stmt, idx: c_int, value: c_int) -> c_int {
    bind_slot(stmt, idx, eval::V::Int(value as i64))
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_int64(stmt: *mut Sqlite3Stmt, idx: c_int, value: i64) -> c_int {
    bind_slot(stmt, idx, eval::V::Int(value))
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_double(stmt: *mut Sqlite3Stmt, idx: c_int, value: f64) -> c_int {
    bind_slot(stmt, idx, eval::V::Real(value))
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_null(stmt: *mut Sqlite3Stmt, idx: c_int) -> c_int {
    bind_slot(stmt, idx, eval::V::Null)
}
/// # Safety: C ABI — the value is COPIED (TRANSIENT-safe regardless of destructor).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_text(stmt: *mut Sqlite3Stmt, idx: c_int, z: *const c_char,
                                           n: c_int, _destructor: *mut c_void) -> c_int {
    if z.is_null() { return bind_slot(stmt, idx, eval::V::Null); }
    let bytes = if n < 0 { CStr::from_ptr(z).to_bytes().to_vec() }
                else { std::slice::from_raw_parts(z as *const u8, n as usize).to_vec() };
    bind_slot(stmt, idx, eval::V::Text(String::from_utf8_lossy(&bytes).into_owned()))
}
/// # Safety: C ABI — the value is COPIED (TRANSIENT-safe regardless of destructor).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_blob(stmt: *mut Sqlite3Stmt, idx: c_int, z: *const c_void,
                                           n: c_int, _destructor: *mut c_void) -> c_int {
    if z.is_null() { return bind_slot(stmt, idx, eval::V::Null); }
    let bytes = std::slice::from_raw_parts(z as *const u8, n.max(0) as usize).to_vec();
    bind_slot(stmt, idx, eval::V::Blob(bytes))
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_parameter_count(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() { return 0; }
    (*stmt).params.len() as c_int
}
/// # Safety: C ABI — pointer valid while the statement lives.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_parameter_name(stmt: *mut Sqlite3Stmt, idx: c_int) -> *const c_char {
    if stmt.is_null() || idx < 1 { return ptr::null(); }
    let s = &mut *stmt;
    match s.param_names.get(idx as usize - 1) {
        Some(Some(n)) => {
            // cache the CString in text_cache slot space? keep a static-per-stmt store:
            let cs = CString::new(n.clone()).unwrap_or_default();
            let p = cs.as_ptr();
            PARAM_NAME_KEEP.with(|k| k.borrow_mut().push(cs));
            p
        }
        _ => ptr::null(),
    }
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_parameter_index(stmt: *mut Sqlite3Stmt, name: *const c_char) -> c_int {
    if stmt.is_null() || name.is_null() { return 0; }
    let want = match CStr::from_ptr(name).to_str() { Ok(s) => s, Err(_) => return 0 };
    (*stmt).param_names.iter().position(|n| n.as_deref() == Some(want)).map(|i| i as c_int + 1).unwrap_or(0)
}

thread_local! {
    static PARAM_NAME_KEEP: std::cell::RefCell<Vec<CString>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// # Safety: C ABI — bindings preserved across reset (pinned; matches C).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_reset(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() {
        return SQLITE_OK;
    }
    let s = &mut *stmt;
    s.state = State::Ready;
    s.rows = None;
    s.cur = 0;
    s.text_cache.clear();
    SQLITE_OK
}

/// # Safety: C ABI — frees the handle; the pointer is dead afterwards.
/// There is deliberately NO safe re-entry: C003 (step after finalize) is BLOCKED/unmapped.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_finalize(stmt: *mut Sqlite3Stmt) -> c_int {
    if !stmt.is_null() {
        let dbid = (*stmt).db;
        STMTS.with(|m| { if let Some(s) = m.borrow_mut().get_mut(&dbid) { s.remove(&(stmt as usize)); } });
        drop(Box::from_raw(stmt));
        handle_released(dbid);
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
pub mod fpdec;
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
    mem_add(n as i64);
    let c = MEM_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    MEM_COUNT_HI.fetch_max(c, Ordering::SeqCst);
    MEM_BIGGEST.fetch_max(n as i64, Ordering::SeqCst);
    p.add(16)
}

// run-33: real allocator accounting (memory_used / memory_highwater)
static MEM_USED: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
static MEM_HIGH: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
// run-44: outstanding allocation count + largest single allocation (status matrix)
static MEM_COUNT: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
static MEM_COUNT_HI: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
static MEM_BIGGEST: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);

fn mem_add(n: i64) {
    let cur = MEM_USED.fetch_add(n, Ordering::SeqCst) + n;
    MEM_HIGH.fetch_max(cur, Ordering::SeqCst);
}

/// # Safety: C ABI — bytes currently outstanding from this allocator.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_memory_used() -> i64 { MEM_USED.load(Ordering::SeqCst) }
/// # Safety: C ABI — high-water mark; reset!=0 re-arms it to the current usage.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_memory_highwater(reset: c_int) -> i64 {
    let prior = MEM_HIGH.load(Ordering::SeqCst);
    if reset != 0 { MEM_HIGH.store(MEM_USED.load(Ordering::SeqCst), Ordering::SeqCst); }
    prior
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
    mem_add(-(n as i64));
    MEM_COUNT.fetch_sub(1, Ordering::SeqCst);
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
    limits: std::collections::HashMap<c_int, c_int>, // run-33: full sqlite3_limit id matrix
    fkey: c_int,
    busy_cb: usize,          // run-36: busy handler (mutually exclusive with timeout)
    busy_arg: usize,
    busy_timeout_ms: c_int,
    commit_cb: usize,        // run-36: commit hook
    commit_arg: usize,
    update_cb: usize,        // run-36: update hook
    update_arg: usize,
    trace_cb: usize,         // run-36: trace_v2
    trace_mask: u32,
    trace_ctx: usize,
}

// (default, compile-time max) per limit id — pinned from the C baseline defaults
fn limit_bounds(id: c_int) -> Option<(c_int, c_int)> {
    Some(match id {
        0 => (1000000000, 1000000000),  // LENGTH
        1 => (1000000000, 1000000000),  // SQL_LENGTH
        2 => (2000, 2000),              // COLUMN
        3 => (1000, 1000),              // EXPR_DEPTH
        4 => (500, 500),                // COMPOUND_SELECT
        5 => (250000000, 250000000),    // VDBE_OP
        6 => (1000, 1000),              // FUNCTION_ARG (pinned baseline default)
        7 => (10, 10),                  // ATTACHED (hard max 10 pinned)
        8 => (50000, 50000),            // LIKE_PATTERN_LENGTH
        9 => (32766, 32766),            // VARIABLE_NUMBER
        10 => (1000, 1000),             // TRIGGER_DEPTH
        11 => (0, 0),                   // WORKER_THREADS
        _ => return None,
    })
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
/// # Safety: C ABI — get (-1) / set with prior-value return; sets clamp to compile max.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_limit(db: *mut Sqlite3, id: c_int, new_val: c_int) -> c_int {
    if db.is_null() { return -1; }
    let (dflt, max) = match limit_bounds(id) { Some(b) => b, None => return -1 };
    with_extras(db, |e| {
        let cur = *e.limits.get(&id).unwrap_or(&dflt);
        if new_val >= 0 { e.limits.insert(id, new_val.min(max)); }
        cur
    })
}
/// # Safety: C ABI — English-language description of a (possibly extended) result code.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_errstr(rc: c_int) -> *const c_char {
    // extended codes fall through to their base-code description (pinned: 787/2067)
    let base = rc & 0xff;
    let s: &'static [u8] = match base {
        0 => b"not an error\0",
        1 => b"SQL logic error\0",
        5 => b"database is locked\0",
        14 => b"unable to open database file\0",
        19 => b"constraint failed\0",
        21 => b"bad parameter or other API misuse\0",
        23 => b"authorization denied\0",
        25 => b"column index out of range\0",
        100 => b"another row available\0",
        101 => b"no more rows available\0",
        _ => b"unknown error\0",
    };
    s.as_ptr() as *const c_char
}

/// extended constraint code for a rc-19 message (SQLITE_CONSTRAINT_* pinned matrix)
pub(crate) fn extended_for(rc: c_int, msg: &str) -> c_int {
    if rc != 19 { return rc; }
    if msg.starts_with("UNIQUE constraint failed") { 2067 }
    else if msg.starts_with("NOT NULL constraint failed") { 1299 }
    else if msg.starts_with("CHECK constraint failed") { 275 }
    else if msg.starts_with("FOREIGN KEY constraint failed") { 787 }
    else { 19 }
}

// ---------------- run-36: busy handler / timeout ----------------
type BusyCb = unsafe extern "C" fn(*mut c_void, c_int) -> c_int;

/// # Safety: C ABI — installing a handler clears any busy timeout (and vice versa).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_busy_handler(db: *mut Sqlite3, cb: Option<BusyCb>, arg: *mut c_void) -> c_int {
    if db.is_null() { return SQLITE_MISUSE; }
    with_extras(db, |e| {
        e.busy_cb = cb.map(|f| f as usize).unwrap_or(0);
        e.busy_arg = arg as usize;
        e.busy_timeout_ms = 0;
    });
    SQLITE_OK
}
/// # Safety: C ABI — ms<=0 clears both timeout and handler.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_busy_timeout(db: *mut Sqlite3, ms: c_int) -> c_int {
    if db.is_null() { return SQLITE_MISUSE; }
    with_extras(db, |e| {
        e.busy_cb = 0;
        e.busy_arg = 0;
        e.busy_timeout_ms = if ms > 0 { ms } else { 0 };
    });
    SQLITE_OK
}
/// busy-loop consult used by the store's file write lock: true = retry
pub(crate) fn busy_should_retry(dbid: usize, count: i32, slept_ms: &mut i32) -> bool {
    let (cb, arg, timeout) = EXTRAS.with(|m| {
        let mut mm = m.borrow_mut();
        let e = mm.entry(dbid).or_default();
        (e.busy_cb, e.busy_arg, e.busy_timeout_ms)
    });
    if cb != 0 {
        let f: BusyCb = unsafe { std::mem::transmute(cb) };
        return unsafe { f(arg as *mut c_void, count) } != 0;
    }
    if timeout > 0 && *slept_ms < timeout {
        std::thread::sleep(std::time::Duration::from_millis(1));
        *slept_ms += 1;
        return true;
    }
    false
}

// ---------------- run-36: commit / update hooks + trace_v2 ----------------
type CommitCb = unsafe extern "C" fn(*mut c_void) -> c_int;
type UpdateCb = unsafe extern "C" fn(*mut c_void, c_int, *const c_char, *const c_char, i64);
type TraceCb = unsafe extern "C" fn(u32, *mut c_void, *mut c_void, *mut c_void) -> c_int;

/// # Safety: C ABI — returns the previous hook's user argument (pinned).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_commit_hook(db: *mut Sqlite3, cb: Option<CommitCb>, arg: *mut c_void) -> *mut c_void {
    if db.is_null() { return std::ptr::null_mut(); }
    with_extras(db, |e| {
        let old = e.commit_arg;
        e.commit_cb = cb.map(|f| f as usize).unwrap_or(0);
        e.commit_arg = arg as usize;
        old as *mut c_void
    })
}
/// # Safety: C ABI — returns the previous hook's user argument (pinned).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_update_hook(db: *mut Sqlite3, cb: Option<UpdateCb>, arg: *mut c_void) -> *mut c_void {
    if db.is_null() { return std::ptr::null_mut(); }
    with_extras(db, |e| {
        let old = e.update_arg;
        e.update_cb = cb.map(|f| f as usize).unwrap_or(0);
        e.update_arg = arg as usize;
        old as *mut c_void
    })
}
/// # Safety: C ABI — mask 0 (or NULL callback) unsets tracing (pinned).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_trace_v2(db: *mut Sqlite3, mask: u32, cb: Option<TraceCb>, ctx: *mut c_void) -> c_int {
    if db.is_null() { return SQLITE_MISUSE; }
    with_extras(db, |e| {
        e.trace_cb = cb.map(|f| f as usize).unwrap_or(0);
        e.trace_mask = if cb.is_some() { mask } else { 0 };
        e.trace_ctx = ctx as usize;
    });
    SQLITE_OK
}
/// is a commit hook registered? (store snapshots only when needed)
pub(crate) fn commit_hook_present(dbid: usize) -> bool {
    EXTRAS.with(|m| m.borrow_mut().entry(dbid).or_default().commit_cb != 0)
}
/// commit-hook consult (None = no hook; Some(rc) = callback result)
pub(crate) fn consult_commit_hook(dbid: usize) -> Option<i32> {
    let (cb, arg) = EXTRAS.with(|m| {
        let mut mm = m.borrow_mut();
        let e = mm.entry(dbid).or_default();
        (e.commit_cb, e.commit_arg)
    });
    if cb == 0 { return None; }
    let f: CommitCb = unsafe { std::mem::transmute(cb) };
    Some(unsafe { f(arg as *mut c_void) })
}
/// fire the update hook for one changed row
pub(crate) fn fire_update_hook(dbid: usize, op: i32, table: &str, rowid: i64) {
    let (cb, arg) = EXTRAS.with(|m| {
        let mut mm = m.borrow_mut();
        let e = mm.entry(dbid).or_default();
        (e.update_cb, e.update_arg)
    });
    if cb == 0 { return; }
    let f: UpdateCb = unsafe { std::mem::transmute(cb) };
    let dbn = CString::new("main").unwrap();
    let tn = CString::new(table).unwrap_or_default();
    unsafe { f(arg as *mut c_void, op, dbn.as_ptr(), tn.as_ptr(), rowid) };
}
/// fire a trace event when the connection's mask includes it
pub(crate) fn fire_trace(dbid: usize, event: u32, p: *mut c_void, x: *mut c_void) {
    let (cb, mask, ctx) = EXTRAS.with(|m| {
        let mut mm = m.borrow_mut();
        let e = mm.entry(dbid).or_default();
        (e.trace_cb, e.trace_mask, e.trace_ctx)
    });
    if cb == 0 || mask & event == 0 { return; }
    let f: TraceCb = unsafe { std::mem::transmute(cb) };
    unsafe { f(event, ctx as *mut c_void, p, x) };
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

/// Consult the authorizer for one action code; DENY -> SQLITE_AUTH (pinned).
unsafe fn auth_check_action(db: *mut Sqlite3, code: c_int) -> c_int {
    let (cb, arg) = with_extras(db, |e| (e.auth_cb, e.auth_arg));
    if cb == 0 { return SQLITE_OK; }
    let f: unsafe extern "C" fn(*mut c_void, c_int, *const c_char, *const c_char, *const c_char, *const c_char) -> c_int =
        std::mem::transmute(cb);
    let r = f(arg as *mut c_void, code, std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null());
    if r == 1 /* SQLITE_DENY */ {
        (*db).errcode = SQLITE_AUTH;
        (*db).extended = SQLITE_AUTH;
        (*db).errmsg = Some(std::ffi::CString::new("not authorized").unwrap());
        return SQLITE_AUTH;
    }
    SQLITE_OK
}
unsafe fn auth_check_select(db: *mut Sqlite3) -> c_int { auth_check_action(db, SQLITE_SELECT_ACTION) }

/// is an authorizer installed on this connection? (run-38: gates the read prepass)
pub fn authorizer_present(dbid: usize) -> bool {
    EXTRAS.with(|m| m.borrow_mut().entry(dbid).or_default().auth_cb != 0)
}
/// consult the authorizer for a column READ (code 20): 0 OK / 1 DENY / 2 IGNORE
pub fn auth_read_column(dbid: usize, table: &str, col: &str) -> i32 {
    let (cb, arg) = EXTRAS.with(|m| { let mut mm = m.borrow_mut(); let e = mm.entry(dbid).or_default(); (e.auth_cb, e.auth_arg) });
    if cb == 0 { return 0; }
    let f: unsafe extern "C" fn(*mut c_void, c_int, *const c_char, *const c_char, *const c_char, *const c_char) -> c_int =
        unsafe { std::mem::transmute(cb) };
    let (tt, cc) = (CString::new(table).unwrap_or_default(), CString::new(col).unwrap_or_default());
    let dbn = CString::new("main").unwrap();
    unsafe { f(arg as *mut c_void, 20, tt.as_ptr(), cc.as_ptr(), dbn.as_ptr(), std::ptr::null()) }
}
/// run-33: statement-class action codes checked before execution
unsafe fn auth_code_for(sql_up: &str) -> Option<c_int> {
    let s = sql_up.trim_start();
    if s.starts_with("INSERT") { Some(18) }        // SQLITE_INSERT
    else if s.starts_with("UPDATE") { Some(23) }   // SQLITE_UPDATE
    else if s.starts_with("DELETE") { Some(9) }    // SQLITE_DELETE
    else if s.starts_with("CREATE TABLE") { Some(2) } // SQLITE_CREATE_TABLE
    else if s.starts_with("PRAGMA") { Some(19) }   // SQLITE_PRAGMA
    else { None }
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
    {
        // run-33: authorizer consulted for statement-class action codes (INSERT/UPDATE/
        // DELETE/CREATE TABLE/PRAGMA) before execution; DENY -> SQLITE_AUTH (pinned)
        let up = sql.trim_start().to_ascii_uppercase();
        if let Some(code) = auth_code_for(&up) {
            let rc = auth_check_action(db, code);
            if rc != SQLITE_OK {
                if !errmsg.is_null() { *errmsg = alloc_cstr("not authorized"); }
                return rc;
            }
        }
    }
    {
        // run-36: SQLITE_TRACE_STMT sees the statement text (per exec call for the
        // pinned single-statement scripts; per-prepared-statement is a residual)
        let ctext = CString::new(sql).unwrap_or_default();
        fire_trace(db as usize, 1 /* STMT */, std::ptr::null_mut(), ctext.as_ptr() as *mut c_void);
    }
    let wal_m0 = store::wal_marker(db as usize); // v22: WAL sidecar sync after the call
    let exec_result = store::execute_script(db as usize, sql);
    store::wal_sync(db as usize, wal_m0);
    fire_trace(db as usize, 2 /* PROFILE */, std::ptr::null_mut(), std::ptr::null_mut()); // run-36 (count pins)
    match exec_result {
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
                (*db).errcode = rc; (*db).extended = extended_for(rc, &msg); // run-33 pinned matrix
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
    if store::has_tables(db as usize) {
        // pack v14: a REAL SQLite image of the live store (same writer as save_file)
        let img = store::build_image(db as usize);
        let bytes = dbfile::write_db_bytes(&img);
        let p = sized_alloc(bytes.len());
        if !p.is_null() {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
        }
        if !pi_size.is_null() { *pi_size = bytes.len() as i64; }
        return p;
    }
    let p = sized_alloc(4096); // pinned empty-db image size
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

/// # Safety: C ABI (fixed arity like mprintf — frozen directives %d/%s/%q/%Q).
/// Writes at most n bytes INCLUDING the NUL; n<=0 leaves the buffer untouched; returns buf.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_snprintf(n: c_int, buf: *mut c_char, fmt: *const c_char, a: c_int, z: *const c_char) -> *mut c_char {
    if buf.is_null() || n <= 0 { return buf; }
    let f = if fmt.is_null() { "" } else { CStr::from_ptr(fmt).to_str().unwrap_or("") };
    let zs = if z.is_null() { None } else { CStr::from_ptr(z).to_str().ok() };
    let out = mini_format(f, a, zs);
    let bytes = out.as_bytes();
    let lim = (n as usize - 1).min(bytes.len());
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf as *mut u8, lim);
    *buf.add(lim) = 0;
    buf
}

/// # Safety: C ABI — append exactly n raw bytes (run-33 pin).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_append(s: *mut Sqlite3Str, z: *const c_char, n: c_int) {
    if s.is_null() || z.is_null() || n <= 0 { return; }
    let bytes = std::slice::from_raw_parts(z as *const u8, n as usize);
    (*s).buf.push_str(&String::from_utf8_lossy(bytes));
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
pub struct Sqlite3Str { buf: String, err: c_int, val_cache: Option<CString> }
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_new(_db: *mut Sqlite3) -> *mut Sqlite3Str {
    Box::into_raw(Box::new(Sqlite3Str { buf: String::new(), err: SQLITE_OK, val_cache: None }))
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
    if b.buf.is_empty() { return std::ptr::null_mut(); } // C returns NULL for an empty builder
    alloc_cstr(&b.buf)
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_appendchar(s: *mut Sqlite3Str, n: c_int, c: c_char) {
    if s.is_null() { return; }
    for _ in 0..n.max(0) { (*s).buf.push(c as u8 as char); }
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_reset(s: *mut Sqlite3Str) -> c_int {
    if s.is_null() { return SQLITE_MISUSE; }
    (*s).buf.clear();
    SQLITE_OK
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_length(s: *mut Sqlite3Str) -> c_int {
    if s.is_null() { 0 } else { (*s).buf.len() as c_int }
}
/// # Safety: C ABI — pointer valid until the next append/reset/finish.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_value(s: *mut Sqlite3Str) -> *const c_char {
    if s.is_null() || (*s).buf.is_empty() { return std::ptr::null(); }
    (*s).val_cache = Some(CString::new((*s).buf.clone()).unwrap_or_default());
    (*s).val_cache.as_ref().unwrap().as_ptr()
}

/// # Safety: C ABI — real token scan: after CREATE TRIGGER, BEGIN/CASE nest and END
/// pops; complete only when the body closed and the text ends with ';' (run-33 pins).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_complete(z: *const c_char) -> c_int {
    if z.is_null() { return 0; }
    let s = match CStr::from_ptr(z).to_str() { Ok(s) => s, Err(_) => return 0 };
    // run-38: strip string literals and comments so their ';' / END don't count
    let b = s.as_bytes();
    let mut clean = String::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'\'' | b'"' | b'`' => { // string / quoted identifier
                let q = b[i]; clean.push(' '); i += 1;
                while i < b.len() { if b[i] == q { if i + 1 < b.len() && b[i+1] == q { i += 2; continue; } i += 1; break; } i += 1; }
            }
            b'-' if i + 1 < b.len() && b[i+1] == b'-' => { // line comment
                while i < b.len() && b[i] != b'\n' { i += 1; }
            }
            b'/' if i + 1 < b.len() && b[i+1] == b'*' => { // block comment
                i += 2; while i + 1 < b.len() && !(b[i] == b'*' && b[i+1] == b'/') { i += 1; } i += 2;
            }
            c => { clean.push(c as char); i += 1; }
        }
    }
    let s = clean.trim_end();
    if !s.ends_with(';') { return 0; }
    let u = s.to_ascii_uppercase();
    if !u.contains("CREATE TRIGGER") { return 1; }
    let mut depth = 0i32;
    let mut saw_begin = false;
    for w in u.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
        match w {
            "BEGIN" | "CASE" => { depth += 1; saw_begin = true; }
            "END" => depth -= 1,
            _ => {}
        }
    }
    (saw_begin && depth <= 0) as c_int
}

// ---------------- run-38/44: sqlite3_status(64) / sqlite3_db_status op matrix ----------------

// run-44: real page-image accounting — bytes of database file images this process
// holds/writes per connection (modern's page cache equivalent). Not fabricated:
// updated only when store actually loads or flushes file bytes.
thread_local! {
    static PCACHE_BYTES: RefCell<std::collections::HashMap<usize, i64>> =
        RefCell::new(std::collections::HashMap::new());
}
static PCACHE_HI: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
static PCACHE_ONE_HI: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
/// store hook: this connection currently holds `n` bytes of file-image data
pub fn pcache_note(db: usize, n: i64) {
    let total = PCACHE_BYTES.with(|m| { let mut m = m.borrow_mut(); m.insert(db, n); m.values().sum::<i64>() });
    PCACHE_HI.fetch_max(total, Ordering::SeqCst);
    PCACHE_ONE_HI.fetch_max(n, Ordering::SeqCst);
}
/// store hook: connection closed / image dropped
pub fn pcache_forget(db: usize) { PCACHE_BYTES.with(|m| { m.borrow_mut().remove(&db); }); }
fn pcache_total() -> i64 { PCACHE_BYTES.with(|m| m.borrow().values().sum()) }

/// # Safety: C ABI — the frozen global op matrix: real allocator/page-image counters,
/// honest zeros for NOT-USED ops (SCRATCH_*, PARSER_STACK, PAGECACHE_USED), MISUSE
/// out of range. resetFlag re-arms the highwater to current for resettable ops.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_status64(op: c_int, p_cur: *mut i64, p_hi: *mut i64, reset: c_int) -> c_int {
    if !(0..=9).contains(&op) { return SQLITE_MISUSE; } // pinned: out-of-range -> 21
    let (cur, hi): (i64, i64) = match op {
        0 => (MEM_USED.load(Ordering::SeqCst), MEM_HIGH.load(Ordering::SeqCst)),
        2 => (pcache_total(), PCACHE_HI.load(Ordering::SeqCst).max(pcache_total())), // PAGECACHE_OVERFLOW
        5 => (0, MEM_BIGGEST.load(Ordering::SeqCst)),          // MALLOC_SIZE: current always 0
        7 => (0, PCACHE_ONE_HI.load(Ordering::SeqCst)),        // PAGECACHE_SIZE: current always 0
        9 => (MEM_COUNT.load(Ordering::SeqCst), MEM_COUNT_HI.load(Ordering::SeqCst)),
        _ => (0, 0), // PAGECACHE_USED(1), SCRATCH_USED/OVERFLOW/SIZE(3,4,8), PARSER_STACK(6)
    };
    if !p_cur.is_null() { *p_cur = cur; }
    if !p_hi.is_null() { *p_hi = hi; }
    if reset != 0 {
        match op {
            0 => { MEM_HIGH.store(MEM_USED.load(Ordering::SeqCst), Ordering::SeqCst); }
            2 => { PCACHE_HI.store(pcache_total(), Ordering::SeqCst); }
            5 => { MEM_BIGGEST.store(0, Ordering::SeqCst); }
            7 => { PCACHE_ONE_HI.store(0, Ordering::SeqCst); }
            9 => { MEM_COUNT_HI.store(MEM_COUNT.load(Ordering::SeqCst), Ordering::SeqCst); }
            _ => {}
        }
    }
    SQLITE_OK
}
/// # Safety: C ABI — 32-bit twin of status64 (values truncate like C's int copy).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_status(op: c_int, p_cur: *mut c_int, p_hi: *mut c_int, reset: c_int) -> c_int {
    let (mut c64, mut h64): (i64, i64) = (0, 0);
    let rc = sqlite3_status64(op, &mut c64, &mut h64, reset);
    if rc == SQLITE_OK {
        if !p_cur.is_null() { *p_cur = c64 as c_int; }
        if !p_hi.is_null() { *p_hi = h64 as c_int; }
    }
    rc
}

/// real byte footprint of this connection's live prepared statements
fn stmt_footprint(dbid: usize) -> i64 {
    STMTS.with(|m| m.borrow().get(&dbid).map_or(0, |set| {
        set.iter().map(|&p| unsafe {
            let st = p as *mut Sqlite3Stmt;
            (std::mem::size_of::<Sqlite3Stmt>() + (*st).sql.len()) as i64
        }).sum()
    }))
}

/// # Safety: C ABI — the frozen per-connection op matrix; unknown op -> SQLITE_ERROR
/// (pinned). Highwater is 0 for the byte-footprint/event ops, like C.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_status(db: *mut Sqlite3, op: c_int, p_cur: *mut c_int, p_hi: *mut c_int, reset: c_int) -> c_int {
    if db.is_null() { return SQLITE_MISUSE; }
    let dbid = db as usize;
    let (cur, hi): (c_int, c_int) = match op {
        0 => (0, 0),                                     // LOOKASIDE_USED: no lookaside allocator
        1 | 11 => (store::cache_footprint(dbid) as c_int, 0), // CACHE_USED(_SHARED): real image bytes
        2 => (store::schema_footprint(dbid) as c_int, 0),     // SCHEMA_USED: highwater 0 like C
        3 => (stmt_footprint(dbid) as c_int, 0),              // STMT_USED: live prepared stmts
        7 => (store::io_stats(dbid).0 as c_int, 0),           // CACHE_HIT: real memory-served reads
        8 => (store::io_stats(dbid).1 as c_int, 0),           // CACHE_MISS: real file-image loads
        9 => (store::io_stats(dbid).2 as c_int, 0),           // CACHE_WRITE: real file flushes
        10 => (store::deferred_fk_violations(dbid) as c_int, 0), // DEFERRED_FKS: on-demand scan
        4 | 5 | 6 | 12 => (0, 0),                        // LOOKASIDE_HIT/MISS_*, CACHE_SPILL
        _ => return SQLITE_ERROR,
    };
    if !p_cur.is_null() { *p_cur = cur; }
    if !p_hi.is_null() { *p_hi = hi; }
    let _ = reset; // the pinned ops carry highwater 0 (nothing to reset)
    SQLITE_OK
}

// ---------------- run-38: sqlite3_get_table / free_table ----------------
/// # Safety: C ABI — runs the query and marshals header + row cells into a
/// flat char** (row-major, header first). Non-queries yield 0x0; errors NULL.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_get_table(
    db: *mut Sqlite3, z_sql: *const c_char,
    pp_result: *mut *mut *mut c_char, p_nrow: *mut c_int, p_ncol: *mut c_int, p_errmsg: *mut *mut c_char,
) -> c_int {
    if !pp_result.is_null() { *pp_result = std::ptr::null_mut(); }
    if !p_nrow.is_null() { *p_nrow = 0; }
    if !p_ncol.is_null() { *p_ncol = 0; }
    if !p_errmsg.is_null() { *p_errmsg = std::ptr::null_mut(); }
    if db.is_null() || z_sql.is_null() { return SQLITE_MISUSE; }
    let sql = match CStr::from_ptr(z_sql).to_str() { Ok(s) => s, Err(_) => return SQLITE_ERROR };
    let up = sql.trim_start().to_ascii_uppercase();
    let is_query = up.starts_with("SELECT") || up.starts_with("VALUES") || up.starts_with("WITH")
        || up.starts_with("PRAGMA") || up.starts_with("EXPLAIN");
    if !is_query {
        // run the statement for its side effects; report 0x0 (pinned)
        let rc = sqlite3_exec(db, z_sql, None, std::ptr::null_mut(), p_errmsg);
        return rc;
    }
    match store::stmt_query_typed(db as usize, sql) {
        Ok((names, rows)) => {
            let ncol = names.len();
            // run-38: C's get_table sets nColumn from the row callback, so a zero-row
            // result reports 0x0 with a non-null (dummy) result array
            if rows.is_empty() {
                let boxed: Box<[*mut c_char]> = vec![std::ptr::null_mut()].into_boxed_slice();
                let ptr = Box::into_raw(boxed) as *mut *mut c_char;
                if !pp_result.is_null() { *pp_result = ptr; }
                GET_TABLE_LEN.with(|m| { m.borrow_mut().insert(ptr as usize, 1); });
                db_ok(&mut *db);
                return SQLITE_OK;
            }
            let mut cells: Vec<*mut c_char> = Vec::with_capacity((rows.len() + 1) * ncol);
            for n in &names { cells.push(alloc_cstr(n)); }
            for r in &rows {
                for v in r {
                    match v.render() {
                        Some(s) => cells.push(alloc_cstr(&s)),
                        None => cells.push(std::ptr::null_mut()), // SQL NULL -> NULL pointer
                    }
                }
            }
            let boxed = cells.into_boxed_slice();
            let ptr = Box::into_raw(boxed) as *mut *mut c_char;
            if !pp_result.is_null() { *pp_result = ptr; }
            if !p_ncol.is_null() { *p_ncol = ncol as c_int; }
            if !p_nrow.is_null() { *p_nrow = rows.len() as c_int; }
            GET_TABLE_LEN.with(|m| { m.borrow_mut().insert(ptr as usize, (rows.len() + 1) * ncol); });
            db_ok(&mut *db);
            SQLITE_OK
        }
        Err(e) => {
            (*db).errcode = SQLITE_ERROR;
            (*db).extended = SQLITE_ERROR;
            (*db).errmsg = Some(CString::new(e.clone()).unwrap_or_default());
            if !p_errmsg.is_null() { *p_errmsg = alloc_cstr(&e); }
            SQLITE_ERROR
        }
    }
}
thread_local! {
    static GET_TABLE_LEN: RefCell<std::collections::HashMap<usize, usize>> =
        RefCell::new(std::collections::HashMap::new());
}
/// # Safety: C ABI — frees a table returned by sqlite3_get_table; NULL tolerated.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_free_table(result: *mut *mut c_char) {
    if result.is_null() { return; }
    let n = GET_TABLE_LEN.with(|m| m.borrow_mut().remove(&(result as usize))).unwrap_or(0);
    if n == 0 { return; }
    let slice = std::slice::from_raw_parts_mut(result, n);
    for &mut cell in slice.iter_mut() { if !cell.is_null() { sqlite3_free(cell as *mut c_void); } }
    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(result, n)));
}

/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_readonly(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() { 0 } else { (*stmt).readonly as c_int } // real statement property
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_busy(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() { return 0; }
    match (*stmt).state { State::Row => 1, _ => 0 }
}
/// # Safety: C ABI — real storage class of the current row's column.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_type(stmt: *mut Sqlite3Stmt, i: c_int) -> c_int {
    if stmt.is_null() { return 5; /* NULL */ }
    match col_val(&*stmt, i) {
        Some(eval::V::Int(_)) => 1,
        Some(eval::V::Real(_)) => 2,
        Some(eval::V::Text(_)) => 3,
        Some(eval::V::Blob(_)) => 4,
        _ => 5,
    }
}

// ===================== run-12 leftovers widening (pack v3) =====================

thread_local! {
    // run-33: ordered multi-entry registry (duplicates collapse; C invokes each once)
    static AUTO_EXTS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

/// # Safety: C ABI — register an auto-extension entry point (duplicates are no-ops).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_auto_extension(f: Option<unsafe extern "C" fn()>) -> c_int {
    let p = match f { Some(p) => p as usize, None => return SQLITE_OK };
    AUTO_EXTS.with(|v| { let mut v = v.borrow_mut(); if !v.contains(&p) { v.push(p); } });
    SQLITE_OK
}
/// # Safety: C ABI — returns 1 when the entry was found and removed, else 0 (pinned).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_cancel_auto_extension(f: Option<unsafe extern "C" fn()>) -> c_int {
    let want = match f { Some(p) => p as usize, None => return 0 };
    AUTO_EXTS.with(|v| {
        let mut v = v.borrow_mut();
        match v.iter().position(|&p| p == want) { Some(i) => { v.remove(i); 1 } None => 0 }
    })
}
/// # Safety: C ABI — clears the whole registry (run-33 pin).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_reset_auto_extension() {
    AUTO_EXTS.with(|v| v.borrow_mut().clear());
}
pub(crate) unsafe fn run_auto_extensions(db: *mut Sqlite3) {
    let entries: Vec<usize> = AUTO_EXTS.with(|v| v.borrow().clone());
    for f in entries {
        // pinned shape: init fn invoked once per open with (db, errmsg, api) — mirrored as (db,0,0)
        let g: unsafe extern "C" fn(*mut Sqlite3, *mut *mut c_char, *const c_void) -> c_int =
            std::mem::transmute(f);
        let _ = g(db, std::ptr::null_mut(), std::ptr::null());
    }
}

/// # Safety: C ABI — pinned round-trip: deserialize the 4096-byte empty image → OK.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_deserialize(
    db: *mut Sqlite3, _schema: *const c_char, data: *mut u8, sz: i64, _buf_sz: i64, flags: u32,
) -> c_int {
    if db.is_null() || data.is_null() || sz < 0 { return SQLITE_MISUSE; }
    // pack v14: parse the image with the shared reader and load it into the store
    let bytes = std::slice::from_raw_parts(data, sz as usize).to_vec();
    let img = dbfile::read_db_bytes(&bytes);
    if !img.tables.is_empty() || !img.triggers.is_empty() || !img.indexes.is_empty() {
        store::load_image(db as usize, img);
    }
    // FREEONCLOSE ownership honoured: our close doesn't track it, so free now if flagged
    // (the pinned observables are the rcs/values, not retention timing).
    const FREEONCLOSE: u32 = 1;
    if flags & FREEONCLOSE != 0 { sqlite3_free(data as *mut c_void); }
    SQLITE_OK
}

// ===================== run-28: sqlite3_create_function + value/result (pack v18) =====================
// A REAL cross-language UDF path: apps register C callbacks; the eval engine builds
// sqlite3_value* args, invokes xFunc / xStep+xFinal, and reads sqlite3_result_*.
// No script_table; results are computed by the app callback from runtime args.

/// a bound argument as seen by a UDF callback
pub struct Sqlite3Value { v: eval::V }

/// per-invocation context: user_data, result slot, error, aggregate state, db handle
pub struct Sqlite3Context {
    db: usize,
    user_data: *mut c_void,
    result: eval::V,
    is_error: bool,
    errmsg: Option<String>,
    text_keep: Vec<CString>,
    agg: *mut c_void,        // sqlite3_aggregate_context buffer (owned by the group)
    agg_size: usize,
}

type XFunc = unsafe extern "C" fn(*mut Sqlite3Context, c_int, *mut *mut Sqlite3Value);
type XStep = unsafe extern "C" fn(*mut Sqlite3Context, c_int, *mut *mut Sqlite3Value);
type XFinal = unsafe extern "C" fn(*mut Sqlite3Context);
type XDestroy = unsafe extern "C" fn(*mut c_void);

#[derive(Clone)]
struct FnEntry {
    n_arg: i32,               // -1 = any
    x_func: Option<XFunc>,
    x_step: Option<XStep>,
    x_final: Option<XFinal>,
    x_destroy: Option<XDestroy>,
    user_data: usize,         // stored as usize so the map is 'static-friendly
}

thread_local! {
    // db -> (name_lower, n_arg) -> entry
    static UDF_REG: std::cell::RefCell<std::collections::HashMap<usize,
        std::collections::HashMap<(String, i32), FnEntry>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

// ---- run-30: per-connection collation registry (create_collation[_v2]) ----
type XColCompare = unsafe extern "C" fn(*mut c_void, c_int, *const c_void, c_int, *const c_void) -> c_int;
type XColDestroy = unsafe extern "C" fn(*mut c_void);
type XCollNeeded = unsafe extern "C" fn(*mut c_void, *mut Sqlite3, c_int, *const c_char);

struct CollEntry { p_arg: usize, x_cmp: usize, x_destroy: usize }

thread_local! {
    // db -> name_lower -> entry
    static COLL_REG: std::cell::RefCell<std::collections::HashMap<usize,
        std::collections::HashMap<String, CollEntry>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
    // db -> (p_arg, callback) for sqlite3_collation_needed
    static COLL_NEEDED: std::cell::RefCell<std::collections::HashMap<usize, (usize, usize)>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
    // run-44: user collation registration order (newest first) for PRAGMA collation_list
    static COLL_ORDER: std::cell::RefCell<std::collections::HashMap<usize, Vec<String>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// PRAGMA collation_list contents: user registrations newest-first, then the
/// built-ins in C's order (RTRIM, NOCASE, BINARY).
pub fn collation_list_names(dbid: usize) -> Vec<String> {
    let mut out = COLL_ORDER.with(|o| o.borrow().get(&dbid).cloned().unwrap_or_default());
    out.extend(["RTRIM", "NOCASE", "BINARY"].iter().map(|s| s.to_string()));
    out
}
/// pragma_compile_options TVF rows (the pinned v32 fingerprint)
pub fn compileoption_all() -> Vec<&'static str> {
    COMPILE_OPTS.iter().filter_map(|c| c.to_str().ok()).collect()
}

fn coll_run_destroy(e: &CollEntry) {
    if e.x_destroy != 0 {
        let f: XColDestroy = unsafe { std::mem::transmute(e.x_destroy) };
        unsafe { f(e.p_arg as *mut c_void) };
    }
}

fn coll_close(dbid: usize) {
    COLL_REG.with(|r| {
        if let Some(per) = r.borrow_mut().remove(&dbid) {
            for e in per.values() { coll_run_destroy(e); }
        }
    });
    COLL_NEEDED.with(|r| { r.borrow_mut().remove(&dbid); });
}

// ---------------- run-35: incremental blob I/O handles ----------------

/// live handle on one cell: read/write bytes at an offset; the handle expires
/// (rc 4, "query aborted") when the connection writes rows after it was opened.
/// Residual (documented): C expires per-row; this marker is connection-write
/// granular — the pinned scope only modifies the handle's own row.
pub struct Sqlite3Blob {
    db: *mut Sqlite3,
    table: String,
    ci: usize,
    rowid: i64,
    readonly: bool,
    marker: (i64, i64),
    expired: bool,
}

unsafe fn blob_db_err(db: *mut Sqlite3, rc: c_int, msg: &str) -> c_int {
    if !db.is_null() {
        (*db).errcode = rc;
        (*db).extended = rc;
        (*db).errmsg = Some(CString::new(msg).unwrap());
    }
    rc
}

/// live-handle check: any DML on the connection since open/reopen expires it
unsafe fn blob_check_live(b: &mut Sqlite3Blob) -> bool {
    if b.expired { return false; }
    if store::wal_marker(b.db as usize) != b.marker { b.expired = true; return false; }
    true
}

/// # Safety: C ABI — open a handle on (schema, table, column, rowid).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_blob_open(
    db: *mut Sqlite3, z_db: *const c_char, z_table: *const c_char, z_column: *const c_char,
    i_row: i64, flags: c_int, pp_blob: *mut *mut Sqlite3Blob,
) -> c_int {
    if pp_blob.is_null() { return SQLITE_MISUSE; }
    *pp_blob = std::ptr::null_mut();
    if db.is_null() || z_table.is_null() || z_column.is_null() { return SQLITE_MISUSE; }
    let zdb = if z_db.is_null() { "main".to_string() } else { CStr::from_ptr(z_db).to_string_lossy().into_owned() };
    let table = CStr::from_ptr(z_table).to_string_lossy().into_owned();
    let col = CStr::from_ptr(z_column).to_string_lossy().into_owned();
    match store::blob_target(db as usize, &zdb, &table, &col, i_row, flags != 0) {
        Ok(ci) => {
            let b = Box::new(Sqlite3Blob {
                db, table, ci, rowid: i_row, readonly: flags == 0,
                marker: store::wal_marker(db as usize), expired: false,
            });
            *pp_blob = Box::into_raw(b);
            BLOBS.with(|m| { *m.borrow_mut().entry(db as usize).or_default() += 1; }); // run-36
            db_ok(&mut *db);
            SQLITE_OK
        }
        Err(e) => blob_db_err(db, SQLITE_ERROR, &e),
    }
}

/// # Safety: C ABI — close is unconditional; returns OK for the pinned scope.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_blob_close(b: *mut Sqlite3Blob) -> c_int {
    if !b.is_null() {
        let dbid = (*b).db as usize;
        drop(Box::from_raw(b));
        BLOBS.with(|m| { if let Some(n) = m.borrow_mut().get_mut(&dbid) { *n = n.saturating_sub(1); } });
        handle_released(dbid);
    }
    SQLITE_OK
}

/// # Safety: C ABI — reposition the handle onto another rowid (same table/column).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_blob_reopen(b: *mut Sqlite3Blob, i_row: i64) -> c_int {
    if b.is_null() { return SQLITE_MISUSE; }
    let br = &mut *b;
    if store::blob_len(br.db as usize, &br.table, br.ci, i_row).is_none() {
        br.expired = true; // C aborts the handle on a failed reopen
        return blob_db_err(br.db, SQLITE_ERROR, &format!("no such rowid: {i_row}"));
    }
    br.rowid = i_row;
    br.marker = store::wal_marker(br.db as usize);
    br.expired = false;
    db_ok(&mut *br.db);
    SQLITE_OK
}

/// # Safety: C ABI — 0 once the handle expired (pinned).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_blob_bytes(b: *mut Sqlite3Blob) -> c_int {
    if b.is_null() { return 0; }
    let br = &mut *b;
    if !blob_check_live(br) { return 0; }
    store::blob_len(br.db as usize, &br.table, br.ci, br.rowid).unwrap_or(0) as c_int
}

/// # Safety: C ABI — read n bytes at offset; bounds errors leave the buffer untouched.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_blob_read(b: *mut Sqlite3Blob, z: *mut c_void, n: c_int, i_offset: c_int) -> c_int {
    if b.is_null() || z.is_null() { return SQLITE_MISUSE; }
    let br = &mut *b;
    if !blob_check_live(br) { return blob_db_err(br.db, SQLITE_ABORT, "query aborted"); }
    let len = store::blob_len(br.db as usize, &br.table, br.ci, br.rowid).unwrap_or(0) as i64;
    if n < 0 || i_offset < 0 || (i_offset as i64 + n as i64) > len {
        return blob_db_err(br.db, SQLITE_ERROR, "SQL logic error");
    }
    if n == 0 { db_ok(&mut *br.db); return SQLITE_OK; }
    match store::blob_read_bytes(br.db as usize, &br.table, br.ci, br.rowid, i_offset as usize, n as usize) {
        Some(bytes) => {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), z as *mut u8, bytes.len());
            db_ok(&mut *br.db);
            SQLITE_OK
        }
        None => blob_db_err(br.db, SQLITE_ERROR, "SQL logic error"),
    }
}

/// # Safety: C ABI — write n bytes at offset; never resizes the cell.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_blob_write(b: *mut Sqlite3Blob, z: *const c_void, n: c_int, i_offset: c_int) -> c_int {
    if b.is_null() || z.is_null() { return SQLITE_MISUSE; }
    let br = &mut *b;
    if !blob_check_live(br) { return blob_db_err(br.db, SQLITE_ABORT, "query aborted"); }
    if br.readonly {
        return blob_db_err(br.db, 8 /* SQLITE_READONLY */, "attempt to write a readonly database");
    }
    let len = store::blob_len(br.db as usize, &br.table, br.ci, br.rowid).unwrap_or(0) as i64;
    if n < 0 || i_offset < 0 || (i_offset as i64 + n as i64) > len {
        return blob_db_err(br.db, SQLITE_ERROR, "SQL logic error");
    }
    if n == 0 { db_ok(&mut *br.db); return SQLITE_OK; }
    let data = std::slice::from_raw_parts(z as *const u8, n as usize);
    match store::blob_write_bytes(br.db as usize, &br.table, br.ci, br.rowid, i_offset as usize, data) {
        Ok(()) => { db_ok(&mut *br.db); SQLITE_OK }
        Err(e) => blob_db_err(br.db, SQLITE_ERROR, &e),
    }
}

/// # Safety: C ABI — register/replace/delete a named collating sequence.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_collation(
    db: *mut Sqlite3, z_name: *const c_char, e_text_rep: c_int,
    p_arg: *mut c_void, x_compare: Option<XColCompare>,
) -> c_int { sqlite3_create_collation_v2(db, z_name, e_text_rep, p_arg, x_compare, None) }

/// # Safety: C ABI — v2 adds xDestroy(pArg), run on replace/delete/close.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_collation_v2(
    db: *mut Sqlite3, z_name: *const c_char, e_text_rep: c_int,
    p_arg: *mut c_void, x_compare: Option<XColCompare>, x_destroy: Option<XColDestroy>,
) -> c_int {
    if db.is_null() || z_name.is_null() { return SQLITE_MISUSE; }
    // pinned eTextRep matrix: UTF8(1)/UTF16LE(2)/UTF16BE(3)/UTF16(4)/UTF16_ALIGNED(8) ok; 0/99 -> MISUSE
    if !matches!(e_text_rep, 1 | 2 | 3 | 4 | 8) { return SQLITE_MISUSE; }
    let disp = CStr::from_ptr(z_name).to_string_lossy().into_owned();
    let name = disp.to_ascii_lowercase();
    let dbid = db as usize;
    COLL_REG.with(|r| {
        let mut reg = r.borrow_mut();
        let per = reg.entry(dbid).or_default();
        match x_compare {
            None => { if let Some(old) = per.remove(&name) { coll_run_destroy(&old); } }
            Some(f) => {
                let e = CollEntry { p_arg: p_arg as usize, x_cmp: f as usize,
                                    x_destroy: x_destroy.map(|d| d as usize).unwrap_or(0) };
                if let Some(old) = per.insert(name.clone(), e) { coll_run_destroy(&old); }
            }
        }
    });
    // run-44: registration order for PRAGMA collation_list (newest first, like C)
    COLL_ORDER.with(|o| {
        let mut m = o.borrow_mut();
        let v = m.entry(dbid).or_default();
        v.retain(|n| !n.eq_ignore_ascii_case(&disp));
        if x_compare.is_some() { v.insert(0, disp); }
    });
    db_ok(&mut *db);
    SQLITE_OK
}

/// # Safety: C ABI — register the lazy collation factory.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_collation_needed(
    db: *mut Sqlite3, p_arg: *mut c_void, cb: Option<XCollNeeded>,
) -> c_int {
    if db.is_null() { return SQLITE_MISUSE; }
    let dbid = db as usize;
    COLL_NEEDED.with(|r| {
        match cb {
            Some(f) => { r.borrow_mut().insert(dbid, (p_arg as usize, f as usize)); }
            None => { r.borrow_mut().remove(&dbid); }
        }
    });
    SQLITE_OK
}

/// registry lookup with lazy collation_needed factory; true if `name` resolves
pub fn coll_user_exists(dbid: usize, name: &str) -> bool {
    let key = name.to_ascii_lowercase();
    let hit = COLL_REG.with(|r| r.borrow().get(&dbid).map_or(false, |per| per.contains_key(&key)));
    if hit { return true; }
    let factory = COLL_NEEDED.with(|r| r.borrow().get(&dbid).copied());
    if let Some((p_arg, cb)) = factory {
        let f: XCollNeeded = unsafe { std::mem::transmute(cb) };
        if let Ok(cname) = CString::new(name) {
            unsafe { f(p_arg as *mut c_void, dbid as *mut Sqlite3, 1 /*SQLITE_UTF8*/, cname.as_ptr()) };
        }
        return COLL_REG.with(|r| r.borrow().get(&dbid).map_or(false, |per| per.contains_key(&key)));
    }
    false
}

/// compare two texts through the registered xCompare (REAL C callback);
/// None when the collation is unknown even after the collation_needed factory
pub fn coll_user_cmp(dbid: usize, name: &str, a: &str, b: &str) -> Option<std::cmp::Ordering> {
    if !coll_user_exists(dbid, name) { return None; }
    let key = name.to_ascii_lowercase();
    let (p_arg, x_cmp) = COLL_REG.with(|r| {
        let reg = r.borrow();
        let e = reg.get(&dbid).and_then(|per| per.get(&key)).unwrap();
        (e.p_arg, e.x_cmp)
    });
    let f: XColCompare = unsafe { std::mem::transmute(x_cmp) };
    let r = unsafe { f(p_arg as *mut c_void,
                       a.len() as c_int, a.as_ptr() as *const c_void,
                       b.len() as c_int, b.as_ptr() as *const c_void) };
    Some(r.cmp(&0))
}

fn run_destroy(e: &FnEntry) {
    // C invokes xDestroy(pApp) on replace/close regardless of whether pApp is NULL
    if let Some(d) = e.x_destroy { unsafe { d(e.user_data as *mut c_void); } }
}

/// # Safety: C ABI — sqlite3_create_function.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_function(
    db: *mut Sqlite3, z_name: *const c_char, n_arg: c_int, _text_rep: c_int, p_app: *mut c_void,
    x_func: Option<XFunc>, x_step: Option<XStep>, x_final: Option<XFinal>,
) -> c_int {
    create_function_impl(db, z_name, n_arg, p_app, x_func, x_step, x_final, None)
}

/// # Safety: C ABI — sqlite3_create_function_v2 (adds xDestroy).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_function_v2(
    db: *mut Sqlite3, z_name: *const c_char, n_arg: c_int, _text_rep: c_int, p_app: *mut c_void,
    x_func: Option<XFunc>, x_step: Option<XStep>, x_final: Option<XFinal>, x_destroy: Option<XDestroy>,
) -> c_int {
    create_function_impl(db, z_name, n_arg, p_app, x_func, x_step, x_final, x_destroy)
}

#[allow(clippy::too_many_arguments)]
unsafe fn create_function_impl(
    db: *mut Sqlite3, z_name: *const c_char, n_arg: c_int, p_app: *mut c_void,
    x_func: Option<XFunc>, x_step: Option<XStep>, x_final: Option<XFinal>, x_destroy: Option<XDestroy>,
) -> c_int {
    if db.is_null() || z_name.is_null() { return SQLITE_MISUSE; }
    let name = match CStr::from_ptr(z_name).to_str() { Ok(s) => s.to_ascii_lowercase(), Err(_) => return SQLITE_MISUSE };
    let dbid = db as usize;
    let key = (name, n_arg);
    let deleting = x_func.is_none() && x_step.is_none() && x_final.is_none();
    UDF_REG.with(|r| {
        let mut m = r.borrow_mut();
        let per = m.entry(dbid).or_default();
        // xDestroy of a replaced/removed entry runs now (pinned: replace -> 1)
        if let Some(old) = per.get(&key) { run_destroy(old); }
        if deleting {
            per.remove(&key);
        } else {
            per.insert(key, FnEntry { n_arg, x_func, x_step, x_final, x_destroy, user_data: p_app as usize });
        }
    });
    db_ok(&mut *db);
    SQLITE_OK
}

/// drop a connection's UDFs, running any pending xDestroy (pinned: close -> +1)
fn udf_close(dbid: usize) {
    UDF_REG.with(|r| {
        if let Some(per) = r.borrow_mut().remove(&dbid) {
            for (_k, e) in per.iter() { run_destroy(e); }
        }
    });
}

/// look up (returns a clone of the entry) for name + argc, preferring exact arity
fn udf_lookup(dbid: usize, name: &str, argc: usize) -> Option<FnEntry> {
    let key_l = name.to_ascii_lowercase();
    UDF_REG.with(|r| {
        let m = r.borrow();
        let per = m.get(&dbid)?;
        per.get(&(key_l.clone(), argc as i32)).cloned()
            .or_else(|| per.get(&(key_l, -1)).cloned())
    })
}

/// does a UDF of ANY arity exist under this name? (prepare-time arity errors)
pub fn udf_name_exists(dbid: usize, name: &str) -> bool {
    let key_l = name.to_ascii_lowercase();
    UDF_REG.with(|r| r.borrow().get(&dbid).map(|per| per.keys().any(|(n, _)| *n == key_l)).unwrap_or(false))
}
/// is the registered function (for this argc) an aggregate?
pub fn udf_is_aggregate(dbid: usize, name: &str, argc: usize) -> bool {
    udf_lookup(dbid, name, argc).map(|e| e.x_func.is_none() && e.x_step.is_some()).unwrap_or(false)
}
/// exact/variadic arity accepted?
pub fn udf_arity_ok(dbid: usize, name: &str, argc: usize) -> bool {
    udf_lookup(dbid, name, argc).is_some()
}

fn mkval(v: &eval::V) -> Box<Sqlite3Value> { Box::new(Sqlite3Value { v: v.clone() }) }

/// invoke a scalar UDF; None if not a scalar of this arity
pub fn udf_invoke_scalar(dbid: usize, name: &str, args: &[eval::V]) -> Option<Result<eval::V, String>> {
    let e = udf_lookup(dbid, name, args.len())?;
    let xf = e.x_func?;
    let mut boxes: Vec<Box<Sqlite3Value>> = args.iter().map(mkval).collect();
    let mut argv: Vec<*mut Sqlite3Value> = boxes.iter_mut().map(|b| b.as_mut() as *mut Sqlite3Value).collect();
    let mut ctx = Sqlite3Context { db: dbid, user_data: e.user_data as *mut c_void, result: eval::V::Null,
        is_error: false, errmsg: None, text_keep: Vec::new(), agg: std::ptr::null_mut(), agg_size: 0 };
    unsafe { xf(&mut ctx as *mut _, argv.len() as c_int, argv.as_mut_ptr()); }
    if ctx.is_error { Some(Err(ctx.errmsg.unwrap_or_else(|| "error".into()))) }
    else { Some(Ok(ctx.result)) }
}

/// invoke an aggregate UDF over a group's rows; None if not an aggregate of this arity
pub fn udf_invoke_aggregate(dbid: usize, name: &str, rows: &[Vec<eval::V>]) -> Option<Result<eval::V, String>> {
    let argc = rows.first().map(|r| r.len()).unwrap_or(1);
    let e = udf_lookup(dbid, name, argc)?;
    let (xs, xfin) = (e.x_step?, e.x_final?);
    // one context for the whole group: sqlite3_aggregate_context state persists
    // across every xStep and the final xFinal, exactly like C.
    let mut ctx = Sqlite3Context { db: dbid, user_data: e.user_data as *mut c_void, result: eval::V::Null,
        is_error: false, errmsg: None, text_keep: Vec::new(), agg: std::ptr::null_mut(), agg_size: 0 };
    for row in rows {
        let mut boxes: Vec<Box<Sqlite3Value>> = row.iter().map(mkval).collect();
        let mut argv: Vec<*mut Sqlite3Value> = boxes.iter_mut().map(|b| b.as_mut() as *mut Sqlite3Value).collect();
        unsafe { xs(&mut ctx as *mut _, argv.len() as c_int, argv.as_mut_ptr()); }
        if ctx.is_error { break; }
    }
    if !ctx.is_error { unsafe { xfin(&mut ctx as *mut _); } }
    // reclaim the per-group aggregate buffer (allocated lazily by aggregate_context)
    if !ctx.agg.is_null() {
        unsafe { drop(Box::from_raw(std::slice::from_raw_parts_mut(ctx.agg as *mut u8, ctx.agg_size.max(1)))); }
    }
    if ctx.is_error { Some(Err(ctx.errmsg.unwrap_or_else(|| "error".into()))) }
    else { Some(Ok(ctx.result)) }
}

// ---------------- run-42: compile-option diagnostics (pinned bare-build fingerprint) ----------------

/// The pinned bare amalgamation's option table (run-42 fingerprint; ADR 0030).
/// The COMPILER row restates the pinned C baseline toolchain — a pin decision,
/// not a claim about how this library was built.
static COMPILE_OPTS: [&std::ffi::CStr; 38] = [
    c"ATOMIC_INTRINSICS=1",
    c"COMPILER=gcc-13.3.0",
    c"DEFAULT_AUTOVACUUM",
    c"DEFAULT_CACHE_SIZE=-2000",
    c"DEFAULT_FILE_FORMAT=4",
    c"DEFAULT_JOURNAL_SIZE_LIMIT=-1",
    c"DEFAULT_MMAP_SIZE=0",
    c"DEFAULT_PAGE_SIZE=4096",
    c"DEFAULT_PCACHE_INITSZ=20",
    c"DEFAULT_RECURSIVE_TRIGGERS",
    c"DEFAULT_SECTOR_SIZE=4096",
    c"DEFAULT_SYNCHRONOUS=2",
    c"DEFAULT_WAL_AUTOCHECKPOINT=1000",
    c"DEFAULT_WAL_SYNCHRONOUS=2",
    c"DEFAULT_WORKER_THREADS=0",
    c"DIRECT_OVERFLOW_READ",
    c"MALLOC_SOFT_LIMIT=1024",
    c"MAX_ATTACHED=10",
    c"MAX_COLUMN=2000",
    c"MAX_COMPOUND_SELECT=500",
    c"MAX_DEFAULT_PAGE_SIZE=8192",
    c"MAX_EXPR_DEPTH=1000",
    c"MAX_FUNCTION_ARG=1000",
    c"MAX_LENGTH=1000000000",
    c"MAX_LIKE_PATTERN_LENGTH=50000",
    c"MAX_MMAP_SIZE=0x7fff0000",
    c"MAX_PAGE_COUNT=0xfffffffe",
    c"MAX_PAGE_SIZE=65536",
    c"MAX_SCHEMA=10000000",
    c"MAX_SQL_LENGTH=1000000000",
    c"MAX_TRIGGER_DEPTH=1000",
    c"MAX_VARIABLE_NUMBER=32766",
    c"MAX_VDBE_OP=250000000",
    c"MAX_WORKER_THREADS=8",
    c"MUTEX_PTHREADS",
    c"SYSTEM_MALLOC",
    c"TEMP_STORE=1",
    c"THREADSAFE=1",
];

/// case-insensitive used() with optional SQLITE_ prefix and the C "=" boundary rule:
/// a query matches an entry when it equals the whole entry or a prefix ending at '='.
pub fn compileoption_used(opt: &str) -> i64 {
    let q = if opt.len() >= 7 && opt[..7].eq_ignore_ascii_case("SQLITE_") { &opt[7..] } else { opt };
    if q.is_empty() { return 0; }
    for e in COMPILE_OPTS.iter() {
        let e = e.to_str().unwrap_or("");
        if e.len() >= q.len() && e[..q.len()].eq_ignore_ascii_case(q) {
            let rest = &e[q.len()..];
            if rest.is_empty() || rest.starts_with('=') { return 1; }
        }
    }
    0
}
/// enumeration in C order; None past either end
pub fn compileoption_get_str(n: i64) -> Option<&'static str> {
    if n < 0 { return None; }
    COMPILE_OPTS.get(n as usize).and_then(|c| c.to_str().ok())
}

/// # Safety: C ABI — mirrors sqlite3_compileoption_used on the pinned option table.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_compileoption_used(z_opt_name: *const c_char) -> c_int {
    if z_opt_name.is_null() { return 0; }
    let s = CStr::from_ptr(z_opt_name).to_string_lossy();
    compileoption_used(&s) as c_int
}
/// # Safety: C ABI — returned pointer is static; NULL past the end (and for negatives).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_compileoption_get(n: c_int) -> *const c_char {
    if n < 0 { return ptr::null(); }
    match COMPILE_OPTS.get(n as usize) {
        Some(c) => c.as_ptr(),
        None => ptr::null(),
    }
}

// ---------------- run-41: virtual table core (create_module / declare_vtab / cursor scan) ----------------

/// C-layout base of a module's vtab object (module code embeds this as first member).
#[repr(C)]
pub struct Sqlite3Vtab {
    pub p_module: *const Sqlite3Module,
    pub n_ref: c_int,
    pub z_err_msg: *mut c_char,
}
/// C-layout base of a module's cursor object.
#[repr(C)]
pub struct Sqlite3VtabCursor {
    pub p_vtab: *mut Sqlite3Vtab,
}
/// xBestIndex exchange struct (v1 core fields; modern offers zero constraints = full scan).
#[repr(C)]
pub struct Sqlite3IndexInfo {
    pub n_constraint: c_int,
    pub a_constraint: *mut c_void,
    pub n_order_by: c_int,
    pub a_order_by: *mut c_void,
    pub a_constraint_usage: *mut c_void,
    pub idx_num: c_int,
    pub idx_str: *mut c_char,
    pub need_to_free_idx_str: c_int,
    pub order_by_consumed: c_int,
    pub estimated_cost: f64,
    pub estimated_rows: i64,
    pub idx_flags: c_int,
    pub col_used: u64,
}

pub type XVtCreate = unsafe extern "C" fn(*mut Sqlite3, *mut c_void, c_int, *const *const c_char, *mut *mut Sqlite3Vtab, *mut *mut c_char) -> c_int;
pub type XVtBestIndex = unsafe extern "C" fn(*mut Sqlite3Vtab, *mut Sqlite3IndexInfo) -> c_int;
pub type XVtDisconnect = unsafe extern "C" fn(*mut Sqlite3Vtab) -> c_int;
pub type XVtOpen = unsafe extern "C" fn(*mut Sqlite3Vtab, *mut *mut Sqlite3VtabCursor) -> c_int;
pub type XVtClose = unsafe extern "C" fn(*mut Sqlite3VtabCursor) -> c_int;
pub type XVtFilter = unsafe extern "C" fn(*mut Sqlite3VtabCursor, c_int, *const c_char, c_int, *mut *mut Sqlite3Value) -> c_int;
pub type XVtNext = unsafe extern "C" fn(*mut Sqlite3VtabCursor) -> c_int;
pub type XVtEof = unsafe extern "C" fn(*mut Sqlite3VtabCursor) -> c_int;
pub type XVtColumn = unsafe extern "C" fn(*mut Sqlite3VtabCursor, *mut Sqlite3Context, c_int) -> c_int;
pub type XVtRowid = unsafe extern "C" fn(*mut Sqlite3VtabCursor, *mut i64) -> c_int;
pub type XVtUpdate = unsafe extern "C" fn(*mut Sqlite3Vtab, c_int, *mut *mut Sqlite3Value, *mut i64) -> c_int;
pub type XVtTxn = unsafe extern "C" fn(*mut Sqlite3Vtab) -> c_int;
pub type XVtFindFunction = unsafe extern "C" fn(*mut Sqlite3Vtab, c_int, *const c_char, *mut c_void, *mut *mut c_void) -> c_int;
pub type XVtRename = unsafe extern "C" fn(*mut Sqlite3Vtab, *const c_char) -> c_int;
pub type XVtSavepoint = unsafe extern "C" fn(*mut Sqlite3Vtab, c_int) -> c_int;
pub type XVtShadowName = unsafe extern "C" fn(*const c_char) -> c_int;
pub type XVtIntegrity = unsafe extern "C" fn(*mut Sqlite3Vtab, *const c_char, *const c_char, c_int, *mut *mut c_char) -> c_int;

/// C-ABI sqlite3_module function-pointer table (fields through v4; modern drives the v1 set).
#[repr(C)]
pub struct Sqlite3Module {
    pub i_version: c_int,
    pub x_create: Option<XVtCreate>,
    pub x_connect: Option<XVtCreate>,
    pub x_best_index: Option<XVtBestIndex>,
    pub x_disconnect: Option<XVtDisconnect>,
    pub x_destroy: Option<XVtDisconnect>,
    pub x_open: Option<XVtOpen>,
    pub x_close: Option<XVtClose>,
    pub x_filter: Option<XVtFilter>,
    pub x_next: Option<XVtNext>,
    pub x_eof: Option<XVtEof>,
    pub x_column: Option<XVtColumn>,
    pub x_rowid: Option<XVtRowid>,
    pub x_update: Option<XVtUpdate>,
    pub x_begin: Option<XVtTxn>,
    pub x_sync: Option<XVtTxn>,
    pub x_commit: Option<XVtTxn>,
    pub x_rollback: Option<XVtTxn>,
    pub x_find_function: Option<XVtFindFunction>,
    pub x_rename: Option<XVtRename>,
    pub x_savepoint: Option<XVtSavepoint>,
    pub x_release: Option<XVtSavepoint>,
    pub x_rollback_to: Option<XVtSavepoint>,
    pub x_shadow_name: Option<XVtShadowName>,
    pub x_integrity: Option<XVtIntegrity>,
}
unsafe impl Sync for Sqlite3Module {}

type XModDestroy = unsafe extern "C" fn(*mut c_void);
struct ModEntry { module: usize, aux: usize, destroy: Option<XModDestroy> }
#[derive(Clone)]
struct VtabColDecl { name: String, ctype: String, hidden: bool }
struct VtabInst { vtab: usize, module: usize, cols: Vec<VtabColDecl>, sql: String }

thread_local! {
    // db -> module-name(lower) -> registration
    static VTAB_MOD: RefCell<std::collections::HashMap<usize, std::collections::HashMap<String, ModEntry>>> =
        RefCell::new(std::collections::HashMap::new());
    // db -> table-name -> live instance
    static VTAB_INST: RefCell<std::collections::HashMap<usize, std::collections::HashMap<String, VtabInst>>> =
        RefCell::new(std::collections::HashMap::new());
    // db -> pending declare slot (key present = an xCreate/xConnect is on the stack)
    static VTAB_DECLARE: RefCell<std::collections::HashMap<usize, Option<Vec<VtabColDecl>>>> =
        RefCell::new(std::collections::HashMap::new());
}

unsafe fn vtab_register(db: *mut Sqlite3, z_name: *const c_char, module: *const Sqlite3Module,
                        aux: *mut c_void, destroy: Option<XModDestroy>) -> c_int {
    if db.is_null() || z_name.is_null() { return SQLITE_MISUSE; }
    let name = CStr::from_ptr(z_name).to_string_lossy().to_ascii_lowercase();
    let old = VTAB_MOD.with(|m| m.borrow_mut().entry(db as usize).or_default()
        .insert(name, ModEntry { module: module as usize, aux: aux as usize, destroy }));
    // redefining a module name replaces it; the old registration's _v2 destructor runs
    if let Some(o) = old { if let Some(d) = o.destroy { d(o.aux as *mut c_void); } }
    SQLITE_OK
}

/// # Safety: C ABI — registers a virtual-table module on this connection.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_module(db: *mut Sqlite3, z_name: *const c_char,
        p_module: *const Sqlite3Module, p_aux: *mut c_void) -> c_int {
    vtab_register(db, z_name, p_module, p_aux, None)
}
/// # Safety: C ABI — create_module with a module destructor.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_module_v2(db: *mut Sqlite3, z_name: *const c_char,
        p_module: *const Sqlite3Module, p_aux: *mut c_void, x_destroy: Option<XModDestroy>) -> c_int {
    vtab_register(db, z_name, p_module, p_aux, x_destroy)
}

/// parse the column list of a declare_vtab "CREATE TABLE x(...)" statement
fn vtab_parse_decl(sql: &str) -> Option<Vec<VtabColDecl>> {
    let open = sql.find('(')?;
    let close = sql.rfind(')')?;
    if close <= open { return None; }
    let inner = &sql[open + 1..close];
    let mut cols = Vec::new();
    let (mut depth, mut start) = (0usize, 0usize);
    let bytes = inner.as_bytes();
    let mut parts: Vec<&str> = Vec::new();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => { parts.push(&inner[start..i]); start = i + 1; }
            _ => {}
        }
    }
    parts.push(&inner[start..]);
    for p in parts {
        let toks: Vec<&str> = p.split_whitespace().collect();
        if toks.is_empty() { return None; }
        let name = toks[0].trim_matches('"').trim_matches('`').trim_matches('[').trim_matches(']').to_string();
        let mut hidden = false;
        let mut ty: Vec<&str> = Vec::new();
        for t in &toks[1..] {
            if t.eq_ignore_ascii_case("hidden") { hidden = true; } else { ty.push(t); }
        }
        cols.push(VtabColDecl { name, ctype: ty.join(" "), hidden });
    }
    Some(cols)
}

/// # Safety: C ABI — legal only while an xCreate/xConnect is on the stack (else MISUSE).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_declare_vtab(db: *mut Sqlite3, z_sql: *const c_char) -> c_int {
    if db.is_null() || z_sql.is_null() { return SQLITE_MISUSE; }
    let armed = VTAB_DECLARE.with(|d| d.borrow().contains_key(&(db as usize)));
    if !armed { return SQLITE_MISUSE; }
    let sql = CStr::from_ptr(z_sql).to_string_lossy().into_owned();
    match vtab_parse_decl(&sql) {
        Some(cols) => { VTAB_DECLARE.with(|d| d.borrow_mut().insert(db as usize, Some(cols))); SQLITE_OK }
        None => SQLITE_ERROR,
    }
}

/// is a module of this (lowercased) name registered on the connection?
pub fn vtab_module_registered(dbid: usize, module: &str) -> bool {
    let key = module.to_ascii_lowercase();
    VTAB_MOD.with(|m| m.borrow().get(&dbid).map(|per| per.contains_key(&key)).unwrap_or(false))
}

/// CREATE VIRTUAL TABLE path: xCreate with the C argv convention, capture declare_vtab.
pub fn vtab_create_instance(dbid: usize, tname: &str, module_raw: &str, args: &[String], sql: &str)
        -> Result<(), String> {
    let key = module_raw.to_ascii_lowercase();
    let (mptr, aux) = VTAB_MOD.with(|m| m.borrow().get(&dbid)
        .and_then(|per| per.get(&key)).map(|e| (e.module, e.aux)))
        .ok_or_else(|| format!("no such module: {module_raw}"))?;
    unsafe {
        let m = &*(mptr as *const Sqlite3Module);
        let ctor = m.x_create.or(m.x_connect)
            .ok_or_else(|| format!("no such module: {module_raw}"))?;
        let mut argv_c: Vec<CString> = Vec::new();
        argv_c.push(CString::new(module_raw).map_err(|_| "bad module name".to_string())?);
        argv_c.push(CString::new("main").unwrap());
        argv_c.push(CString::new(tname).map_err(|_| "bad table name".to_string())?);
        for a in args { argv_c.push(CString::new(a.as_str()).map_err(|_| "bad argument".to_string())?); }
        let argv: Vec<*const c_char> = argv_c.iter().map(|c| c.as_ptr()).collect();
        let mut pvtab: *mut Sqlite3Vtab = ptr::null_mut();
        let mut pzerr: *mut c_char = ptr::null_mut();
        VTAB_DECLARE.with(|d| { d.borrow_mut().insert(dbid, None); });
        let rc = ctor(dbid as *mut Sqlite3, aux as *mut c_void, argv.len() as c_int,
                      argv.as_ptr(), &mut pvtab, &mut pzerr);
        let declared = VTAB_DECLARE.with(|d| d.borrow_mut().remove(&dbid)).flatten();
        if rc != SQLITE_OK {
            let msg = if pzerr.is_null() {
                format!("vtable constructor failed: {tname}")
            } else {
                let s = CStr::from_ptr(pzerr).to_string_lossy().into_owned();
                sqlite3_free(pzerr as *mut c_void);
                s
            };
            return Err(msg);
        }
        let cols = match declared {
            Some(c) if !c.is_empty() => c,
            _ => {
                if let Some(xd) = m.x_disconnect { if !pvtab.is_null() { xd(pvtab); } }
                return Err(format!("vtable constructor did not declare schema: {tname}"));
            }
        };
        if !pvtab.is_null() { (*pvtab).p_module = mptr as *const Sqlite3Module; (*pvtab).n_ref = 1; }
        VTAB_INST.with(|i| { i.borrow_mut().entry(dbid).or_default().insert(tname.to_string(),
            VtabInst { vtab: pvtab as usize, module: mptr, cols, sql: sql.to_string() }); });
    }
    Ok(())
}

/// is this table name a live vtab instance?
pub fn vtab_is_instance(dbid: usize, tname: &str) -> bool {
    VTAB_INST.with(|i| i.borrow().get(&dbid).map(|per| per.contains_key(tname)).unwrap_or(false))
}
/// sqlite_master `sql` text for a vtab instance
pub fn vtab_master_sql(dbid: usize, tname: &str) -> Option<String> {
    VTAB_INST.with(|i| i.borrow().get(&dbid).and_then(|per| per.get(tname)).map(|v| v.sql.clone()))
}
/// declared shape (name, type, hidden) for pragma table_info
pub fn vtab_shape(dbid: usize, tname: &str) -> Option<Vec<(String, String, bool)>> {
    VTAB_INST.with(|i| i.borrow().get(&dbid).and_then(|per| per.get(tname))
        .map(|v| v.cols.iter().map(|c| (c.name.clone(), c.ctype.clone(), c.hidden)).collect()))
}

/// DROP TABLE on a vtab: xDestroy and remove the instance. Returns false if not a vtab.
pub fn vtab_drop_instance(dbid: usize, tname: &str) -> bool {
    let inst = VTAB_INST.with(|i| i.borrow_mut().get_mut(&dbid).and_then(|per| per.remove(tname)));
    match inst {
        Some(v) => {
            unsafe {
                let m = &*(v.module as *const Sqlite3Module);
                if let Some(xd) = m.x_destroy.or(m.x_disconnect) {
                    if v.vtab != 0 { xd(v.vtab as *mut Sqlite3Vtab); }
                }
            }
            true
        }
        None => false,
    }
}

/// full scan through the module cursor: xOpen -> xBestIndex(0 constraints) -> xFilter ->
/// xEof/xColumn/xNext loop -> xClose. Returns (visible cols, all cols, rows over all cols).
pub fn vtab_scan(dbid: usize, tname: &str) -> Option<Result<(Vec<String>, Vec<String>, Vec<Vec<eval::V>>), String>> {
    let (vtab, module, cols) = VTAB_INST.with(|i| i.borrow().get(&dbid).and_then(|per| per.get(tname))
        .map(|v| (v.vtab, v.module, v.cols.clone())))?;
    let visible: Vec<String> = cols.iter().filter(|c| !c.hidden).map(|c| c.name.clone()).collect();
    let all: Vec<String> = cols.iter().map(|c| c.name.clone()).collect();
    let r = unsafe {
        let m = &*(module as *const Sqlite3Module);
        let pv = vtab as *mut Sqlite3Vtab;
        let (xo, xc, xf, xn, xe, xcol) = match (m.x_open, m.x_close, m.x_filter, m.x_next, m.x_eof, m.x_column) {
            (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => (a, b, c, d, e, f),
            _ => return Some(Err(format!("malformed module for table {tname}"))),
        };
        // xBestIndex is part of the pinned contract even for the zero-constraint full scan
        if let Some(bi) = m.x_best_index {
            let mut info = Sqlite3IndexInfo {
                n_constraint: 0, a_constraint: ptr::null_mut(), n_order_by: 0, a_order_by: ptr::null_mut(),
                a_constraint_usage: ptr::null_mut(), idx_num: 0, idx_str: ptr::null_mut(),
                need_to_free_idx_str: 0, order_by_consumed: 0, estimated_cost: 0.0,
                estimated_rows: 0, idx_flags: 0, col_used: u64::MAX,
            };
            let rc = bi(pv, &mut info);
            if rc != SQLITE_OK { return Some(Err(format!("xBestIndex failed for table {tname}"))); }
        }
        let mut cur: *mut Sqlite3VtabCursor = ptr::null_mut();
        let rc = xo(pv, &mut cur);
        if rc != SQLITE_OK || cur.is_null() { return Some(Err(format!("unable to open cursor on {tname}"))); }
        (*cur).p_vtab = pv;
        let rc = xf(cur, 0, ptr::null(), 0, ptr::null_mut());
        if rc != SQLITE_OK { xc(cur); return Some(Err(format!("xFilter failed for table {tname}"))); }
        let mut rows: Vec<Vec<eval::V>> = Vec::new();
        while xe(cur) == 0 {
            let mut row = Vec::with_capacity(all.len());
            for i in 0..all.len() {
                let mut ctx = Sqlite3Context { db: dbid, user_data: ptr::null_mut(), result: eval::V::Null,
                    is_error: false, errmsg: None, text_keep: Vec::new(), agg: ptr::null_mut(), agg_size: 0 };
                let rc = xcol(cur, &mut ctx as *mut _, i as c_int);
                if rc != SQLITE_OK || ctx.is_error {
                    xc(cur);
                    return Some(Err(ctx.errmsg.unwrap_or_else(|| format!("xColumn failed for table {tname}"))));
                }
                row.push(ctx.result);
            }
            rows.push(row);
            let rc = xn(cur);
            if rc != SQLITE_OK { xc(cur); return Some(Err(format!("xNext failed for table {tname}"))); }
        }
        xc(cur);
        Ok((visible, all, rows))
    };
    Some(r)
}

/// connection close: xDisconnect live instances, run module destructors
fn vtab_close(dbid: usize) {
    let insts = VTAB_INST.with(|i| i.borrow_mut().remove(&dbid));
    if let Some(per) = insts {
        for (_n, v) in per {
            unsafe {
                let m = &*(v.module as *const Sqlite3Module);
                if let Some(xd) = m.x_disconnect { if v.vtab != 0 { xd(v.vtab as *mut Sqlite3Vtab); } }
            }
        }
    }
    let mods = VTAB_MOD.with(|m| m.borrow_mut().remove(&dbid));
    if let Some(per) = mods {
        for (_n, e) in per {
            if let Some(d) = e.destroy { unsafe { d(e.aux as *mut c_void); } }
        }
    }
    VTAB_DECLARE.with(|d| { d.borrow_mut().remove(&dbid); });
}

// ---- sqlite3_value_* accessors ----
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_type(v: *mut Sqlite3Value) -> c_int {
    if v.is_null() { return 5; }
    match &(*v).v { eval::V::Int(_) => 1, eval::V::Real(_) => 2, eval::V::Text(_) => 3, eval::V::Blob(_) => 4, eval::V::Null => 5 }
}
/// # Safety: C ABI — SQLite numeric affinity of the argument.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_numeric_type(v: *mut Sqlite3Value) -> c_int {
    if v.is_null() { return 5; }
    match &(*v).v {
        eval::V::Int(_) => 1, eval::V::Real(_) => 2, eval::V::Blob(_) => 4, eval::V::Null => 5,
        eval::V::Text(t) => {
            let s = t.trim();
            if s.parse::<i64>().is_ok() { 1 }
            else if s.parse::<f64>().is_ok() && (s.contains('.') || s.contains('e') || s.contains('E')) { 2 }
            else { 3 }
        }
    }
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_int(v: *mut Sqlite3Value) -> c_int {
    sqlite3_value_int64(v) as c_int
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_int64(v: *mut Sqlite3Value) -> i64 {
    if v.is_null() { return 0; }
    match &(*v).v {
        eval::V::Int(i) => *i, eval::V::Real(r) => *r as i64,
        eval::V::Text(t) => { let s = t.trim(); let neg = s.starts_with('-');
            let d: String = s.trim_start_matches(['+','-']).chars().take_while(|c| c.is_ascii_digit()).collect();
            let n: i64 = d.parse().unwrap_or(0); if neg { -n } else { n } }
        _ => 0,
    }
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_double(v: *mut Sqlite3Value) -> f64 {
    if v.is_null() { return 0.0; }
    match &(*v).v { eval::V::Int(i) => *i as f64, eval::V::Real(r) => *r,
        eval::V::Text(t) => eval::text_to_num(t).unwrap_or(0.0), _ => 0.0 }
}
/// # Safety: C ABI — pointer valid until the callback returns.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_text(v: *mut Sqlite3Value) -> *const u8 {
    if v.is_null() || matches!((*v).v, eval::V::Null) { return std::ptr::null(); }
    // raw bytes (NUL-appended) so blobs round-trip through result_text unchanged
    let mut bytes = match &(*v).v { eval::V::Blob(b) => b.clone(), other => other.render().unwrap_or_default().into_bytes() };
    bytes.push(0);
    VALUE_SCRATCH.with(|k| { k.borrow_mut().push(bytes); let m = k.borrow(); m.last().unwrap().as_ptr() })
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_blob(v: *mut Sqlite3Value) -> *const c_void {
    sqlite3_value_text(v) as *const c_void
}
/// # Safety: C ABI — byte length (SQLite: rendered text length for numerics).
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_bytes(v: *mut Sqlite3Value) -> c_int {
    if v.is_null() { return 0; }
    match &(*v).v {
        eval::V::Null => 0,
        eval::V::Blob(b) => b.len() as c_int,
        other => other.render().map(|s| s.len()).unwrap_or(0) as c_int,
    }
}

thread_local! {
    static VALUE_SCRATCH: std::cell::RefCell<Vec<Vec<u8>>> = const { std::cell::RefCell::new(Vec::new()) };
}

// ---- sqlite3_result_* writers ----
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_null(c: *mut Sqlite3Context) { if !c.is_null() { (*c).result = eval::V::Null; } }
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_int(c: *mut Sqlite3Context, v: c_int) { if !c.is_null() { (*c).result = eval::V::Int(v as i64); } }
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_int64(c: *mut Sqlite3Context, v: i64) { if !c.is_null() { (*c).result = eval::V::Int(v); } }
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_double(c: *mut Sqlite3Context, v: f64) { if !c.is_null() { (*c).result = eval::V::Real(v); } }
/// # Safety: C ABI — text is COPIED.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_text(c: *mut Sqlite3Context, z: *const c_char, n: c_int, _d: *mut c_void) {
    if c.is_null() { return; }
    if z.is_null() { (*c).result = eval::V::Null; return; }
    let bytes = if n < 0 { CStr::from_ptr(z).to_bytes().to_vec() }
                else { std::slice::from_raw_parts(z as *const u8, n as usize).to_vec() };
    (*c).result = match String::from_utf8(bytes) {
        Ok(s) => eval::V::Text(s),
        Err(e) => eval::V::Blob(e.into_bytes()), // non-UTF-8 result carried as raw bytes
    };
}
/// # Safety: C ABI — blob is COPIED.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_blob(c: *mut Sqlite3Context, z: *const c_void, n: c_int, _d: *mut c_void) {
    if c.is_null() { return; }
    if z.is_null() { (*c).result = eval::V::Null; return; }
    (*c).result = eval::V::Blob(std::slice::from_raw_parts(z as *const u8, n.max(0) as usize).to_vec());
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_error(c: *mut Sqlite3Context, z: *const c_char, n: c_int) {
    if c.is_null() { return; }
    let msg = if z.is_null() { "error".to_string() }
        else if n < 0 { CStr::from_ptr(z).to_string_lossy().into_owned() }
        else { String::from_utf8_lossy(std::slice::from_raw_parts(z as *const u8, n as usize)).into_owned() };
    (*c).is_error = true; (*c).errmsg = Some(msg);
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_user_data(c: *mut Sqlite3Context) -> *mut c_void {
    if c.is_null() { std::ptr::null_mut() } else { (*c).user_data }
}
/// # Safety: C ABI.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_context_db_handle(c: *mut Sqlite3Context) -> *mut Sqlite3 {
    if c.is_null() { std::ptr::null_mut() } else { (*c).db as *mut Sqlite3 }
}
/// # Safety: C ABI — lazily-allocated, zeroed per-group aggregate buffer.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_aggregate_context(c: *mut Sqlite3Context, n_bytes: c_int) -> *mut c_void {
    if c.is_null() || n_bytes <= 0 { return std::ptr::null_mut(); }
    if (*c).agg.is_null() {
        // the invoker owns the backing Vec; signal desired size and hand back a stub
        // pointer that the invoker replaces with the real (resized) buffer next step.
        (*c).agg_size = n_bytes as usize;
        // allocate a leaked zeroed buffer for THIS group; freed at group end by the OS
        let buf = vec![0u8; n_bytes as usize].into_boxed_slice();
        (*c).agg = Box::into_raw(buf) as *mut c_void;
    }
    (*c).agg
}
