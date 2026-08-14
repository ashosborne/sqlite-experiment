//! run-58: the first bytecode slice. Constant SELECTs compile to a real Vdbe
//! program whose EXPLAIN listing matches probed C row-for-row, and sqlite3_step
//! of that SQL executes the program through a dispatch loop. Everything else
//! stays on the kitchen evaluator (FROM, joins, DML, CTEs — honestly named).
//!
//! Opcodes implemented (exactly the ones C's programs for the pinned SQL use):
//! Init, Goto, Integer, String8, Add, ResultRow, Halt. No 199-opcode census, no
//! table-scan opcodes, no OP_Program/interrupt — those stay in the residual.

use crate::eval::V;
use crate::store::Val;
use std::sync::atomic::{AtomicU64, Ordering};

static DISPATCH: AtomicU64 = AtomicU64::new(0);
/// executed-opcode counter — the anti-cheat proof that step really dispatches.
pub fn dispatch_count() -> u64 { DISPATCH.load(Ordering::SeqCst) }

static CURSOR_READS: AtomicU64 = AtomicU64::new(0);
/// run-59: cells the scan cursor positioned on (Rewind/Next landing on a row).
/// Moves on SELECT col FROM t; does NOT move on constant SELECT 1 (no cursor)
/// or on kitchen-fallback SQL (joins etc. never enter the loop).
pub fn cursor_read_count() -> u64 { CURSOR_READS.load(Ordering::SeqCst) }

#[derive(Clone, Debug)]
pub struct Op {
    pub opcode: &'static str,
    pub p1: i64,
    pub p2: i64,
    pub p3: i64,
    pub p4: Option<String>,
    pub p5: i64,
}
fn op_(opcode: &'static str, p1: i64, p2: i64, p3: i64) -> Op {
    Op { opcode, p1, p2, p3, p4: None, p5: 0 }
}

/// compile a CONSTANT SELECT into C's program shape. None = not this slice's
/// scope (the kitchen keeps it). Patterns are exactly the probed ones:
///   SELECT <int>                      -> Init,Integer,ResultRow,Halt,Goto
///   SELECT <int> WHERE 1              -> same (C folds the true WHERE)
///   SELECT <int> WHERE 0              -> Init,Goto(->Halt),Integer,ResultRow,Halt,Goto
///   SELECT <int>+<int>                -> Add with init-section register loads
///   SELECT '<text>'                   -> String8
///   SELECT <int>, <int>               -> two Integers, ResultRow(1,2)
pub fn compile(sql: &str) -> Option<Vec<Op>> {
    let s = sql.trim().trim_end_matches(';').trim();
    let up = s.to_ascii_uppercase();
    if !up.starts_with("SELECT") { return None; }
    let body = s["SELECT".len()..].trim();
    if body.is_empty() { return None; }

    // optional constant WHERE tail
    let (items_txt, wh) = match body.to_ascii_uppercase().find(" WHERE ") {
        Some(p) => {
            let w = body[p + 7..].trim();
            let flag = match w { "1" => Some(true), "0" => Some(false), _ => None }?;
            (body[..p].trim(), Some(flag))
        }
        None => (body, None),
    };

    let parse_int = |t: &str| -> Option<i64> {
        let t = t.trim();
        if !t.is_empty() && t.chars().all(|c| c.is_ascii_digit()) { t.parse().ok() } else { None }
    };
    let parse_text = |t: &str| -> Option<String> {
        let t = t.trim();
        if t.len() >= 2 && t.starts_with('\'') && t.ends_with('\'') && !t[1..t.len() - 1].contains('\'') {
            Some(t[1..t.len() - 1].to_string())
        } else { None }
    };

    // SELECT <int>, <int>
    if items_txt.contains(',') && wh.is_none() {
        let parts: Vec<&str> = items_txt.split(',').collect();
        if parts.len() == 2 {
            let a = parse_int(parts[0])?;
            let b = parse_int(parts[1])?;
            return Some(vec![
                op_("Init", 0, 5, 0),
                op_("Integer", a, 1, 0),
                op_("Integer", b, 2, 0),
                op_("ResultRow", 1, 2, 0),
                op_("Halt", 0, 0, 0),
                op_("Goto", 0, 1, 0),
            ]);
        }
        return None;
    }
    // SELECT <int>+<int> (C does NOT constant-fold: Add + init-section loads)
    if let Some(pp) = items_txt.find('+') {
        if wh.is_none() {
            let a = parse_int(&items_txt[..pp])?;
            let b = parse_int(&items_txt[pp + 1..])?;
            return Some(vec![
                op_("Init", 0, 4, 0),
                op_("Add", 3, 2, 1),
                op_("ResultRow", 1, 1, 0),
                op_("Halt", 0, 0, 0),
                op_("Integer", a, 2, 0),
                op_("Integer", b, 3, 0),
                op_("Goto", 0, 1, 0),
            ]);
        }
        return None;
    }
    // SELECT '<text>'
    if let Some(t) = parse_text(items_txt) {
        if wh.is_none() {
            let mut s8 = op_("String8", 0, 1, 0);
            s8.p4 = Some(t);
            return Some(vec![
                op_("Init", 0, 4, 0),
                s8,
                op_("ResultRow", 1, 1, 0),
                op_("Halt", 0, 0, 0),
                op_("Goto", 0, 1, 0),
            ]);
        }
        return None;
    }
    // SELECT <int> [WHERE 1|0]
    let n = parse_int(items_txt)?;
    match wh {
        None | Some(true) => Some(vec![
            op_("Init", 0, 4, 0),
            op_("Integer", n, 1, 0),
            op_("ResultRow", 1, 1, 0),
            op_("Halt", 0, 0, 0),
            op_("Goto", 0, 1, 0),
        ]),
        Some(false) => Some(vec![
            op_("Init", 0, 5, 0),
            op_("Goto", 0, 4, 0),
            op_("Integer", n, 1, 0),
            op_("ResultRow", 1, 1, 0),
            op_("Halt", 0, 0, 0),
            op_("Goto", 0, 1, 0),
        ]),
    }
}

/// run-59: parse the ONE table-scan shape this slice owns:
///   SELECT <col>(, <col>)* FROM <table>
/// plain identifiers only — no WHERE / join / alias / expression / qualifier.
/// Returns (table, columns). None => not ours (kitchen or v47 constant path).
pub fn parse_scan(sql: &str) -> Option<(String, Vec<String>)> {
    let s = sql.trim().trim_end_matches(';').trim();
    let up = s.to_ascii_uppercase();
    if !up.starts_with("SELECT ") { return None; }
    let fp = up.find(" FROM ")?;
    let cols_txt = &s[7..fp];
    let tbl = s[fp + 6..].trim();
    let is_ident = |t: &str| -> bool {
        !t.is_empty()
            && t.chars().next().map_or(false, |c| c.is_ascii_alphabetic() || c == '_')
            && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    };
    if !is_ident(tbl) { return None; }
    let mut cols = Vec::new();
    for c in cols_txt.split(',') {
        let c = c.trim();
        if !is_ident(c) { return None; }
        // bare keywords that would change meaning are not column names here
        if ["DISTINCT", "ALL"].contains(&c.to_ascii_uppercase().as_str()) { return None; }
        cols.push(c.to_string());
    }
    if cols.is_empty() { return None; }
    Some((tbl.to_string(), cols))
}

/// run-60: the WHERE shape this slice owns:
///   SELECT <col>(, <col>)* FROM <table> WHERE <col|rowid> <op> (<int-lit> | ?)
/// ops: = <> != > < >= <=. Returns (table, select cols, where target, op, rhs).
#[derive(Clone, Debug, PartialEq)]
pub enum WhereRhs { Lit(i64), Var(u32) }
#[derive(Clone, Debug, PartialEq)]
pub enum WhereCol { Named(String), Rowid }
pub fn parse_where_scan(sql: &str) -> Option<(String, Vec<String>, WhereCol, &'static str, WhereRhs)> {
    let s = sql.trim().trim_end_matches(';').trim();
    let up = s.to_ascii_uppercase();
    let wp = up.find(" WHERE ")?;
    let (head, tail) = (&s[..wp], s[wp + 7..].trim());
    let (tbl, cols) = parse_scan(head)?;
    // split the predicate: <ident> <op> <rhs> — longest ops first
    let mut found: Option<(usize, &'static str)> = None;
    for op in ["<>", "!=", ">=", "<=", "=", ">", "<"] {
        if let Some(p) = tail.find(op) { found = Some((p, op)); break; }
    }
    let (p, op) = found?;
    let lhs = tail[..p].trim();
    let rhs_txt = tail[p + op.len()..].trim();
    let is_ident = |t: &str| -> bool {
        !t.is_empty()
            && t.chars().next().map_or(false, |c| c.is_ascii_alphabetic() || c == '_')
            && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    };
    if !is_ident(lhs) { return None; }
    let target = if lhs.eq_ignore_ascii_case("rowid") { WhereCol::Rowid } else { WhereCol::Named(lhs.to_string()) };
    let rhs = if rhs_txt == "?" {
        WhereRhs::Var(1)
    } else if !rhs_txt.is_empty() && rhs_txt.chars().all(|c| c.is_ascii_digit()) {
        WhereRhs::Lit(rhs_txt.parse().ok()?)
    } else {
        return None;
    };
    // rowid predicates: only the probed equality seek shape
    if target == WhereCol::Rowid && op != "=" { return None; }
    Some((tbl, cols, target, op, rhs))
}

/// C INVERTS the WHERE test into a jump-to-Next compare (probed on the pin):
///   =  -> Ne,  <>/!= -> Eq,  >  -> Le,  <  -> Ge,  >= -> Lt,  <= -> Gt
fn inverted_cmp(op: &str) -> &'static str {
    match op {
        "=" => "Ne", "<>" | "!=" => "Eq", ">" => "Le", "<" => "Ge", ">=" => "Lt", "<=" => "Gt",
        _ => unreachable!("unprobed op {op}"),
    }
}

/// C's WHERE-compare scan for k result columns (probed):
///   Init 0 8+k | OpenRead 0 root 0 p4=hint | Rewind 0 7+k | Column 0 wcol 1 |
///   <InvCmp> 2 6+k 1 p4=BINARY-8 p5=84 | Column x k (-> r3..) | ResultRow 3 k |
///   Next 0 3 p5=1 | Halt | Transaction 0 0 cookie p4=0 p5=1 |
///   Integer lit 2 (or Variable n 2) | Goto 0 1
pub fn compile_where_scan(root: u32, cookie: u32, sel_idx: &[usize], wcol: usize,
                          op: &str, rhs: &WhereRhs) -> Vec<Op> {
    let k = sel_idx.len() as i64;
    let hint = sel_idx.iter().copied().chain(std::iter::once(wcol)).max().unwrap_or(0) as i64 + 1;
    let mut prog = Vec::with_capacity(12 + sel_idx.len());
    prog.push(op_("Init", 0, 8 + k, 0));
    let mut openread = op_("OpenRead", 0, root as i64, 0);
    openread.p4 = Some(hint.to_string());
    prog.push(openread);
    prog.push(op_("Rewind", 0, 7 + k, 0));
    prog.push(op_("Column", 0, wcol as i64, 1));
    let mut cmp = op_(inverted_cmp(op), 2, 6 + k, 1);
    cmp.p4 = Some("BINARY-8".to_string());
    cmp.p5 = 84;
    prog.push(cmp);
    for (i, c) in sel_idx.iter().enumerate() {
        prog.push(op_("Column", 0, *c as i64, i as i64 + 3));
    }
    prog.push(op_("ResultRow", 3, k, 0));
    let mut next = op_("Next", 0, 3, 0);
    next.p5 = 1;
    prog.push(next);
    prog.push(op_("Halt", 0, 0, 0));
    let mut txn = op_("Transaction", 0, 0, cookie as i64);
    txn.p4 = Some("0".to_string());
    txn.p5 = 1;
    prog.push(txn);
    prog.push(match rhs {
        WhereRhs::Lit(n) => op_("Integer", *n, 2, 0),
        WhereRhs::Var(n) => op_("Variable", *n as i64, 2, 0),
    });
    prog.push(op_("Goto", 0, 1, 0));
    prog
}

/// C's rowid-equality seek for k result columns (probed — no Rewind/Next loop):
///   Init 0 6+k | OpenRead 0 root 0 p4=hint | Integer lit 1 | SeekRowid 0 5+k 1 |
///   Column x k (-> r2..) | ResultRow 2 k | Halt | Transaction | Goto 0 1
pub fn compile_seek_rowid(root: u32, cookie: u32, sel_idx: &[usize], rhs: &WhereRhs) -> Vec<Op> {
    let k = sel_idx.len() as i64;
    let hint = sel_idx.iter().copied().max().unwrap_or(0) as i64 + 1;
    let mut prog = Vec::with_capacity(10 + sel_idx.len());
    prog.push(op_("Init", 0, 6 + k, 0));
    let mut openread = op_("OpenRead", 0, root as i64, 0);
    openread.p4 = Some(hint.to_string());
    prog.push(openread);
    prog.push(match rhs {
        WhereRhs::Lit(n) => op_("Integer", *n, 1, 0),
        WhereRhs::Var(n) => op_("Variable", *n as i64, 1, 0),
    });
    prog.push(op_("SeekRowid", 0, 5 + k, 1));
    for (i, c) in sel_idx.iter().enumerate() {
        prog.push(op_("Column", 0, *c as i64, i as i64 + 2));
    }
    prog.push(op_("ResultRow", 2, k, 0));
    prog.push(op_("Halt", 0, 0, 0));
    let mut txn = op_("Transaction", 0, 0, cookie as i64);
    txn.p4 = Some("0".to_string());
    txn.p5 = 1;
    prog.push(txn);
    prog.push(op_("Goto", 0, 1, 0));
    prog
}

/// run-61: the single-row INSERT shape this slice owns:
///   INSERT INTO <table>[(<col>, ...)] VALUES(<int-lit>|'<text>'|?, ...)
#[derive(Clone, Debug)]
pub enum InsVal { Int(i64), Text(String), Var(u32) }
pub fn parse_insert(sql: &str) -> Option<(String, Option<Vec<String>>, Vec<InsVal>)> {
    let s = sql.trim().trim_end_matches(';').trim();
    let up = s.to_ascii_uppercase();
    if !up.starts_with("INSERT INTO ") { return None; }
    let rest = s["INSERT INTO ".len()..].trim();
    let vp = rest.to_ascii_uppercase().find("VALUES")?;
    let head = rest[..vp].trim();
    let tail = rest[vp + 6..].trim();
    let is_ident = |t: &str| -> bool {
        !t.is_empty()
            && t.chars().next().map_or(false, |c| c.is_ascii_alphabetic() || c == '_')
            && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    };
    // head = table [(cols)]
    let (tbl, cols) = match head.find('(') {
        Some(p) => {
            if !head.ends_with(')') { return None; }
            let t = head[..p].trim();
            let cl: Vec<String> = head[p + 1..head.len() - 1].split(',').map(|c| c.trim().to_string()).collect();
            if cl.is_empty() || cl.iter().any(|c| !is_ident(c)) { return None; }
            (t, Some(cl))
        }
        None => (head, None),
    };
    if !is_ident(tbl) { return None; }
    // tail = (v, v, ...) — one row only
    if !tail.starts_with('(') || !tail.ends_with(')') { return None; }
    let inner = &tail[1..tail.len() - 1];
    let mut vals = Vec::new();
    let mut var_no = 0u32;
    let (mut cur, mut inq) = (String::new(), false);
    let mut parts: Vec<String> = Vec::new();
    for ch in inner.chars() {
        match ch {
            '\'' => { inq = !inq; cur.push(ch); }
            ',' if !inq => { parts.push(cur.trim().to_string()); cur.clear(); }
            _ => cur.push(ch),
        }
    }
    if inq { return None; }
    if !cur.trim().is_empty() { parts.push(cur.trim().to_string()); }
    for p in parts {
        if p == "?" { var_no += 1; vals.push(InsVal::Var(var_no)); }
        else if !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()) { vals.push(InsVal::Int(p.parse().ok()?)); }
        else if p.len() >= 2 && p.starts_with('\'') && p.ends_with('\'') && !p[1..p.len() - 1].contains('\'') {
            vals.push(InsVal::Text(p[1..p.len() - 1].to_string()));
        } else { return None; }
    }
    if vals.is_empty() { return None; }
    Some((tbl.to_string(), cols, vals))
}

/// C's single-row INSERT program for k values (probed):
///   Init 0 6+k | OpenWrite 0 root 0 p4=k | <value loads -> r2..r1+k> |
///   NewRowid 0 1 | MakeRecord 2 k 2+k p4=<affinity string> |
///   Insert 0 2+k 1 p4=<table> p5=57 | Halt |
///   Transaction 0 1 cookie p4=0 p5=1 (a WRITE transaction: p2=1) | Goto 0 1
pub fn compile_insert(root: u32, cookie: u32, tname: &str, aff: &str, vals: &[InsVal]) -> Vec<Op> {
    let k = vals.len() as i64;
    let mut prog = Vec::with_capacity(10 + vals.len());
    prog.push(op_("Init", 0, 6 + k, 0));
    let mut ow = op_("OpenWrite", 0, root as i64, 0);
    ow.p4 = Some(k.to_string());
    prog.push(ow);
    for (i, v) in vals.iter().enumerate() {
        let reg = i as i64 + 2;
        prog.push(match v {
            InsVal::Int(n) => op_("Integer", *n, reg, 0),
            InsVal::Text(t) => { let mut o = op_("String8", 0, reg, 0); o.p4 = Some(t.clone()); o }
            InsVal::Var(n) => op_("Variable", *n as i64, reg, 0),
        });
    }
    prog.push(op_("NewRowid", 0, 1, 0));
    let mut mr = op_("MakeRecord", 2, k, 2 + k);
    mr.p4 = Some(aff.to_string());
    prog.push(mr);
    let mut ins = op_("Insert", 0, 2 + k, 1);
    ins.p4 = Some(tname.to_string());
    ins.p5 = 57;
    prog.push(ins);
    prog.push(op_("Halt", 0, 0, 0));
    let mut txn = op_("Transaction", 0, 1, cookie as i64);
    txn.p4 = Some("0".to_string());
    txn.p5 = 1;
    prog.push(txn);
    prog.push(op_("Goto", 0, 1, 0));
    prog
}

/// C's table-scan program for SELECT of k columns from a rowid table:
///   Init 0 6+k | OpenRead 0 root 0 p4=hint | Rewind 0 5+k | Column x k |
///   ResultRow 1 k | Next 0 3 p5=1 | Halt | Transaction 0 0 cookie p4=0 p5=1 |
///   Goto 0 1
/// (probed on the pin: OpenRead p4 = max used column + 1; Transaction p3 = the
/// schema cookie from the file header; Rewind jumps to Halt when empty).
pub fn compile_scan(root: u32, cookie: u32, colidx: &[usize]) -> Vec<Op> {
    let k = colidx.len() as i64;
    let hint = colidx.iter().copied().max().unwrap_or(0) as i64 + 1;
    let mut prog = Vec::with_capacity(8 + colidx.len());
    prog.push(op_("Init", 0, 6 + k, 0));
    let mut openread = op_("OpenRead", 0, root as i64, 0);
    openread.p4 = Some(hint.to_string());
    prog.push(openread);
    prog.push(op_("Rewind", 0, 5 + k, 0));
    for (i, c) in colidx.iter().enumerate() {
        prog.push(op_("Column", 0, *c as i64, i as i64 + 1));
    }
    prog.push(op_("ResultRow", 1, k, 0));
    let mut next = op_("Next", 0, 3, 0);
    next.p5 = 1;
    prog.push(next);
    prog.push(op_("Halt", 0, 0, 0));
    let mut txn = op_("Transaction", 0, 0, cookie as i64);
    txn.p4 = Some("0".to_string());
    txn.p5 = 1;
    prog.push(txn);
    prog.push(op_("Goto", 0, 1, 0));
    prog
}

/// the EXPLAIN listing rows for a program: addr, opcode, p1, p2, p3, p4, p5,
/// comment — p4 NULL unless set, p5 printed as an integer, comment NULL.
pub fn explain_rows(prog: &[Op]) -> Vec<Vec<Option<String>>> {
    prog.iter().enumerate().map(|(addr, o)| vec![
        Some(addr.to_string()),
        Some(o.opcode.to_string()),
        Some(o.p1.to_string()),
        Some(o.p2.to_string()),
        Some(o.p3.to_string()),
        o.p4.clone(),
        Some(o.p5.to_string()),
        None,
    ]).collect()
}

/// the dispatch loop: run the program, returning result rows. Init/Goto move the
/// pc; Integer/String8 load registers; Add computes r[p3]=r[p1]+r[p2]; ResultRow
/// emits r[p1..p1+p2-1]; Halt stops. Every executed opcode bumps the counter.
pub fn execute(prog: &[Op]) -> Vec<Vec<V>> { execute_with(prog, None) }

fn val_to_v(v: &Val) -> V {
    match v {
        Val::Null => V::Null,
        Val::Int(i) => V::Int(*i),
        Val::Real(r) => V::Real(*r),
        Val::Text(t) => V::Text(t.clone()),
        Val::Blob(b) => V::Blob(b.clone()),
    }
}

/// run-59: the loop with an optional read cursor. `cells` are the table's btree
/// cells (rowid, record payload) parsed from the FILE IMAGE by
/// pager::read_table_cells — OpenRead opens the cursor on them, Rewind moves to
/// the first cell (or jumps p2 when empty), Column DECODES THE CURRENT CELL'S
/// PAYLOAD into a register (never the kitchen store), Next advances and loops to
/// p2 while rows remain. Transaction is the read-txn no-op of this slice.
pub fn execute_with(prog: &[Op], cells: Option<&[(i64, Vec<u8>)]>) -> Vec<Vec<V>> {
    execute_bound(prog, cells, &[])
}

/// SQLite's cross-type value order for the BINARY-8 compares of this slice:
/// NULL < numbers < text < blob. Some(ordering) — None only when either side is
/// NULL (the p5 & 0x10 JUMPIFNULL bit decides the jump then).
fn cmp_v(a: &V, b: &V) -> Option<std::cmp::Ordering> {
    use std::cmp::Ordering::*;
    let rank = |v: &V| match v { V::Null => 0, V::Int(_) | V::Real(_) => 1, V::Text(_) => 2, V::Blob(_) => 3 };
    match (a, b) {
        (V::Null, _) | (_, V::Null) => None,
        (V::Int(x), V::Int(y)) => Some(x.cmp(y)),
        (V::Real(x), V::Real(y)) => x.partial_cmp(y).or(Some(Equal)),
        (V::Int(x), V::Real(y)) => (*x as f64).partial_cmp(y).or(Some(Equal)),
        (V::Real(x), V::Int(y)) => x.partial_cmp(&(*y as f64)).or(Some(Equal)),
        (V::Text(x), V::Text(y)) => Some(x.cmp(y)),
        (V::Blob(x), V::Blob(y)) => Some(x.cmp(y)),
        _ => Some(rank(a).cmp(&rank(b))),
    }
}

/// run-60: the loop with 1-based statement parameters (OP_Variable) and the
/// compare opcodes C emits for WHERE (jump-to-p2 when the relation holds, or
/// when a side is NULL and p5 carries the jump-if-null bit 0x10).
pub fn execute_bound(prog: &[Op], cells: Option<&[(i64, Vec<u8>)]>, params: &[V]) -> Vec<Vec<V>> {
    execute_dml(prog, cells, params).0
}

static INSERTS: AtomicU64 = AtomicU64::new(0);
/// run-61: cells put by OP_Insert on the VDBE path. Moves on the VM INSERT;
/// does NOT move on kitchen DML or on any read.
pub fn insert_count() -> u64 { INSERTS.load(Ordering::SeqCst) }

fn v_to_val(v: &V) -> Val {
    match v {
        V::Null => Val::Null,
        V::Int(i) => Val::Int(*i),
        V::Real(r) => Val::Real(*r),
        V::Text(t) => Val::Text(t.clone()),
        V::Blob(b) => Val::Blob(b.clone()),
    }
}

/// run-61: the loop with the DML opcodes. Returns (result rows, pending insert):
/// OpenWrite opens the write cursor on the table's cells; NewRowid computes
/// max(rowid)+1 into a register; MakeRecord encodes registers p1..p1+p2-1 into
/// a record blob (p4's affinity string is replicated from C's listing, not
/// applied — vdbe-engine-002 stays none); Insert hands the (rowid, payload)
/// cell to the caller, who puts it through pager::insert_cell.
pub fn execute_dml(prog: &[Op], cells: Option<&[(i64, Vec<u8>)]>, params: &[V]) -> (Vec<Vec<V>>, Option<(i64, Vec<u8>)>) {
    let mut regs: Vec<V> = vec![V::Null; 32];
    let mut out: Vec<Vec<V>> = Vec::new();
    let mut pc: usize = 0;
    let mut steps = 0u32;
    let mut cur: Option<&[(i64, Vec<u8>)]> = None; // opened by OpenRead/OpenWrite
    let mut pos: usize = 0;
    let mut pending_insert: Option<(i64, Vec<u8>)> = None;
    while pc < prog.len() && steps < 1_000_000 {
        steps += 1;
        DISPATCH.fetch_add(1, Ordering::SeqCst);
        let o = &prog[pc];
        match o.opcode {
            "Init" | "Goto" => { pc = o.p2 as usize; continue; }
            "Integer" => { regs[o.p2 as usize] = V::Int(o.p1); }
            "String8" => { regs[o.p2 as usize] = V::Text(o.p4.clone().unwrap_or_default()); }
            "Add" => {
                let a = match &regs[o.p1 as usize] { V::Int(i) => *i, _ => 0 };
                let b = match &regs[o.p2 as usize] { V::Int(i) => *i, _ => 0 };
                regs[o.p3 as usize] = V::Int(a + b);
            }
            "Transaction" => {} // read transaction on the main db of this slice
            "Variable" => {
                regs[o.p2 as usize] = params.get(o.p1 as usize - 1).cloned().unwrap_or(V::Null);
            }
            "Eq" | "Ne" | "Lt" | "Le" | "Gt" | "Ge" => {
                // compare r[p3] (left) with r[p1] (right); jump to p2 when the
                // relation holds — C's inverted WHERE test lands on Next/Halt.
                use std::cmp::Ordering::*;
                let jump = match cmp_v(&regs[o.p3 as usize], &regs[o.p1 as usize]) {
                    None => o.p5 & 0x10 != 0, // NULL side: jump-if-null bit
                    Some(ord) => match o.opcode {
                        "Eq" => ord == Equal,
                        "Ne" => ord != Equal,
                        "Lt" => ord == Less,
                        "Le" => ord != Greater,
                        "Gt" => ord == Greater,
                        "Ge" => ord != Less,
                        _ => unreachable!(),
                    },
                };
                if jump { pc = o.p2 as usize; continue; }
            }
            "SeekRowid" => {
                // position the cursor on the cell whose rowid equals r[p3];
                // jump p2 when there is no such row. A single-cell touch.
                let c = cur.expect("SeekRowid before OpenRead");
                let want = match &regs[o.p3 as usize] { V::Int(i) => Some(*i), _ => None };
                match want.and_then(|w| c.iter().position(|(rid, _)| *rid == w)) {
                    Some(p) => {
                        pos = p;
                        CURSOR_READS.fetch_add(1, Ordering::SeqCst); // positioned on the sought cell
                    }
                    None => { pc = o.p2 as usize; continue; }
                }
            }
            "OpenRead" | "OpenWrite" => {
                cur = Some(cells.expect("Open cursor without a source (must never compile)"));
                pos = 0;
            }
            "NewRowid" => {
                let c = cur.expect("NewRowid before OpenWrite");
                let max = c.iter().map(|(r, _)| *r).max().unwrap_or(0);
                regs[o.p2 as usize] = V::Int(max + 1);
            }
            "MakeRecord" => {
                let vals: Vec<Val> = (o.p1..o.p1 + o.p2).map(|r| v_to_val(&regs[r as usize])).collect();
                regs[o.p3 as usize] = V::Blob(crate::dbfile::encode_record(&vals));
            }
            "Insert" => {
                let rowid = match &regs[o.p3 as usize] { V::Int(i) => *i, _ => 0 };
                let payload = match &regs[o.p2 as usize] { V::Blob(b) => b.clone(), _ => Vec::new() };
                pending_insert = Some((rowid, payload));
                INSERTS.fetch_add(1, Ordering::SeqCst);
            }
            "Rewind" => {
                let c = cur.expect("Rewind before OpenRead");
                pos = 0;
                if c.is_empty() { pc = o.p2 as usize; continue; }
                CURSOR_READS.fetch_add(1, Ordering::SeqCst); // positioned on the first cell
            }
            "Column" => {
                let c = cur.expect("Column before OpenRead");
                let payload = &c[pos].1;
                let vals = crate::dbfile::decode_record(payload);
                regs[o.p3 as usize] = vals.get(o.p2 as usize).map(val_to_v).unwrap_or(V::Null);
            }
            "Next" => {
                let c = cur.expect("Next before OpenRead");
                pos += 1;
                if pos < c.len() {
                    CURSOR_READS.fetch_add(1, Ordering::SeqCst); // positioned on the next cell
                    pc = o.p2 as usize;
                    continue;
                }
            }
            "ResultRow" => {
                let row: Vec<V> = (o.p1..o.p1 + o.p2).map(|r| regs[r as usize].clone()).collect();
                out.push(row);
            }
            "Halt" => break,
            other => panic!("vdbe: unimplemented opcode {other} (must never compile)"),
        }
        pc += 1;
    }
    (out, pending_insert)
}
