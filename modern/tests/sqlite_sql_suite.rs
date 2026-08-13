//! run-54: SQL-only regression pack against the kitchen's sqlite3_exec C ABI.
//!
//! Cases are the vendored extract of the upstream SQLite TCL suite
//! (do_execsql_test / do_catchsql_test / SQL-only do_test) at
//! sqlite/sqlite@2665df90e7bbdcffa59102204090f7c9ede04cc4, VERSION 3.54.0.
//! This is NOT a migration claim and fills NO behavioural pins — it replays
//! extracted SQL through the same C ABI the rest of the kitchen tests use.
//!
//! Pack location: `<repo>/tests/sqlite-sql-suite/cases.jsonl`, resolved as
//! `CARGO_MANIFEST_DIR/../tests/sqlite-sql-suite/`.
//!
//! Default scope is the run-54 first slice (values/unique/coalesce/upsert1/null).
//! Override with `SQLITE_SQL_SUITE_SLICE=a.test,b.test` (comma list) to run
//! other files. `known-fail.txt` lists ids that are expected to mismatch; those
//! still run and are reported as known-fail (a listed case that now passes is an
//! XPASS and is printed). A mismatch NOT in known-fail is a harness failure.

use sqlite3_rust_spine::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

// ---- one exec: collect every cell of every row in order (NULL -> None) ----
thread_local! {
    static CELLS: RefCell<Vec<Option<String>>> = const { RefCell::new(Vec::new()) };
}
unsafe extern "C" fn collect_cb(_arg: *mut c_void, ncol: c_int, argv: *mut *mut c_char, _cols: *mut *mut c_char) -> c_int {
    CELLS.with(|c| {
        let mut v = c.borrow_mut();
        for i in 0..ncol as isize {
            let p = *argv.offset(i);
            if p.is_null() { v.push(None); }
            else { v.push(Some(CStr::from_ptr(p).to_string_lossy().into_owned())); }
        }
    });
    0
}

struct ExecOut { rc: i32, errmsg: String, cells: Vec<Option<String>> }

unsafe fn run_exec(db: *mut Sqlite3, sql: &str) -> ExecOut {
    CELLS.with(|c| c.borrow_mut().clear());
    let csql = match CString::new(sql) { Ok(s) => s, Err(_) => {
        return ExecOut { rc: 1, errmsg: "embedded NUL in SQL".into(), cells: Vec::new() };
    } };
    let mut em: *mut c_char = ptr::null_mut();
    let rc = sqlite3_exec(db, csql.as_ptr(), Some(collect_cb), ptr::null_mut(), &mut em);
    let errmsg = if em.is_null() { String::new() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
    let cells = CELLS.with(|c| c.borrow_mut().drain(..).collect());
    ExecOut { rc, errmsg, cells }
}

// ---- Tcl list flattening (matches `$db eval` -> `[list {*}$result]`) ----
/// quote one element the way Tcl's Tcl_ConvertElement does for the common
/// (no-backslash-needed) case: empty -> {}, whitespace/brace-bearing with
/// balanced braces -> {s}, otherwise the bare string.
fn tcl_elem(s: &str) -> String {
    if s.is_empty() { return "{}".into(); }
    let needs = s.bytes().any(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b'{' | b'}' | b'"' | b'\\' | b'[' | b']' | b'$' | b';'));
    if !needs { return s.to_string(); }
    // if braces are balanced and no trailing backslash, Tcl braces the element
    let mut depth = 0i32;
    let mut ok = true;
    for b in s.bytes() {
        match b { b'{' => depth += 1, b'}' => { depth -= 1; if depth < 0 { ok = false; break; } }, _ => {} }
    }
    if ok && depth == 0 && !s.ends_with('\\') { format!("{{{s}}}") }
    else {
        // backslash-escape form (rare in this slice); good enough + honest fallback
        let mut out = String::new();
        for c in s.chars() {
            if matches!(c, ' ' | '\t' | '\n' | '{' | '}' | '"' | '\\' | '[' | ']' | '$' | ';') { out.push('\\'); }
            out.push(c);
        }
        out
    }
}
fn flatten(cells: &[Option<String>]) -> String {
    cells.iter()
        .map(|c| match c { Some(s) => tcl_elem(s), None => "{}".into() })
        .collect::<Vec<_>>()
        .join(" ")
}

/// parse a catchsql expected `{rc errmsg}` list: first bare token is the rc,
/// the remainder (possibly brace-wrapped) is the message.
fn parse_catch_expected(exp: &str) -> (i32, String) {
    let t = exp.trim();
    let (head, rest) = match t.split_once(char::is_whitespace) {
        Some((a, b)) => (a, b.trim()),
        None => (t, ""),
    };
    let rc: i32 = head.parse().unwrap_or(-999);
    let msg = if rest.starts_with('{') && rest.ends_with('}') && rest.len() >= 2 {
        rest[1..rest.len() - 1].to_string()
    } else { rest.to_string() };
    (rc, msg)
}

struct Case {
    id: String,
    source_file: String,
    kind: String,
    sql: String,
    expected: String,
    role: String,
}

// minimal JSONL-object parser (no external deps — pack v1 bans them). Each line
// is a flat object of string/number fields; we only need the named strings.
fn parse_case(line: &str) -> Case {
    let b: Vec<char> = line.chars().collect();
    let mut i = 0usize;
    let mut fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    // read a JSON string starting at b[i]=='"' -> (value, next index)
    fn read_str(b: &[char], mut i: usize) -> (String, usize) {
        debug_assert!(b[i] == '"');
        i += 1;
        let mut s = String::new();
        while i < b.len() {
            match b[i] {
                '"' => { i += 1; break; }
                '\\' => {
                    i += 1;
                    match b.get(i) {
                        Some('n') => s.push('\n'), Some('t') => s.push('\t'),
                        Some('r') => s.push('\r'), Some('"') => s.push('"'),
                        Some('\\') => s.push('\\'), Some('/') => s.push('/'),
                        Some('b') => s.push('\u{8}'), Some('f') => s.push('\u{c}'),
                        Some('u') => {
                            let hex: String = b[i + 1..(i + 5).min(b.len())].iter().collect();
                            if let Ok(cp) = u32::from_str_radix(&hex, 16) {
                                if let Some(ch) = char::from_u32(cp) { s.push(ch); }
                            }
                            i += 4;
                        }
                        Some(c) => s.push(*c),
                        None => {}
                    }
                    i += 1;
                }
                c => { s.push(c); i += 1; }
            }
        }
        (s, i)
    }
    // skip to first '{'
    while i < b.len() && b[i] != '{' { i += 1; }
    i += 1;
    loop {
        while i < b.len() && (b[i].is_whitespace() || b[i] == ',') { i += 1; }
        if i >= b.len() || b[i] == '}' { break; }
        let (key, ni) = read_str(&b, i);
        i = ni;
        while i < b.len() && (b[i].is_whitespace() || b[i] == ':') { i += 1; }
        if i >= b.len() { break; }
        if b[i] == '"' {
            let (val, ni) = read_str(&b, i);
            i = ni;
            fields.insert(key, val);
        } else {
            // number / literal — read until , or }
            let st = i;
            while i < b.len() && b[i] != ',' && b[i] != '}' { i += 1; }
            fields.insert(key, b[st..i].iter().collect::<String>().trim().to_string());
        }
    }
    let g = |k: &str| fields.get(k).cloned().unwrap_or_default();
    Case { id: g("id"), source_file: g("source_file"), kind: g("kind"),
           sql: g("sql"), expected: g("expected"), role: g("role") }
}

fn suite_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.push("tests/sqlite-sql-suite");
    p
}

fn load_known_fail() -> std::collections::HashSet<String> {
    let p = suite_dir().join("known-fail.txt");
    let mut set = std::collections::HashSet::new();
    if let Ok(txt) = std::fs::read_to_string(&p) {
        for line in txt.lines() {
            let t = line.trim();
            // a full-line comment starts with '#'; ids may themselves contain '#'
            // (e.g. values-3.1.2#2), so the id is the FIRST whitespace token and any
            // trailing "  # reason" is dropped only when preceded by whitespace.
            if t.is_empty() || t.starts_with('#') { continue; }
            let id = t.split_whitespace().next().unwrap_or("");
            if !id.is_empty() { set.insert(id.to_string()); }
        }
    }
    set
}

const DEFAULT_SLICE: &[&str] = &["values.test", "unique.test", "coalesce.test", "upsert1.test", "null.test"];

#[test]
fn sqlite_sql_suite() {
    let cases_path = suite_dir().join("cases.jsonl");
    let raw = std::fs::read_to_string(&cases_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", cases_path.display()));
    let all: Vec<Case> = raw.lines().filter(|l| !l.trim().is_empty())
        .map(parse_case)
        .collect();

    // scope: env SQLITE_SQL_SUITE_SLICE overrides the default first slice
    let slice: Vec<String> = match std::env::var("SQLITE_SQL_SUITE_SLICE") {
        Ok(v) => v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        Err(_) => DEFAULT_SLICE.iter().map(|s| s.to_string()).collect(),
    };
    let known = load_known_fail();

    // group by source_file, preserving JSONL order
    let mut by_file: Vec<(String, Vec<&Case>)> = Vec::new();
    let mut idx: BTreeMap<String, usize> = BTreeMap::new();
    for c in &all {
        if !slice.contains(&c.source_file) { continue; }
        match idx.get(&c.source_file) {
            Some(&i) => by_file[i].1.push(c),
            None => { idx.insert(c.source_file.clone(), by_file.len()); by_file.push((c.source_file.clone(), vec![c])); }
        }
    }

    let (mut tot_pass, mut tot_fail, mut tot_known, mut tot_xpass, mut tot_setuperr) = (0, 0, 0, 0, 0);
    let mut unexpected: Vec<String> = Vec::new();
    let mut xpass_ids: Vec<String> = Vec::new();

    for (file, cases) in &by_file {
        let db = unsafe {
            let mut db: *mut Sqlite3 = ptr::null_mut();
            sqlite3_open(c":memory:".as_ptr(), &mut db);
            db
        };
        let (mut p, mut f, mut kf, mut xp, mut se) = (0, 0, 0, 0, 0);
        for c in cases {
            let out = unsafe { run_exec(db, &c.sql) };
            if c.role == "setup" {
                if out.rc != 0 {
                    se += 1; tot_setuperr += 1;
                    let msg = format!("{} SETUP-ERROR rc={} {}", c.id, out.rc, out.errmsg);
                    if !known.contains(&c.id) { unexpected.push(msg); }
                }
                continue;
            }
            let matched = match c.kind.as_str() {
                "execsql" => out.rc == 0 && flatten(&out.cells) == c.expected,
                "catchsql" => {
                    let (erc, emsg) = parse_catch_expected(&c.expected);
                    let got_rc = if out.rc == 0 { 0 } else { 1 };
                    // compare rc always; compare errmsg on failures (ADR 0042 records
                    // that wording matched in bulk for this slice — no rc-only fallback needed)
                    got_rc == erc && (erc == 0 || out.errmsg == emsg)
                }
                other => panic!("unknown kind {other} for case {}", c.id),
            };
            let listed = known.contains(&c.id);
            match (matched, listed) {
                (true, false) => { p += 1; tot_pass += 1; }
                (true, true) => { xp += 1; tot_xpass += 1; xpass_ids.push(c.id.clone()); }
                (false, true) => { kf += 1; tot_known += 1; }
                (false, false) => {
                    f += 1; tot_fail += 1;
                    let got = match c.kind.as_str() {
                        "catchsql" => format!("rc={} msg={:?}", out.rc, out.errmsg),
                        _ => format!("rc={} rows={:?}", out.rc, flatten(&out.cells)),
                    };
                    unexpected.push(format!("{} [{}] expected={:?} got={}", c.id, c.kind, c.expected, got));
                }
            }
        }
        unsafe { sqlite3_close(db); }
        println!("  {file:<16} pass={p} fail={f} known-fail={kf} xpass={xp} setup-err={se} (n={})", cases.len());
    }

    println!("\n== sqlite-sql-suite totals ==");
    println!("pass={tot_pass} fail={tot_fail} known-fail={tot_known} xpass={tot_xpass} setup-err={tot_setuperr}");
    println!("git ref 2665df90 / VERSION 3.54.0 — SQL-only replay, NOT a migration claim");
    if !xpass_ids.is_empty() {
        println!("\nXPASS (listed in known-fail but now passing — prune known-fail.txt):");
        for id in &xpass_ids { println!("  XPASS {id}"); }
    }
    if !unexpected.is_empty() {
        println!("\nUNEXPECTED failures (not in known-fail.txt):");
        for m in &unexpected { println!("  {m}"); }
    }

    assert!(tot_pass >= 50, "first-slice green bar not met: only {tot_pass} passed (need >= 50)");
    assert!(unexpected.is_empty(), "{} unexpected failure(s) — list them in known-fail.txt or fix", unexpected.len());
}
