//! Engine v5 kitchen store (pack sqlite-experiment-c-to-rust@5).
//!
//! A REAL — but still deliberately toy — in-memory row store. v5 grows it to
//! serve the eleven re-homed table scripts: qualified names + rowid, UNIQUE
//! (column + index) with OR IGNORE / OR REPLACE, ALTER RENAME / ADD COLUMN
//! DEFAULT, PK upserts (DO NOTHING / DO UPDATE SET c=excluded.c), foreign keys
//! (insert check rc=19, ON DELETE CASCADE, DROP-parent check rc=19) and AFTER
//! INSERT triggers with `new.col [*N]` bodies.
//!
//! Honest limits: UNIQUE/FK/triggers here are in-memory rules over Vec rows —
//! not SQLite's btree, not a planner, not durable, not concurrent. Values come
//! from statement text; the pack forbids script-string lookup for kitchen SQL.

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Int(i64),
    Text(String),
    Null,
}
impl Val {
    fn render(&self) -> Option<String> {
        match self {
            Val::Int(i) => Some(i.to_string()),
            Val::Text(t) => Some(t.clone()),
            Val::Null => None,
        }
    }
}

#[derive(Clone, Default)]
struct Col {
    name: String,
    unique: bool, // PK or UNIQUE or UNIQUE INDEX
    default: Option<Val>,
    references: Option<(String, String, bool)>, // (parent table, parent col, on-delete-cascade)
}

#[derive(Default)]
struct Table {
    cols: Vec<Col>,
    rows: Vec<(i64, Vec<Val>)>, // (rowid, values)
    next_rowid: i64,
}

#[derive(Clone)]
struct Trigger {
    table: String,                       // ON <table>, AFTER INSERT
    body_target: String,                 // INSERT INTO <target>
    body_col: String,                    // new.<col>
    body_mult: i64,                      // new.<col> * N (N=1 when absent)
}

#[derive(Default)]
pub struct Store {
    tables: Vec<(String, Table)>,
    catalog: Vec<(String, String)>, // (type: table|index|trigger, name) — creation order
    index_owner: HashMap<String, String>, // index name -> table
    triggers: Vec<(String, Trigger)>,     // (trigger name, def)
    fk_on: bool,
    changes: i64,
    total_changes: i64,
}

pub enum Outcome {
    NotKitchen,
    Done { rows: Vec<Vec<Option<String>>>, rc: i32, err: Option<String> },
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

// ------------------------------ parsing ------------------------------------

/// Split on ';' but keep CREATE TRIGGER ... BEGIN ... END; bodies whole.
fn split_statements(script: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut buf = String::new();
    for frag in script.split(';') {
        if !buf.is_empty() {
            buf.push(';');
        }
        buf.push_str(frag);
        let up = buf.to_ascii_uppercase();
        let in_trigger = up.contains("CREATE TRIGGER") && up.contains("BEGIN")
            && !up.trim_end().ends_with("END");
        if !in_trigger {
            let s = buf.trim().to_string();
            if !s.is_empty() {
                out.push(s);
            }
            buf.clear();
        }
    }
    let s = buf.trim().to_string();
    if !s.is_empty() {
        out.push(s);
    }
    out
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

fn ident(s: &str) -> Option<String> {
    let t = s.trim();
    if !t.is_empty() && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Some(t.to_string())
    } else {
        None
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Policy { Abort, Ignore, Replace, DoNothing, DoUpdate }

enum Stmt {
    PragmaFkOn,
    Create { name: String, cols: Vec<Col> },
    CreateIndex { name: String, table: String, col: String, unique: bool },
    CreateTrigger { name: String, def: Trigger },
    Drop { name: String },
    RenameTable { from: String, to: String },
    AddColumn { table: String, col: String, default: Option<Val> },
    Insert { name: String, collist: Option<Vec<String>>, rows: Vec<Vec<Val>>, policy: Policy,
             upd_col: Option<String> /* DO UPDATE SET c=excluded.c */ },
    Update { name: String, col: String, add: Option<i64>, set: Option<Val>, wh: Option<(String, i64)> },
    Delete { name: String, wh: Option<(String, i64)> },
    Select { items: Vec<String>, target: String, wh: Option<(String, String)>, order_by: Option<String> },
}

fn parse_coldefs(inner: &str) -> Option<Vec<Col>> {
    // split on commas that are OUTSIDE parens (REFERENCES par(id) has inner parens)
    let mut defs = Vec::new();
    let mut depth = 0;
    let mut cur = String::new();
    for ch in inner.chars() {
        match ch {
            '(' => { depth += 1; cur.push(ch); }
            ')' => { depth -= 1; cur.push(ch); }
            ',' if depth == 0 => { defs.push(cur.clone()); cur.clear(); }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() { defs.push(cur); }
    let mut cols = Vec::new();
    for d in defs {
        let d = d.trim();
        let name = ident(d.split_whitespace().next()?)?;
        let up = d.to_ascii_uppercase();
        let mut col = Col { name, ..Default::default() };
        if up.contains("PRIMARY KEY") || up.contains(" UNIQUE") { col.unique = true; }
        if let Some(rp) = up.find("REFERENCES ") {
            let rest = &d[rp + "REFERENCES ".len()..];
            let open = rest.find('(')?;
            let parent = ident(&rest[..open])?;
            let close = rest.find(')')?;
            let pcol = ident(&rest[open + 1..close])?;
            let cascade = up.contains("ON DELETE CASCADE");
            col.references = Some((parent, pcol, cascade));
        }
        if let Some(dp) = up.find("DEFAULT ") {
            col.default = parse_literal(d[dp + "DEFAULT ".len()..].split_whitespace().next()?);
        }
        cols.push(col);
    }
    Some(cols)
}

fn parse_where_int(w: &str) -> Option<(String, i64)> {
    let eq = w.find('=')?;
    Some((ident(&w[..eq])?, w[eq + 1..].trim().parse::<i64>().ok()?))
}

fn parse_stmt(s: &str) -> Option<Stmt> {
    let up = s.to_ascii_uppercase();
    if up == "PRAGMA FOREIGN_KEYS=ON" || up == "PRAGMA FOREIGN_KEYS = ON" {
        return Some(Stmt::PragmaFkOn); // ONLY this pragma; all others stay recognizer territory
    }
    if let Some(_r) = up.strip_prefix("CREATE TABLE ") {
        let open = s.find('(')?;
        let name = ident(&s["CREATE TABLE ".len()..open])?;
        let inner = &s[open + 1..s.rfind(')')?];
        return Some(Stmt::Create { name, cols: parse_coldefs(inner)? });
    }
    if up.starts_with("CREATE UNIQUE INDEX ") || up.starts_with("CREATE INDEX ") {
        let unique = up.starts_with("CREATE UNIQUE");
        let skip = if unique { "CREATE UNIQUE INDEX " } else { "CREATE INDEX " }.len();
        let rest = &s[skip..];
        let onp = rest.to_ascii_uppercase().find(" ON ")?;
        let name = ident(&rest[..onp])?;
        let tail = &rest[onp + 4..];
        let open = tail.find('(')?;
        let table = ident(&tail[..open])?;
        let col = ident(&tail[open + 1..tail.find(')')?])?;
        return Some(Stmt::CreateIndex { name, table, col, unique });
    }
    if up.starts_with("CREATE TRIGGER ") {
        // shape: CREATE TRIGGER <n> AFTER INSERT ON <t> BEGIN INSERT INTO <x> VALUES(new.<c>[*N]); END
        let rest = &s["CREATE TRIGGER ".len()..];
        let after = rest.to_ascii_uppercase().find(" AFTER INSERT ON ")?;
        let name = ident(&rest[..after])?;
        let tail = &rest[after + " AFTER INSERT ON ".len()..];
        let beg = tail.to_ascii_uppercase().find(" BEGIN ")?;
        let table = ident(&tail[..beg])?;
        let mut body = tail[beg + 7..].trim();
        body = body.strip_suffix("END").unwrap_or(body).trim_end();
        body = body.strip_suffix(';').unwrap_or(body).trim();
        let bup = body.to_ascii_uppercase();
        let bt = bup.strip_prefix("INSERT INTO ")?;
        let vpos = bt.find(" VALUES(")?;
        let body_target = ident(&body["INSERT INTO ".len()..]["".len()..vpos])?;
        let expr = body["INSERT INTO ".len() + vpos + " VALUES(".len()..].trim_end_matches(')').trim();
        let eup = expr.to_ascii_lowercase();
        let e = eup.strip_prefix("new.")?;
        let (body_col, body_mult) = match e.find('*') {
            Some(m) => (ident(&e[..m])?, e[m + 1..].trim().parse::<i64>().ok()?),
            None => (ident(e)?, 1),
        };
        return Some(Stmt::CreateTrigger { name, def: Trigger { table, body_target, body_col, body_mult } });
    }
    if up.starts_with("DROP TABLE ") {
        return Some(Stmt::Drop { name: ident(&s["DROP TABLE ".len()..])? });
    }
    if up.starts_with("ALTER TABLE ") {
        let rest = &s["ALTER TABLE ".len()..];
        let rup = rest.to_ascii_uppercase();
        if let Some(rn) = rup.find(" RENAME TO ") {
            return Some(Stmt::RenameTable { from: ident(&rest[..rn])?, to: ident(&rest[rn + 11..])? });
        }
        if let Some(ac) = rup.find(" ADD COLUMN ") {
            let table = ident(&rest[..ac])?;
            let coldef = &rest[ac + 12..];
            let cols = parse_coldefs(coldef)?;
            let c = cols.into_iter().next()?;
            return Some(Stmt::AddColumn { table, col: c.name, default: c.default });
        }
        return None;
    }
    if up.starts_with("INSERT ") {
        let (policy0, after) = if up.starts_with("INSERT OR IGNORE INTO ") {
            (Policy::Ignore, &s["INSERT OR IGNORE INTO ".len()..])
        } else if up.starts_with("INSERT OR REPLACE INTO ") {
            (Policy::Replace, &s["INSERT OR REPLACE INTO ".len()..])
        } else if up.starts_with("INSERT INTO ") {
            (Policy::Abort, &s["INSERT INTO ".len()..])
        } else {
            return None;
        };
        let aup = after.to_ascii_uppercase();
        let vpos = aup.find(" VALUES")?;
        let head = after[..vpos].trim();
        let (name, collist) = match head.find('(') {
            Some(o) => {
                let n = ident(&head[..o])?;
                let inner = &head[o + 1..head.rfind(')')?];
                let cl = inner.split(',').map(|c| ident(c)).collect::<Option<Vec<_>>>()?;
                (n, Some(cl))
            }
            None => (ident(head)?, None),
        };
        let mut vals = after[vpos + " VALUES".len()..].trim();
        let mut policy = policy0;
        let mut upd_col = None;
        let vup = vals.to_ascii_uppercase();
        if let Some(oc) = vup.find(" ON CONFLICT") {
            let clause = &vals[oc..];
            let cup = clause.to_ascii_uppercase();
            if cup.contains("DO NOTHING") {
                policy = Policy::DoNothing;
            } else if let Some(du) = cup.find("DO UPDATE SET ") {
                policy = Policy::DoUpdate;
                let assign = &clause[du + "DO UPDATE SET ".len()..];
                let eq = assign.find('=')?;
                let c = ident(&assign[..eq])?;
                let rhs = assign[eq + 1..].trim().to_ascii_lowercase();
                if rhs != format!("excluded.{}", c) { return None; } // only c=excluded.c
                upd_col = Some(c);
            } else {
                return None;
            }
            vals = vals[..oc].trim();
        }
        let mut rows = Vec::new();
        for grp in vals.split("),") {
            let grp = grp.trim().trim_start_matches('(').trim_end_matches(')');
            let mut row = Vec::new();
            for tok in grp.split(',') {
                row.push(parse_literal(tok)?);
            }
            rows.push(row);
        }
        return Some(Stmt::Insert { name, collist, rows, policy, upd_col });
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
            if base != col { return None; }
            (Some(rhs[plus + 1..].trim().parse::<i64>().ok()?), None)
        } else {
            (None, Some(parse_literal(rhs)?))
        };
        let wh = match wh_txt { Some(w) => Some(parse_where_int(w)?), None => None };
        return Some(Stmt::Update { name, col, add, set, wh });
    }
    if up.starts_with("DELETE FROM ") {
        let after = &s["DELETE FROM ".len()..];
        let (nm, wh_txt) = match after.to_ascii_uppercase().find(" WHERE ") {
            Some(w) => (&after[..w], Some(&after[w + 7..])),
            None => (after, None),
        };
        let wh = match wh_txt { Some(w) => Some(parse_where_int(w)?), None => None };
        return Some(Stmt::Delete { name: ident(nm)?, wh });
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
        let mut wh = None;
        if let Some(w) = rest.to_ascii_uppercase().find(" WHERE ") {
            let cond = rest[w + 7..].trim().to_string();
            let eq = cond.find('=')?;
            let key = ident(&cond[..eq])?; // sqlite_master filters: name='x' | type='x'
            if key != "name" && key != "type" { return None; }
            let v = parse_literal(cond[eq + 1..].trim())?;
            wh = Some((key, v.render()?));
            rest = rest[..w].trim().to_string();
        }
        let target = ident(&rest)?;
        for it in &items {
            let base = it.strip_prefix(&format!("{target}.")).unwrap_or(it);
            let ok = base == "count(*)" || base == "changes()" || base == "total_changes()"
                || base == "rowid" || ident(base).is_some();
            if !ok { return None; }
        }
        return Some(Stmt::Select { items, target, wh, order_by });
    }
    None
}

// ------------------------------ execution ----------------------------------

const FK_ERR: &str = "FOREIGN KEY constraint failed";
const UNIQ_ERR: &str = "UNIQUE constraint failed";

fn conflict_row(t: &Table, vals: &[Val]) -> Option<usize> {
    for (ci, col) in t.cols.iter().enumerate() {
        if !col.unique { continue; }
        if let Some(v) = vals.get(ci) {
            if let Some(pos) = t.rows.iter().position(|(_, r)| r.get(ci) == Some(v)) {
                return Some(pos);
            }
        }
    }
    None
}

pub fn execute_script(db: usize, script: &str) -> Outcome {
    let parsed: Option<Vec<Stmt>> =
        split_statements(script).iter().map(|s| parse_stmt(s)).collect();
    let stmts = match parsed {
        Some(v) if !v.is_empty() => v,
        _ => return Outcome::NotKitchen, // all-or-nothing: never mix store + recognizer
    };
    // Referenced-table gate: the store only claims a script when every table it
    // touches is created in-script or already lives in this connection's store.
    // Anything else (pragma_* projections, vtabs, json_each, ...) is NotKitchen.
    {
        let mut known: Vec<String> = with_store(db, |st| st.tables.iter().map(|(n, _)| n.clone()).collect());
        let mut ok = true;
        for s in &stmts {
            match s {
                Stmt::Create { name, cols } => {
                    for c in cols {
                        if let Some((p, _, _)) = &c.references {
                            if !known.contains(p) { ok = false; }
                        }
                    }
                    known.push(name.clone());
                }
                Stmt::CreateIndex { table, .. } => { if !known.contains(table) { ok = false; } }
                Stmt::CreateTrigger { def, .. } => {
                    if !known.contains(&def.table) || !known.contains(&def.body_target) { ok = false; }
                }
                Stmt::Drop { name } | Stmt::Insert { name, .. } | Stmt::Update { name, .. }
                | Stmt::Delete { name, .. } => { if !known.contains(name) { ok = false; } }
                Stmt::RenameTable { from, to } => {
                    if !known.contains(from) { ok = false; }
                    known.push(to.clone());
                }
                Stmt::AddColumn { table, .. } => { if !known.contains(table) { ok = false; } }
                Stmt::Select { target, .. } => {
                    if target != "sqlite_master" && !known.contains(target) { ok = false; }
                }
                Stmt::PragmaFkOn => {}
            }
        }
        if !ok { return Outcome::NotKitchen; }
    }
    let mut out: Vec<Vec<Option<String>>> = Vec::new();
    let res: Result<(), String> = with_store(db, |st| {
        for stmt in stmts {
            match stmt {
                Stmt::PragmaFkOn => st.fk_on = true,
                Stmt::Create { name, cols } => {
                    st.catalog.push(("table".into(), name.clone()));
                    st.tables.push((name, Table { cols, ..Default::default() }));
                }
                Stmt::CreateIndex { name, table, col, unique } => {
                    st.catalog.push(("index".into(), name.clone()));
                    st.index_owner.insert(name, table.clone());
                    if unique {
                        let t = st.tables.iter_mut().find(|(n, _)| *n == table)
                            .ok_or("no such table")?;
                        if let Some(c) = t.1.cols.iter_mut().find(|c| c.name == col) {
                            c.unique = true;
                        }
                    }
                }
                Stmt::CreateTrigger { name, def } => {
                    st.catalog.push(("trigger".into(), name.clone()));
                    st.triggers.push((name, def));
                }
                Stmt::Drop { name } => {
                    if st.fk_on {
                        // DROP parent while child rows still reference it -> rc 19
                        for (cn, ct) in &st.tables {
                            if *cn == name { continue; }
                            for (ci, col) in ct.cols.iter().enumerate() {
                                if let Some((p, _pc, _)) = &col.references {
                                    if *p == name
                                        && ct.rows.iter().any(|(_, r)| r.get(ci).map_or(false, |v| *v != Val::Null))
                                    {
                                        return Err(FK_ERR.into());
                                    }
                                }
                            }
                        }
                    }
                    st.tables.retain(|(n, _)| *n != name);
                    let idx: Vec<String> = st.index_owner.iter()
                        .filter(|(_, t)| **t == name).map(|(i, _)| i.clone()).collect();
                    st.catalog.retain(|(ty, n)| !(ty == "table" && *n == name)
                        && !(ty == "index" && idx.contains(n)));
                    for i in idx { st.index_owner.remove(&i); }
                }
                Stmt::RenameTable { from, to } => {
                    let t = st.tables.iter_mut().find(|(n, _)| *n == from).ok_or("no such table")?;
                    t.0 = to.clone();
                    for e in st.catalog.iter_mut() {
                        if e.0 == "table" && e.1 == from { e.1 = to.clone(); }
                    }
                }
                Stmt::AddColumn { table, col, default } => {
                    let t = st.tables.iter_mut().find(|(n, _)| *n == table).ok_or("no such table")?;
                    let d = default.clone().unwrap_or(Val::Null);
                    t.1.cols.push(Col { name: col, default, ..Default::default() });
                    for (_, r) in t.1.rows.iter_mut() { r.push(d.clone()); }
                }
                Stmt::Insert { name, collist, rows, policy, upd_col } => {
                    // FK pre-check (immediate, insert-time)
                    let fk_on = st.fk_on;
                    // borrow dance: gather parent existence checks first
                    let mut pending: Vec<Vec<Val>> = Vec::new();
                    {
                        let (cols_meta, _) = {
                            let t = st.tables.iter().find(|(n, _)| *n == name).ok_or("no such table")?;
                            (t.1.cols.clone(), ())
                        };
                        for r in &rows {
                            // map through collist / defaults to full-width row
                            let full: Vec<Val> = match &collist {
                                None => r.clone(),
                                Some(cl) => cols_meta.iter().map(|c| {
                                    match cl.iter().position(|n| *n == c.name) {
                                        Some(i) => r[i].clone(),
                                        None => c.default.clone().unwrap_or(Val::Null),
                                    }
                                }).collect(),
                            };
                            if fk_on {
                                for (ci, col) in cols_meta.iter().enumerate() {
                                    if let Some((p, pc, _)) = &col.references {
                                        let v = full.get(ci).cloned().unwrap_or(Val::Null);
                                        if v != Val::Null {
                                            let parent = st.tables.iter().find(|(n, _)| n == p)
                                                .ok_or(FK_ERR.to_string())?;
                                            let pci = parent.1.cols.iter().position(|c| c.name == *pc)
                                                .ok_or(FK_ERR.to_string())?;
                                            if !parent.1.rows.iter().any(|(_, pr)| pr.get(pci) == Some(&v)) {
                                                return Err(FK_ERR.into());
                                            }
                                        }
                                    }
                                }
                            }
                            pending.push(full);
                        }
                    }
                    let ti = st.tables.iter().position(|(n, _)| *n == name).ok_or("no such table")?;
                    let mut inserted: Vec<(String, Vec<Val>)> = Vec::new(); // for triggers
                    let mut n_changes = 0i64;
                    {
                        let t = &mut st.tables[ti].1;
                        for full in pending {
                            match conflict_row(t, &full) {
                                Some(pos) => match policy {
                                    Policy::Abort => return Err(UNIQ_ERR.into()),
                                    Policy::Ignore | Policy::DoNothing => continue,
                                    Policy::Replace => {
                                        t.rows.remove(pos);
                                        t.next_rowid += 1;
                                        let rid = t.next_rowid;
                                        t.rows.push((rid, full.clone()));
                                        n_changes += 1;
                                        inserted.push((name.clone(), full));
                                    }
                                    Policy::DoUpdate => {
                                        let uc = upd_col.as_ref().ok_or("bad upsert")?;
                                        let ci = t.cols.iter().position(|c| c.name == *uc)
                                            .ok_or("no such column")?;
                                        let newv = full[ci].clone(); // excluded.<uc>
                                        t.rows[pos].1[ci] = newv;
                                        n_changes += 1;
                                    }
                                },
                                None => {
                                    t.next_rowid += 1;
                                    let rid = t.next_rowid;
                                    t.rows.push((rid, full.clone()));
                                    n_changes += 1;
                                    inserted.push((name.clone(), full));
                                }
                            }
                        }
                    }
                    st.changes = n_changes;
                    st.total_changes += n_changes;
                    // AFTER INSERT triggers (single-level, no recursion — toy)
                    let trigs: Vec<Trigger> = st.triggers.iter()
                        .filter(|(_, d)| d.table == name).map(|(_, d)| d.clone()).collect();
                    for (_tn, full) in &inserted {
                        for tg in &trigs {
                            let src_ci = {
                                let t = &st.tables[ti].1;
                                t.cols.iter().position(|c| c.name == tg.body_col)
                                    .ok_or("no such column")?
                            };
                            let v = match &full[src_ci] {
                                Val::Int(i) => Val::Int(i * tg.body_mult),
                                v => v.clone(),
                            };
                            let tt = st.tables.iter_mut().find(|(n, _)| *n == tg.body_target)
                                .ok_or("no such table")?;
                            tt.1.next_rowid += 1;
                            let rid = tt.1.next_rowid;
                            tt.1.rows.push((rid, vec![v]));
                            st.total_changes += 1;
                        }
                    }
                }
                Stmt::Update { name, col, add, set, wh } => {
                    let t = st.tables.iter_mut().find(|(n, _)| *n == name).ok_or("no such table")?;
                    let ci = t.1.cols.iter().position(|c| c.name == col).ok_or("no such column")?;
                    let wi = wh.as_ref().and_then(|(wc, _)| t.1.cols.iter().position(|c| c.name == *wc));
                    let mut n = 0;
                    for (_, row) in t.1.rows.iter_mut() {
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
                    // collect deleted parent key values for cascade
                    let (deleted_keys, n): (Vec<(String, Vec<Val>)>, i64) = {
                        let t = st.tables.iter_mut().find(|(n, _)| *n == name).ok_or("no such table")?;
                        let wi = wh.as_ref().and_then(|(wc, _)| t.1.cols.iter().position(|c| c.name == *wc));
                        let before = t.1.rows.len();
                        let mut deleted: Vec<Vec<Val>> = Vec::new();
                        t.1.rows.retain(|(_, r)| {
                            let hit = match (&wh, wi) {
                                (Some((_, wv)), Some(wi)) => r[wi] == Val::Int(*wv),
                                _ => true,
                            };
                            if hit { deleted.push(r.clone()); }
                            !hit
                        });
                        let cols: Vec<String> = t.1.cols.iter().map(|c| c.name.clone()).collect();
                        (deleted.into_iter().map(|r| (cols.join(","), r)).collect(),
                         (before - t.1.rows.len()) as i64)
                    };
                    st.changes = n;
                    st.total_changes += n;
                    if st.fk_on && n > 0 {
                        // ON DELETE CASCADE (toy): remove child rows whose fk value matched a deleted parent key
                        let parent_cols: Vec<String> = deleted_keys.first()
                            .map(|(c, _)| c.split(',').map(|s| s.to_string()).collect()).unwrap_or_default();
                        let child_specs: Vec<(usize, usize, usize, bool)> = st.tables.iter().enumerate()
                            .flat_map(|(tix, (tn, tt))| {
                                if *tn == name { return Vec::new(); }
                                tt.cols.iter().enumerate().filter_map(|(ci, c)| {
                                    c.references.as_ref().and_then(|(p, pc, casc)| {
                                        if *p == name {
                                            parent_cols.iter().position(|x| x == pc).map(|pci| (tix, ci, pci, *casc))
                                        } else { None }
                                    })
                                }).collect::<Vec<_>>()
                            }).collect();
                        for (tix, ci, pci, casc) in child_specs {
                            let dead: Vec<Val> = deleted_keys.iter().map(|(_, r)| r[pci].clone()).collect();
                            let child = &mut st.tables[tix].1;
                            if casc {
                                let before = child.rows.len();
                                child.rows.retain(|(_, r)| !dead.contains(&r[ci]));
                                st.total_changes += (before - child.rows.len()) as i64;
                            } else if child.rows.iter().any(|(_, r)| dead.contains(&r[ci])) {
                                return Err(FK_ERR.into()); // RESTRICT-style (immediate)
                            }
                        }
                    }
                }
                Stmt::Select { items, target, wh, order_by } => {
                    if target == "sqlite_master" {
                        let cnt = st.catalog.iter().filter(|(ty, n)| match &wh {
                            Some((k, v)) if k == "name" => n == v,
                            Some((k, v)) if k == "type" => ty == v,
                            _ => true,
                        }).count();
                        out.push(vec![Some(cnt.to_string())]);
                        continue;
                    }
                    let t = st.tables.iter().find(|(n, _)| *n == target).ok_or("no such table")?;
                    let mut rows: Vec<&(i64, Vec<Val>)> = t.1.rows.iter().collect();
                    if let Some(ob) = &order_by {
                        let oi = t.1.cols.iter().position(|c| c.name == *ob).ok_or("no such column")?;
                        rows.sort_by_key(|(_, r)| match &r[oi] { Val::Int(i) => *i, _ => 0 });
                    }
                    let aggregate = items.iter().any(|i| i == "count(*)");
                    let render_item = |it: &str, rid: i64, r: &Vec<Val>| -> Result<Option<String>, String> {
                        let base = it.strip_prefix(&format!("{target}.")).unwrap_or(it);
                        if base == "changes()" { return Ok(Some(st.changes.to_string())); }
                        if base == "total_changes()" { return Ok(Some(st.total_changes.to_string())); }
                        if base == "count(*)" { return Ok(Some(t.1.rows.len().to_string())); }
                        if base == "rowid" { return Ok(Some(rid.to_string())); }
                        let ci = t.1.cols.iter().position(|c| c.name == base)
                            .ok_or("no such column".to_string())?;
                        Ok(r[ci].render())
                    };
                    if aggregate {
                        // toy mirror of SQLite's bare-column-in-aggregate: values from the first row
                        let (rid, r) = rows.first().map(|(a, b)| (*a, b.clone()))
                            .unwrap_or((0, vec![Val::Null; t.1.cols.len()]));
                        let mut orow = Vec::new();
                        for it in &items { orow.push(render_item(it, rid, &r)?); }
                        out.push(orow);
                    } else {
                        for (rid, r) in rows {
                            let mut orow = Vec::new();
                            for it in &items { orow.push(render_item(it, *rid, r)?); }
                            out.push(orow);
                        }
                    }
                }
            }
        }
        Ok(())
    });
    match res {
        Ok(()) => Outcome::Done { rows: out, rc: 0, err: None },
        Err(e) => {
            let rc = if e == FK_ERR || e == UNIQ_ERR { 19 } else { 1 };
            Outcome::Done { rows: Vec::new(), rc, err: Some(e) }
        }
    }
}
