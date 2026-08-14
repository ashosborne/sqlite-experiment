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
fn op(opcode: &'static str, p1: i64, p2: i64, p3: i64) -> Op {
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
                op("Init", 0, 5, 0),
                op("Integer", a, 1, 0),
                op("Integer", b, 2, 0),
                op("ResultRow", 1, 2, 0),
                op("Halt", 0, 0, 0),
                op("Goto", 0, 1, 0),
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
                op("Init", 0, 4, 0),
                op("Add", 3, 2, 1),
                op("ResultRow", 1, 1, 0),
                op("Halt", 0, 0, 0),
                op("Integer", a, 2, 0),
                op("Integer", b, 3, 0),
                op("Goto", 0, 1, 0),
            ]);
        }
        return None;
    }
    // SELECT '<text>'
    if let Some(t) = parse_text(items_txt) {
        if wh.is_none() {
            let mut s8 = op("String8", 0, 1, 0);
            s8.p4 = Some(t);
            return Some(vec![
                op("Init", 0, 4, 0),
                s8,
                op("ResultRow", 1, 1, 0),
                op("Halt", 0, 0, 0),
                op("Goto", 0, 1, 0),
            ]);
        }
        return None;
    }
    // SELECT <int> [WHERE 1|0]
    let n = parse_int(items_txt)?;
    match wh {
        None | Some(true) => Some(vec![
            op("Init", 0, 4, 0),
            op("Integer", n, 1, 0),
            op("ResultRow", 1, 1, 0),
            op("Halt", 0, 0, 0),
            op("Goto", 0, 1, 0),
        ]),
        Some(false) => Some(vec![
            op("Init", 0, 5, 0),
            op("Goto", 0, 4, 0),
            op("Integer", n, 1, 0),
            op("ResultRow", 1, 1, 0),
            op("Halt", 0, 0, 0),
            op("Goto", 0, 1, 0),
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
    prog.push(op("Init", 0, 6 + k, 0));
    let mut openread = op("OpenRead", 0, root as i64, 0);
    openread.p4 = Some(hint.to_string());
    prog.push(openread);
    prog.push(op("Rewind", 0, 5 + k, 0));
    for (i, c) in colidx.iter().enumerate() {
        prog.push(op("Column", 0, *c as i64, i as i64 + 1));
    }
    prog.push(op("ResultRow", 1, k, 0));
    let mut next = op("Next", 0, 3, 0);
    next.p5 = 1;
    prog.push(next);
    prog.push(op("Halt", 0, 0, 0));
    let mut txn = op("Transaction", 0, 0, cookie as i64);
    txn.p4 = Some("0".to_string());
    txn.p5 = 1;
    prog.push(txn);
    prog.push(op("Goto", 0, 1, 0));
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
    let mut regs: Vec<V> = vec![V::Null; 32];
    let mut out: Vec<Vec<V>> = Vec::new();
    let mut pc: usize = 0;
    let mut steps = 0u32;
    let mut cur: Option<&[(i64, Vec<u8>)]> = None; // opened by OpenRead
    let mut pos: usize = 0;
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
            "OpenRead" => {
                cur = Some(cells.expect("OpenRead without a cursor source (must never compile)"));
                pos = 0;
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
    out
}
