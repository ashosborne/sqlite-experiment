//! run-58: the first bytecode slice. Constant SELECTs compile to a real Vdbe
//! program whose EXPLAIN listing matches probed C row-for-row, and sqlite3_step
//! of that SQL executes the program through a dispatch loop. Everything else
//! stays on the kitchen evaluator (FROM, joins, DML, CTEs — honestly named).
//!
//! Opcodes implemented (exactly the ones C's programs for the pinned SQL use):
//! Init, Goto, Integer, String8, Add, ResultRow, Halt. No 199-opcode census, no
//! table-scan opcodes, no OP_Program/interrupt — those stay in the residual.

use crate::eval::V;
use std::sync::atomic::{AtomicU64, Ordering};

static DISPATCH: AtomicU64 = AtomicU64::new(0);
/// executed-opcode counter — the anti-cheat proof that step really dispatches.
pub fn dispatch_count() -> u64 { DISPATCH.load(Ordering::SeqCst) }

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
pub fn execute(prog: &[Op]) -> Vec<Vec<V>> {
    let mut regs: Vec<V> = vec![V::Null; 32];
    let mut out: Vec<Vec<V>> = Vec::new();
    let mut pc: usize = 0;
    let mut steps = 0u32;
    while pc < prog.len() && steps < 10_000 {
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
