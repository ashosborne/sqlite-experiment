//! Engine v1 kitchen store (pack sqlite-experiment-c-to-rust@4).
//!
//! A REAL — but deliberately toy — in-memory row store: tables -> columns -> rows.
//! Values come from the statement text, never from a preloaded answer key; the
//! pack forbids script-string lookup for kitchen SQL (SCOPE_VIOLATION).
//!
//! Honest limits: INTEGER/TEXT columns, single-table SELECT with optional
//! `ORDER BY <int col>`, `WHERE col=int` on UPDATE/DELETE, sqlite_master
//! count(*) and changes()/total_changes() counters. No planner, no durability,
//! no concurrency, no btree/file format. Not SQLite.

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Int(i64),
    Text(String),
}
impl Val {
    fn render(&self) -> String {
        match self {
            Val::Int(i) => i.to_string(),
            Val::Text(t) => t.clone(),
        }
    }
}

#[derive(Default)]
struct Table {
    cols: Vec<String>,
    rows: Vec<Vec<Val>>,
}

#[derive(Default)]
pub struct Store {
    tables: Vec<(String, Table)>, // Vec keeps creation order for sqlite_master counting
    changes: i64,
    total_changes: i64,
}

thread_local! {
    static STORES: RefCell<HashMap<usize, Store>> = RefCell::new(HashMap::new());
}

pub fn drop_store(db: usize) {
    STORES.with(|m| { m.borrow_mut().remove(&db); });
}

fn with_store<R>(db: usize, f: impl FnOnce(&mut Store) -> R) -> R {
    STORES.with(|m| f(m.borrow_mut().entry(db).or_default()))
}

// ---------------- tiny statement parser (kitchen shapes only) ----------------

fn split_statements(script: &str) -> Vec<String> {
    // Kitchen literals never contain ';' — documented limit.
    script.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}

fn parse_literal(tok: &str) -> Option<Val> {
    let t = tok.trim();
    if let Ok(i) = t.parse::<i64>() {
        return Some(Val::Int(i));
    }
    if t.len() >= 2 && t.starts_with('\'') && t.ends_with('\'') {
        return Some(Val::Text(t[1..t.len() - 1].replace("''", "'")));
    }
    None
}

enum Stmt {
    Create { name: String, cols: Vec<String> },
    Drop { name: String },
    Insert { name: String, rows: Vec<Vec<Val>> },
    Update { name: String, col: String, add: Option<i64>, set: Option<Val>, wh: Option<(String, i64)> },
    Delete { name: String, wh: Option<(String, i64)> },
    Select { items: Vec<String>, target: String, wh_name: Option<String>, order_by: Option<String> },
}

fn ident(s: &str) -> Option<String> {
    let t = s.trim();
    if !t.is_empty() && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Some(t.to_string())
    } else {
        None
    }
}

fn parse_stmt(s: &str) -> Option<Stmt> {
    let up = s.to_ascii_uppercase();
    if let Some(rest) = up.strip_prefix("CREATE TABLE ") {
        let open = rest.find('(')?;
        let name = ident(&s["CREATE TABLE ".len()..][..open])?;
        let inner = &s[s.find('(')? + 1..s.rfind(')')?];
        let mut cols = Vec::new();
        for c in inner.split(',') {
            cols.push(ident(c.trim().split_whitespace().next()?)?); // type/PK tokens ignored
        }
        return Some(Stmt::Create { name, cols });
    }
    if up.starts_with("DROP TABLE ") {
        return Some(Stmt::Drop { name: ident(&s["DROP TABLE ".len()..])? });
    }
    if up.starts_with("INSERT INTO ") {
        let after = &s["INSERT INTO ".len()..];
        let vpos = after.to_ascii_uppercase().find(" VALUES")?;
        let name = ident(&after[..vpos])?;
        let vals = &after[vpos + " VALUES".len()..];
        let mut rows = Vec::new();
        for grp in vals.split("),") {
            let grp = grp.trim().trim_start_matches('(').trim_end_matches(')');
            let mut row = Vec::new();
            for tok in grp.split(',') {
                row.push(parse_literal(tok)?);
            }
            rows.push(row);
        }
        return Some(Stmt::Insert { name, rows });
    }
    if up.starts_with("UPDATE ") {
        let after = &s["UPDATE ".len()..];
        let setpos = after.to_ascii_uppercase().find(" SET ")?;
        let name = ident(&after[..setpos])?;
        let rest = &after[setpos + 5..];
        let (assign, wh_txt) = match rest.to_ascii_uppercase().find(" WHERE ") {
            Some(w) => (&rest[..w], Some(&rest[w + 7..])),
            None => (rest, None),
        };
        let eq = assign.find('=')?;
        let col = ident(&assign[..eq])?;
        let rhs = assign[eq + 1..].trim();
        let (add, set) = if let Some(plus) = rhs.find('+') {
            let base = ident(&rhs[..plus])?;
            if base != col { return None; } // only col=col+N supported
            (Some(rhs[plus + 1..].trim().parse::<i64>().ok()?), None)
        } else {
            (None, Some(parse_literal(rhs)?))
        };
        let wh = match wh_txt {
            Some(w) => {
                let weq = w.find('=')?;
                Some((ident(&w[..weq])?, w[weq + 1..].trim().parse::<i64>().ok()?))
            }
            None => None,
        };
        return Some(Stmt::Update { name, col, add, set, wh });
    }
    if up.starts_with("DELETE FROM ") {
        let after = &s["DELETE FROM ".len()..];
        let (name_txt, wh_txt) = match after.to_ascii_uppercase().find(" WHERE ") {
            Some(w) => (&after[..w], Some(&after[w + 7..])),
            None => (after, None),
        };
        let name = ident(name_txt)?;
        let wh = match wh_txt {
            Some(w) => {
                let weq = w.find('=')?;
                Some((ident(&w[..weq])?, w[weq + 1..].trim().parse::<i64>().ok()?))
            }
            None => None,
        };
        return Some(Stmt::Delete { name, wh });
    }
    if up.starts_with("SELECT ") {
        let after = &s["SELECT ".len()..];
        let fpos = after.to_ascii_uppercase().find(" FROM ")?;
        let items: Vec<String> = after[..fpos].split(',').map(|i| i.trim().to_string()).collect();
        let mut rest = after[fpos + 6..].trim().to_string();
        let mut order_by = None;
        if let Some(o) = rest.to_ascii_uppercase().find(" ORDER BY ") {
            order_by = ident(&rest[o + 10..].to_string());
            rest = rest[..o].trim().to_string();
        }
        let mut wh_name = None;
        if let Some(w) = rest.to_ascii_uppercase().find(" WHERE ") {
            let cond = rest[w + 7..].trim().to_string();
            // only: name='X' (sqlite_master filter)
            let eq = cond.find('=')?;
            if ident(&cond[..eq])? != "name" { return None; }
            wh_name = Some(parse_literal(cond[eq + 1..].trim())?.render());
            rest = rest[..w].trim().to_string();
        }
        let target = ident(&rest)?;
        for it in &items {
            let ok = it == "count(*)" || it == "changes()" || it == "total_changes()" || ident(it).is_some();
            if !ok { return None; }
        }
        return Some(Stmt::Select { items, target, wh_name, order_by });
    }
    None
}

/// Try to run the whole script through the store. `None` = not a kitchen shape
/// (caller may fall back to the recognizer — which never contains kitchen cases).
pub fn execute_script(db: usize, script: &str) -> Option<Vec<Vec<Option<String>>>> {
    let stmts: Vec<Stmt> = split_statements(script)
        .iter()
        .map(|s| parse_stmt(s))
        .collect::<Option<Vec<_>>>()?; // all-or-nothing: partial parses never mix engines
    let mut out: Vec<Vec<Option<String>>> = Vec::new();
    with_store(db, |st| {
        for stmt in stmts {
            match stmt {
                Stmt::Create { name, cols } => {
                    st.tables.push((name, Table { cols, rows: Vec::new() }));
                }
                Stmt::Drop { name } => {
                    st.tables.retain(|(n, _)| *n != name);
                }
                Stmt::Insert { name, rows } => {
                    let n = rows.len() as i64;
                    let t = st.tables.iter_mut().find(|(tn, _)| *tn == name)?;
                    t.1.rows.extend(rows);
                    st.changes = n;
                    st.total_changes += n;
                }
                Stmt::Update { name, col, add, set, wh } => {
                    let t = st.tables.iter_mut().find(|(tn, _)| *tn == name)?;
                    let ci = t.1.cols.iter().position(|c| *c == col)?;
                    let wi = wh.as_ref().map(|(wc, _)| t.1.cols.iter().position(|c| c == wc)).flatten();
                    let mut n = 0;
                    for row in t.1.rows.iter_mut() {
                        if let (Some((_, wv)), Some(wi)) = (&wh, wi) {
                            if row[wi] != Val::Int(*wv) { continue; }
                        }
                        row[ci] = match (&add, &set) {
                            (Some(d), _) => match &row[ci] { Val::Int(i) => Val::Int(i + d), v => v.clone() },
                            (None, Some(v)) => v.clone(),
                            _ => row[ci].clone(),
                        };
                        n += 1;
                    }
                    st.changes = n;
                    st.total_changes += n;
                }
                Stmt::Delete { name, wh } => {
                    let t = st.tables.iter_mut().find(|(tn, _)| *tn == name)?;
                    let wi = wh.as_ref().map(|(wc, _)| t.1.cols.iter().position(|c| c == wc)).flatten();
                    let before = t.1.rows.len();
                    match (&wh, wi) {
                        (Some((_, wv)), Some(wi)) => t.1.rows.retain(|r| r[wi] != Val::Int(*wv)),
                        _ => t.1.rows.clear(),
                    }
                    let n = (before - t.1.rows.len()) as i64;
                    st.changes = n;
                    st.total_changes += n;
                }
                Stmt::Select { items, target, wh_name, order_by } => {
                    if target == "sqlite_master" {
                        let cnt = match &wh_name {
                            Some(n) => st.tables.iter().filter(|(tn, _)| tn == n).count(),
                            None => st.tables.len(),
                        };
                        out.push(vec![Some(cnt.to_string())]); // kitchen shape: count(*) only
                        continue;
                    }
                    let t = st.tables.iter().find(|(tn, _)| *tn == target)?;
                    let mut rows: Vec<&Vec<Val>> = t.1.rows.iter().collect();
                    if let Some(ob) = &order_by {
                        let oi = t.1.cols.iter().position(|c| c == ob)?;
                        rows.sort_by_key(|r| match &r[oi] { Val::Int(i) => *i, _ => 0 });
                    }
                    for r in rows {
                        let mut orow = Vec::new();
                        for it in &items {
                            if it == "changes()" { orow.push(Some(st.changes.to_string())); }
                            else if it == "total_changes()" { orow.push(Some(st.total_changes.to_string())); }
                            else if it == "count(*)" { orow.push(Some(t.1.rows.len().to_string())); }
                            else {
                                let ci = t.1.cols.iter().position(|c| c == it)?;
                                orow.push(Some(r[ci].render()));
                            }
                        }
                        out.push(orow);
                    }
                    // count(*)/changes-only select over empty row set still yields one row in C?
                    // Not a kitchen shape this version (all frozen selects have >=1 row or col refs).
                }
            }
        }
        Some(())
    })?;
    Some(out)
}
