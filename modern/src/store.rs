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

use crate::dbfile::{self, DbImage, TableImage, TriggerImage};
use crate::eval;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

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
    not_null: bool,
    check: Option<String>, // CHECK(<expr>) — evaluated for real via eval::eval_standalone
    default: Option<Val>,
    references: Option<(String, String, u8, u8)>, // (parent, pcol, on-delete, on-update): 0=none 1=CASCADE 2=SET NULL 3=RESTRICT 4=SET DEFAULT
}

#[derive(Default)]
struct Table {
    cols: Vec<Col>,
    rows: Vec<(i64, Vec<Val>)>, // (rowid, values)
    next_rowid: i64,
    create_sql: String, // raw CREATE TABLE text (for durable schema)
}

#[derive(Clone)]
struct Trigger {
    table: String,               // ON <table>
    timing: u8,                  // 0=BEFORE 1=AFTER 2=INSTEAD OF
    event: u8,                   // 0=INSERT 1=UPDATE 2=DELETE
    of_col: Option<String>,      // UPDATE OF <col> restriction
    when: Option<String>,        // WHEN <expr> (evaluated for real)
    body: Vec<(String, Vec<String>)>, // INSERT INTO <target> VALUES(<exprs using old./new.>)
    raw: String,                 // raw CREATE TRIGGER text (for durable schema)
}

#[derive(Default)]
pub struct Store {
    pub conn: eval::Conn,
    pub views: std::collections::HashMap<String, String>, // view name -> SELECT body
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

fn val_to_ev(v: &Val) -> eval::V {
    match v { Val::Null => eval::V::Null, Val::Int(i) => eval::V::Int(*i), Val::Text(t) => eval::V::Text(t.clone()) }
}

thread_local! {
    static STORES: RefCell<HashMap<usize, Store>> = RefCell::new(HashMap::new());
    static PATHS: RefCell<HashMap<usize, PathBuf>> = RefCell::new(HashMap::new());
}

/// File-backed open: record the path and, if the file already holds a SQLite DB,
/// load its tables + FK metadata + triggers into this connection's store.
pub fn open_file(db: usize, path: &str) {
    with_store(db, |st| st.conn.is_file = true);
    let pb = PathBuf::from(path);
    PATHS.with(|m| { m.borrow_mut().insert(db, pb.clone()); });
    if let Ok(img) = dbfile::read_db(&pb) {
        with_store(db, |st| {
            for ti in img.tables {
                let inner_open = ti.sql.find('(');
                let inner_close = ti.sql.rfind(')');
                let cols = match (inner_open, inner_close) {
                    (Some(o), Some(c)) if c > o => parse_coldefs(&ti.sql[o + 1..c]).unwrap_or_default(),
                    _ => ti.sql.is_empty().then(Vec::new).unwrap_or_default(),
                };
                let cols = if cols.is_empty() {
                    ti.rows.first().map(|(_, r)| (0..r.len()).map(|i| Col { name: format!("c{i}"), ..Default::default() }).collect()).unwrap_or_default()
                } else { cols };
                let mut tab = Table { cols, create_sql: ti.sql.clone(), ..Default::default() };
                let mut maxr = 0i64;
                for (rid, vals) in ti.rows { if rid > maxr { maxr = rid; } tab.rows.push((rid, vals)); }
                tab.next_rowid = maxr;
                st.catalog.push(("table".into(), ti.name.clone()));
                st.tables.push((ti.name, tab));
            }
            for tg in img.triggers {
                if let Some(Stmt::CreateTrigger { name, def, .. }) = parse_stmt(&tg.sql) {
                    st.catalog.push(("trigger".into(), name.clone()));
                    st.triggers.push((name, def));
                }
            }
        });
    }
}

pub fn drop_store(db: usize) {
    STORES.with(|m| { m.borrow_mut().remove(&db); });
}

/// Persist a file-backed connection to a real (C-readable) SQLite database file.
/// Durability limit (pack v7): column-UNIQUE constraints are stripped from the
/// persisted table sql (rows are already de-duplicated in-session; no on-disk
/// autoindex is built). IPK/FK/REFERENCES and triggers ARE persisted.
pub fn save_file(db: usize) {
    let path = PATHS.with(|m| m.borrow().get(&db).cloned());
    if let Some(pb) = path {
        with_store(db, |st| {
            let tables: Vec<TableImage> = st.tables.iter().map(|(n, t)| {
                let sql = if t.create_sql.is_empty() {
                    format!("CREATE TABLE {}({})", n, t.cols.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(","))
                } else {
                    sanitize_sql(&t.create_sql)
                };
                TableImage { name: n.clone(), sql, rows: t.rows.clone() }
            }).collect();
            let triggers: Vec<TriggerImage> = st.triggers.iter()
                .map(|(n, d)| TriggerImage { name: n.clone(), tbl: d.table.clone(), sql: d.raw.clone() })
                .collect();
            let _ = dbfile::write_db(&pb, &DbImage { tables, triggers });
        });
    }
    PATHS.with(|m| { m.borrow_mut().remove(&db); });
}

/// Strip on-disk-unsupported constraints (standalone UNIQUE) from persisted table
/// sql; keep INTEGER PRIMARY KEY (rowid alias, no autoindex) and REFERENCES (FK).
fn sanitize_sql(sql: &str) -> String {
    let mut out = sql.to_string();
    // remove case-insensitive standalone " UNIQUE" tokens
    loop {
        let up = out.to_ascii_uppercase();
        if let Some(p) = up.find(" UNIQUE") {
            out.replace_range(p..p + " UNIQUE".len(), "");
        } else { break; }
    }
    out
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
    if t.eq_ignore_ascii_case("NULL") {
        return Some(Val::Null);
    }
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
    Create { name: String, cols: Vec<Col>, sql: String },
    CreateIndex { name: String, table: String, col: String, unique: bool },
    CreateTrigger { name: String, def: Trigger, sql: String },
    CreateView { name: String, body: String, sql: String },
    DropView { name: String },
    DropTrigger { name: String },
    RenameColumn { table: String, from: String, to: String },
    DropColumn { table: String, col: String },
    Drop { name: String },
    RenameTable { from: String, to: String },
    AddColumn { table: String, col: String, default: Option<Val> },
    Insert { name: String, collist: Option<Vec<String>>, rows: Vec<Vec<Val>>, policy: Policy,
             upd_col: Option<String>, /* DO UPDATE SET c=excluded.c */
             upd_where: Option<String> /* DO UPDATE ... WHERE <expr> */ },
    Update { name: String, col: String, add: Option<i64>, set: Option<Val>, wh: Option<(String, i64)> },
    Delete { name: String, wh: Option<(String, i64)> },
    Select { items: Vec<String>, target: String, wh: Option<(String, Val)>, order_by: Option<String> },
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
        if up.contains("NOT NULL") { col.not_null = true; }
        if let Some(cp) = up.find("CHECK") {
            let rest = &d[cp + 5..];
            let open = rest.find('(')?;
            let mut depth = 0; let mut end = None;
            for (i, ch) in rest.char_indices().skip(open) {
                match ch { '(' => depth += 1, ')' => { depth -= 1; if depth == 0 { end = Some(i); break; } }, _ => {} }
            }
            col.check = Some(rest[open + 1..end?].trim().to_string());
        }
        if let Some(rp) = up.find("REFERENCES ") {
            let rest = &d[rp + "REFERENCES ".len()..];
            let open = rest.find('(')?;
            let parent = ident(&rest[..open])?;
            let close = rest.find(')')?;
            let pcol = ident(&rest[open + 1..close])?;
            let act = |kw: &str| -> u8 {
                if up.contains(&format!("{kw} CASCADE")) { 1 }
                else if up.contains(&format!("{kw} SET NULL")) { 2 }
                else if up.contains(&format!("{kw} RESTRICT")) { 3 }
                else if up.contains(&format!("{kw} SET DEFAULT")) { 4 } else { 0 }
            };
            col.references = Some((parent, pcol, act("ON DELETE"), act("ON UPDATE")));
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
        return Some(Stmt::Create { name, cols: parse_coldefs(inner)?, sql: s.trim().to_string() });
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
        // CREATE TRIGGER <n> [BEFORE|AFTER] [INSERT|UPDATE|DELETE] ON <t> [WHEN <e>] BEGIN <INSERT...;>+ END
        let rest = s["CREATE TRIGGER ".len()..].trim();
        let rup = rest.to_ascii_uppercase();
        let hdr = [
            (" BEFORE INSERT ON ", 0u8, 0u8), (" BEFORE UPDATE ON ", 0, 1), (" BEFORE DELETE ON ", 0, 2),
            (" AFTER INSERT ON ", 1, 0), (" AFTER UPDATE ON ", 1, 1), (" AFTER DELETE ON ", 1, 2),
            (" INSTEAD OF INSERT ON ", 2, 0), (" INSTEAD OF UPDATE ON ", 2, 1), (" INSTEAD OF DELETE ON ", 2, 2),
        ];
        let mut of_col: Option<String> = None;
        let (timing, event, kwlen, kp) = match hdr.iter().find_map(|(kw, ti, ev)| rup.find(kw).map(|p| (*ti, *ev, kw.len(), p))) {
            Some(x) => x,
            None => {
                // UPDATE OF <col> forms: "<name> BEFORE|AFTER UPDATE OF <col> ON <table>"
                let (ti, base) = if let Some(p) = rup.find(" BEFORE UPDATE OF ") { (0u8, p) }
                                 else if let Some(p) = rup.find(" AFTER UPDATE OF ") { (1u8, p) }
                                 else { return None; };
                let kw_len = if ti == 0 { " BEFORE UPDATE OF ".len() } else { " AFTER UPDATE OF ".len() };
                let tail2 = &rest[base + kw_len..];
                let onp = tail2.to_ascii_uppercase().find(" ON ")?;
                of_col = Some(ident(&tail2[..onp])?);
                (ti, 1u8, kw_len + onp + 4, base)
            }
        };
        let name = ident(&rest[..kp])?;
        let tail = &rest[kp + kwlen..];
        let tup = tail.to_ascii_uppercase();
        let beg = tup.find(" BEGIN ")?;
        let head = tail[..beg].trim();
        let hup = head.to_ascii_uppercase();
        let (table, when) = match hup.find(" WHEN ") {
            Some(wp) => (ident(&head[..wp])?, Some(head[wp + 6..].trim().to_string())),
            None => (ident(head)?, None),
        };
        let mut bodytxt = tail[beg + 7..].trim();
        bodytxt = bodytxt.strip_suffix("END").unwrap_or(bodytxt).trim_end();
        let mut body = Vec::new();
        for stmt in bodytxt.split(';') {
            let stmt = stmt.trim();
            if stmt.is_empty() { continue; }
            let sup = stmt.to_ascii_uppercase();
            if sup.starts_with("SELECT RAISE(") {
                let inner = &stmt["SELECT RAISE(".len()..stmt.rfind(')')?];
                let msg = inner.split_once(',').map(|(_, m)| m.trim().trim_matches('\'').to_string())
                    .unwrap_or_else(|| "RAISE".into());
                body.push(("#raise".to_string(), vec![format!("'{}'", msg.replace('\'', "''"))]));
                continue;
            }
            if !sup.starts_with("INSERT INTO ") { return None; }
            let after_kw = &stmt["INSERT INTO ".len()..];
            let vpos = after_kw.to_ascii_uppercase().find(" VALUES(").or_else(|| after_kw.to_ascii_uppercase().find(" VALUES ("))?;
            let target = ident(&after_kw[..vpos])?;
            let open = after_kw[vpos..].find('(')? + vpos;
            let exprs_txt = after_kw[open + 1..].trim_end().trim_end_matches(')');
            let mut exprs = Vec::new();
            let mut depth = 0; let mut cur = String::new();
            for ch in exprs_txt.chars() {
                match ch { '(' => { depth += 1; cur.push(ch); } ')' => { depth -= 1; cur.push(ch); }
                          ',' if depth == 0 => { exprs.push(cur.trim().to_string()); cur.clear(); }
                          _ => cur.push(ch) }
            }
            if !cur.trim().is_empty() { exprs.push(cur.trim().to_string()); }
            body.push((target, exprs));
        }
        if body.is_empty() { return None; }
        return Some(Stmt::CreateTrigger { name, def: Trigger { table, timing, event, of_col, when, body, raw: s.trim().to_string() }, sql: s.trim().to_string() });
    }
    if up.starts_with("CREATE VIEW ") {
        let rest = &s["CREATE VIEW ".len()..];
        let ap = rest.to_ascii_uppercase().find(" AS ")?;
        let name = ident(&rest[..ap])?;
        return Some(Stmt::CreateView { name, body: rest[ap + 4..].trim().to_string(), sql: s.trim().to_string() });
    }
    if up.starts_with("DROP VIEW ") {
        return Some(Stmt::DropView { name: ident(&s["DROP VIEW ".len()..])? });
    }
    if up.starts_with("DROP TRIGGER ") {
        return Some(Stmt::DropTrigger { name: ident(&s["DROP TRIGGER ".len()..])? });
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
        if let Some(rc_) = rup.find(" RENAME COLUMN ") {
            let table = ident(&rest[..rc_])?;
            let tail = &rest[rc_ + " RENAME COLUMN ".len()..];
            let top = tail.to_ascii_uppercase().find(" TO ")?;
            return Some(Stmt::RenameColumn { table, from: ident(&tail[..top])?, to: ident(&tail[top + 4..])? });
        }
        if let Some(dc) = rup.find(" DROP COLUMN ") {
            let table = ident(&rest[..dc])?;
            return Some(Stmt::DropColumn { table, col: ident(&rest[dc + " DROP COLUMN ".len()..])? });
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
        } else if up.starts_with("INSERT OR FAIL INTO ") {
            (Policy::Abort, &s["INSERT OR FAIL INTO ".len()..])
        } else if up.starts_with("INSERT OR ABORT INTO ") {
            (Policy::Abort, &s["INSERT OR ABORT INTO ".len()..])
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
        let mut upd_where = None;
        let vup = vals.to_ascii_uppercase();
        if let Some(oc) = vup.find(" ON CONFLICT") {
            let clause = &vals[oc..];
            let cup = clause.to_ascii_uppercase();
            if cup.contains("DO NOTHING") {
                policy = Policy::DoNothing;
            } else if let Some(du) = cup.find("DO UPDATE SET ") {
                policy = Policy::DoUpdate;
                let mut assign = clause[du + "DO UPDATE SET ".len()..].to_string();
                if let Some(wp) = assign.to_ascii_uppercase().find(" WHERE ") {
                    upd_where = Some(assign[wp + 7..].trim().to_string());
                    assign = assign[..wp].to_string();
                }
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
        return Some(Stmt::Insert { name, collist, rows, policy, upd_col, upd_where });
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
            if order_by.is_none() { return None; } // COLLATE/NULLS/multi-key ORDER BY -> evaluator
            rest = rest[..o].trim().to_string();
        }
        let mut wh = None;
        if let Some(w) = rest.to_ascii_uppercase().find(" WHERE ") {
            let cond = rest[w + 7..].trim().to_string();
            let eq = cond.find('=')?;
            let key = ident(&cond[..eq])?; // sqlite_master: name=/type= ; user tables: col=literal
            let v = parse_literal(cond[eq + 1..].trim())?;
            wh = Some((key, v));
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

fn view_colnames(body: &str) -> Vec<String> {
    let up = body.to_ascii_uppercase();
    let start = if up.starts_with("SELECT") { 6 } else { 0 };
    let end = up.find(" FROM ").unwrap_or(body.len());
    body[start..end].split(',').map(|item| {
        let it = item.trim();
        let iu = it.to_ascii_uppercase();
        if let Some(p) = iu.rfind(" AS ") { it[p+4..].trim().to_string() }
        else { it.rsplit('.').next().unwrap_or(it).trim().to_string() }
    }).collect()
}

fn ev_truthy(v: &eval::V) -> bool {
    match v { eval::V::Null => false, eval::V::Int(i) => *i != 0, eval::V::Real(r) => *r != 0.0,
              eval::V::Text(t) => t.parse::<f64>().map(|f| f != 0.0).unwrap_or(false), eval::V::Blob(_) => true }
}
fn ev_to_val(v: eval::V) -> Val {
    match v { eval::V::Null => Val::Null, eval::V::Int(i) => Val::Int(i), eval::V::Real(r) => Val::Int(r as i64),
              eval::V::Text(t) => Val::Text(t), eval::V::Blob(b) => Val::Text(String::from_utf8_lossy(&b).into_owned()) }
}
/// Fire matching triggers for one row event. WHEN + body value expressions are
/// evaluated for real (eval::eval_standalone) against old.*/new.* bindings.
fn fire_triggers(st: &mut Store, table: &str, timing: u8, event: u8,
                 old: Option<&Vec<Val>>, new: Option<&Vec<Val>>) -> Result<(), String> {
    fire_triggers_d(st, table, timing, event, old, new, None, 0)
}
fn fire_triggers_d(st: &mut Store, table: &str, timing: u8, event: u8,
                 old: Option<&Vec<Val>>, new: Option<&Vec<Val>>,
                 upd_col: Option<&str>, depth: u32) -> Result<(), String> {
    if depth > 16 { return Err("too many levels of trigger recursion".into()); }
    let trigs: Vec<Trigger> = st.triggers.iter()
        .filter(|(_, d)| d.table == table && d.timing == timing && d.event == event)
        .filter(|(_, d)| match (&d.of_col, upd_col) {
            (Some(oc), Some(uc)) => oc == uc,
            (Some(_), None) => event != 1, // UPDATE OF requires a matching updated column
            _ => true,
        })
        .map(|(_, d)| d.clone()).collect();
    if trigs.is_empty() { return Ok(()); }
    let colnames: Vec<String> = st.tables.iter().find(|(n, _)| n == table)
        .map(|(_, t)| t.cols.iter().map(|c| c.name.clone()).collect()).unwrap_or_default();
    let mut env: std::collections::HashMap<String, eval::V> = Default::default();
    if let Some(o) = old { for (i, c) in colnames.iter().enumerate() { env.insert(format!("old.{c}"), val_to_ev(o.get(i).unwrap_or(&Val::Null))); } }
    if let Some(nw) = new { for (i, c) in colnames.iter().enumerate() { env.insert(format!("new.{c}"), val_to_ev(nw.get(i).unwrap_or(&Val::Null))); } }
    for tg in trigs {
        if let Some(w) = &tg.when {
            if !ev_truthy(&eval::eval_standalone(w, &env)?) { continue; }
        }
        for (target, exprs) in &tg.body {
            if target == "#raise" {
                let msg = eval::eval_standalone(&exprs[0], &env)?;
                return Err(format!("__RAISE__{}", match msg { eval::V::Text(m) => m, v => v.render().unwrap_or_default() }));
            }
            let mut row = Vec::new();
            for e in exprs { row.push(ev_to_val(eval::eval_standalone(e, &env)?)); }
            {
                let tt = st.tables.iter_mut().find(|(n, _)| n == target).ok_or("no such table")?;
                tt.1.next_rowid += 1;
                let rid = tt.1.next_rowid;
                tt.1.rows.push((rid, row.clone()));
                st.total_changes += 1;
            }
            // PRAGMA recursive_triggers=ON: a trigger-body INSERT re-fires INSERT triggers
            if st.conn.pragmas.get("recursive_triggers").copied().unwrap_or(0) == 1 {
                fire_triggers_d(st, &target.clone(), 0, 0, None, Some(&row), None, depth + 1)?;
                fire_triggers_d(st, &target.clone(), 1, 0, None, Some(&row), None, depth + 1)?;
            }
        }
    }
    Ok(())
}

pub fn execute_script(db: usize, script: &str) -> Outcome {
    let raw_stmts = split_statements(script);
    if raw_stmts.is_empty() { return Outcome::Done { rows: Vec::new(), rc: 0, err: None }; }
    let mut out: Vec<Vec<Option<String>>> = Vec::new();
    let res: Result<(), String> = with_store(db, |st| {
        for s in &raw_stmts {
            let parsed = parse_stmt(s);
            // a kitchen SELECT only counts if its table actually lives in this store;
            // otherwise (pragma_* projections, TVFs) it belongs to the evaluator
            let kitchen_ok = match &parsed {
                Some(Stmt::Select { target, .. }) =>
                    target == "sqlite_master" || st.tables.iter().any(|(n, _)| n == target),
                Some(_) => true,
                None => false,
            };
            let stmt = match parsed {
                Some(st2) if kitchen_ok => st2,
                _ => {
                    // not a kitchen statement -> real expression/pragma/attach evaluator (pack v8)
                    let snap: std::collections::HashMap<String, (Vec<String>, Vec<Vec<eval::V>>)> =
                        st.tables.iter().map(|(n, t)| (n.clone(),
                            (t.cols.iter().map(|c| c.name.clone()).collect(),
                             t.rows.iter().map(|(_, r)| r.iter().map(val_to_ev).collect()).collect()))).collect();
                    let fk: std::collections::HashMap<String, usize> = st.tables.iter()
                        .map(|(n, t)| (n.clone(), t.cols.iter().filter(|c| c.references.is_some()).count())).collect();
                    let idx: std::collections::HashMap<String, usize> = {
                        let mut m = std::collections::HashMap::new();
                        for (_i, tn) in st.index_owner.values().map(|t| (0, t.clone())) { *m.entry(tn).or_insert(0) += 1; }
                        for (n, _) in &st.tables { m.entry(n.clone()).or_insert(0); }
                        m
                    };
                    let mut ctx = eval::Ctx { conn: &mut st.conn, tables: &snap, fk_counts: &fk, index_counts: &idx, views: &st.views };
                    match eval::run_stmt(&mut ctx, s) {
                        Ok(Some(rows)) => { out.extend(rows); continue; }
                        Ok(None) => return Err(format!("unsupported statement: {}", s)),
                        Err(e) => return Err(e),
                    }
                }
            };
            match stmt {
                Stmt::PragmaFkOn => { st.fk_on = true; st.conn.pragmas.insert("foreign_keys".into(), 1); }
                Stmt::Create { name, cols, sql } => {
                    st.catalog.push(("table".into(), name.clone()));
                    st.tables.push((name, Table { cols, create_sql: sql, ..Default::default() }));
                    st.conn.schema_version += 1;
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
                Stmt::CreateTrigger { name, def, sql: _ } => {
                    st.catalog.push(("trigger".into(), name.clone()));
                    st.triggers.push((name, def));
                }
                Stmt::CreateView { name, body, sql: _ } => {
                    st.views.insert(name.clone(), body);
                    st.catalog.push(("view".into(), name));
                    st.conn.schema_version += 1;
                }
                Stmt::RenameColumn { table, from, to } => {
                    let t = st.tables.iter_mut().find(|(n, _)| *n == table).ok_or("no such table")?;
                    let c = t.1.cols.iter_mut().find(|c| c.name == from).ok_or("no such column")?;
                    c.name = to;
                    st.conn.schema_version += 1;
                }
                Stmt::DropColumn { table, col } => {
                    let t = st.tables.iter_mut().find(|(n, _)| *n == table).ok_or("no such table")?;
                    let ci = t.1.cols.iter().position(|c| c.name == col).ok_or("no such column")?;
                    t.1.cols.remove(ci);
                    for (_, r) in t.1.rows.iter_mut() { if ci < r.len() { r.remove(ci); } }
                    st.conn.schema_version += 1;
                }
                Stmt::DropTrigger { name } => {
                    st.triggers.retain(|(n, _)| *n != name);
                    st.catalog.retain(|(ty, n)| !(ty == "trigger" && *n == name));
                    st.conn.schema_version += 1;
                }
                Stmt::DropView { name } => {
                    st.views.remove(&name);
                    st.catalog.retain(|(ty, n)| !(ty == "view" && *n == name));
                    st.conn.schema_version += 1;
                }
                Stmt::Drop { name } => {
                    if st.fk_on {
                        // DROP parent while child rows still reference it -> rc 19
                        for (cn, ct) in &st.tables {
                            if *cn == name { continue; }
                            for (ci, col) in ct.cols.iter().enumerate() {
                                if let Some((p, _pc, _, _)) = &col.references {
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
                    st.conn.schema_version += 1;
                    let idx: Vec<String> = st.index_owner.iter()
                        .filter(|(_, t)| **t == name).map(|(i, _)| i.clone()).collect();
                    st.catalog.retain(|(ty, n)| !(ty == "table" && *n == name)
                        && !(ty == "index" && idx.contains(n)));
                    for i in idx { st.index_owner.remove(&i); }
                }
                Stmt::RenameTable { from, to } => {
                    let t = st.tables.iter_mut().find(|(n, _)| *n == from).ok_or("no such table")?;
                    t.0 = to.clone();
                    t.1.create_sql = format!("CREATE TABLE {}({})", to,
                        t.1.cols.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(","));
                    for e in st.catalog.iter_mut() {
                        if e.0 == "table" && e.1 == from { e.1 = to.clone(); }
                    }
                }
                Stmt::AddColumn { table, col, default } => {
                    let t = st.tables.iter_mut().find(|(n, _)| *n == table).ok_or("no such table")?;
                    let d = default.clone().unwrap_or(Val::Null);
                    t.1.cols.push(Col { name: col, default, ..Default::default() });
                    for (_, r) in t.1.rows.iter_mut() { r.push(d.clone()); }
                    t.1.create_sql = format!("CREATE TABLE {}({})", table,
                        t.1.cols.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(","));
                }
                Stmt::Insert { name, collist, rows, policy, upd_col, upd_where } => {
                    if st.views.contains_key(&name) {
                        // INSTEAD OF INSERT triggers make views writable
                        let has_instead = st.triggers.iter().any(|(_, d)| d.table == name && d.timing == 2 && d.event == 0);
                        if !has_instead {
                            return Err(format!("cannot modify {name} because it is a view"));
                        }
                        let vcols = view_colnames(&st.views[&name]);
                        let trigs: Vec<Trigger> = st.triggers.iter()
                            .filter(|(_, d)| d.table == name && d.timing == 2 && d.event == 0)
                            .map(|(_, d)| d.clone()).collect();
                        for r in &rows {
                            let mut env: std::collections::HashMap<String, eval::V> = Default::default();
                            for (i, c) in vcols.iter().enumerate() {
                                env.insert(format!("new.{c}"), val_to_ev(r.get(i).unwrap_or(&Val::Null)));
                            }
                            for tg in &trigs {
                                if let Some(w) = &tg.when {
                                    if !ev_truthy(&eval::eval_standalone(w, &env)?) { continue; }
                                }
                                for (target, exprs) in &tg.body {
                                    if target == "#raise" {
                                        let msg = eval::eval_standalone(&exprs[0], &env)?;
                                        return Err(format!("__RAISE__{}", msg.render().unwrap_or_default()));
                                    }
                                    let mut row = Vec::new();
                                    for e in exprs { row.push(ev_to_val(eval::eval_standalone(e, &env)?)); }
                                    let tt = st.tables.iter_mut().find(|(n, _)| n == target).ok_or("no such table")?;
                                    tt.1.next_rowid += 1;
                                    let rid = tt.1.next_rowid;
                                    tt.1.rows.push((rid, row));
                                    st.total_changes += 1;
                                }
                            }
                        }
                        continue;
                    }
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
                            // NOT NULL + CHECK constraints (evaluated for real)
                            let mut viol: Option<String> = None;
                            for (ci, col) in cols_meta.iter().enumerate() {
                                let v = full.get(ci).cloned().unwrap_or(Val::Null);
                                if col.not_null && v == Val::Null {
                                    viol = Some(format!("NOT NULL constraint failed: {}.{}", name, col.name));
                                    break;
                                }
                                if let Some(chk) = &col.check {
                                    let mut env: std::collections::HashMap<String, eval::V> = Default::default();
                                    for (cj, cc) in cols_meta.iter().enumerate() {
                                        env.insert(cc.name.clone(), val_to_ev(full.get(cj).unwrap_or(&Val::Null)));
                                    }
                                    let r = eval::eval_standalone(chk, &env)?;
                                    // NULL result passes a CHECK (SQL semantics); false fails
                                    if !matches!(r, eval::V::Null) && !ev_truthy(&r) {
                                        viol = Some(format!("CHECK constraint failed: {}", name));
                                        break;
                                    }
                                }
                            }
                            if let Some(msg) = viol {
                                if matches!(policy, Policy::Ignore | Policy::DoNothing) { continue; }
                                return Err(msg);
                            }
                            if fk_on {
                                for (ci, col) in cols_meta.iter().enumerate() {
                                    if let Some((p, pc, _, _)) = &col.references {
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
                    for full in &pending {
                        fire_triggers(st, &name, 0, 0, None, Some(full))?; // BEFORE INSERT
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
                                        // optional DO UPDATE ... WHERE: evaluated with excluded.* + existing row
                                        if let Some(w) = &upd_where {
                                            let mut env: std::collections::HashMap<String, eval::V> = Default::default();
                                            for (cj, cc) in t.cols.iter().enumerate() {
                                                env.insert(format!("excluded.{}", cc.name), val_to_ev(full.get(cj).unwrap_or(&Val::Null)));
                                                env.insert(cc.name.clone(), val_to_ev(t.rows[pos].1.get(cj).unwrap_or(&Val::Null)));
                                            }
                                            if !ev_truthy(&eval::eval_standalone(w, &env)?) { continue; }
                                        }
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
                    for (_tn, full) in &inserted {
                        fire_triggers(st, &name, 1, 0, None, Some(full))?; // AFTER INSERT
                    }
                }
                Stmt::Update { name, col, add, set, wh } => {
                    if st.views.contains_key(&name) {
                        return Err(format!("cannot modify {name} because it is a view"));
                    }
                    // plan updates first so BEFORE/AFTER UPDATE triggers can fire per row
                    let planned: Vec<(usize, Vec<Val>, Vec<Val>)> = {
                        let t = st.tables.iter().find(|(n, _)| *n == name).ok_or("no such table")?;
                        let ci = t.1.cols.iter().position(|c| c.name == col).ok_or("no such column")?;
                        let wi = wh.as_ref().and_then(|(wc, _)| t.1.cols.iter().position(|c| c.name == *wc));
                        t.1.rows.iter().enumerate().filter_map(|(ri, (_, row))| {
                            if let (Some((_, wv)), Some(wi)) = (&wh, wi) {
                                if row[wi] != Val::Int(*wv) { return None; }
                            }
                            let mut newr = row.clone();
                            newr[ci] = match (&add, &set) {
                                (Some(d), _) => match &row[ci] { Val::Int(i) => Val::Int(i + d), v => v.clone() },
                                (None, Some(v)) => v.clone(),
                                _ => row[ci].clone(),
                            };
                            Some((ri, row.clone(), newr))
                        }).collect()
                    };
                    for (_, o, nw) in &planned { fire_triggers_d(st, &name, 0, 1, Some(o), Some(nw), Some(&col), 0)?; }
                    {
                        let t = st.tables.iter_mut().find(|(n, _)| *n == name).ok_or("no such table")?;
                        for (ri, _, nw) in &planned { t.1.rows[*ri].1 = nw.clone(); }
                    }
                    let n = planned.len() as i64;
                    st.changes = n;
                    st.total_changes += n;
                    // FK ON UPDATE actions: propagate parent-key changes to children
                    if st.fk_on && !planned.is_empty() {
                        let ci = st.tables.iter().find(|(n2, _)| *n2 == name)
                            .and_then(|(_, t)| t.cols.iter().position(|c| c.name == col));
                        if let Some(ci) = ci {
                            let pcol_name = col.clone();
                            let changes: Vec<(Val, Val)> = planned.iter()
                                .map(|(_, o, nw)| (o[ci].clone(), nw[ci].clone()))
                                .filter(|(o, nw)| o != nw).collect();
                            if !changes.is_empty() {
                                let specs: Vec<(usize, usize, u8)> = st.tables.iter().enumerate()
                                    .flat_map(|(tix, (tn, tt))| {
                                        if *tn == name { return Vec::new(); }
                                        tt.cols.iter().enumerate().filter_map(|(cci, c)| {
                                            c.references.as_ref().and_then(|(p, pc, _, ua)| {
                                                if *p == name && *pc == pcol_name { Some((tix, cci, *ua)) } else { None }
                                            })
                                        }).collect::<Vec<_>>()
                                    }).collect();
                                for (tix, cci, ua) in specs {
                                    let child = &mut st.tables[tix].1;
                                    for (oldv, newv) in &changes {
                                        match ua {
                                            1 => { for (_, r) in child.rows.iter_mut() { if r[cci] == *oldv { r[cci] = newv.clone(); } } }
                                            2 => { for (_, r) in child.rows.iter_mut() { if r[cci] == *oldv { r[cci] = Val::Null; } } }
                                            4 => { let dv = child.cols[cci].default.clone().unwrap_or(Val::Null);
                                                   for (_, r) in child.rows.iter_mut() { if r[cci] == *oldv { r[cci] = dv.clone(); } } }
                                            _ => { if child.rows.iter().any(|(_, r)| r[cci] == *oldv) { return Err(FK_ERR.into()); } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    for (_, o, nw) in &planned { fire_triggers_d(st, &name, 1, 1, Some(o), Some(nw), Some(&col), 0)?; }
                }
                Stmt::Delete { name, wh } => {
                    if st.views.contains_key(&name) {
                        return Err(format!("cannot modify {name} because it is a view"));
                    }
                    // pre-compute hit rows so BEFORE DELETE triggers can fire per row
                    let hits: Vec<Vec<Val>> = {
                        let t = st.tables.iter().find(|(n, _)| *n == name).ok_or("no such table")?;
                        let wi = wh.as_ref().and_then(|(wc, _)| t.1.cols.iter().position(|c| c.name == *wc));
                        t.1.rows.iter().filter(|(_, r)| match (&wh, wi) {
                            (Some((_, wv)), Some(wi)) => r[wi] == Val::Int(*wv),
                            _ => true,
                        }).map(|(_, r)| r.clone()).collect()
                    };
                    for o in &hits { fire_triggers(st, &name, 0, 2, Some(o), None)?; }
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
                        let child_specs: Vec<(usize, usize, usize, u8)> = st.tables.iter().enumerate()
                            .flat_map(|(tix, (tn, tt))| {
                                if *tn == name { return Vec::new(); }
                                tt.cols.iter().enumerate().filter_map(|(ci, c)| {
                                    c.references.as_ref().and_then(|(p, pc, action, _)| {
                                        if *p == name {
                                            parent_cols.iter().position(|x| x == pc).map(|pci| (tix, ci, pci, *action))
                                        } else { None }
                                    })
                                }).collect::<Vec<_>>()
                            }).collect();
                        for (tix, ci, pci, action) in child_specs {
                            let dead: Vec<Val> = deleted_keys.iter().map(|(_, r)| r[pci].clone()).collect();
                            let child = &mut st.tables[tix].1;
                            match action {
                                1 => { // ON DELETE CASCADE
                                    let before = child.rows.len();
                                    child.rows.retain(|(_, r)| !dead.contains(&r[ci]));
                                    st.total_changes += (before - child.rows.len()) as i64;
                                }
                                2 => { // ON DELETE SET NULL
                                    for (_, r) in child.rows.iter_mut() {
                                        if dead.contains(&r[ci]) { r[ci] = Val::Null; }
                                    }
                                }
                                4 => { // ON DELETE SET DEFAULT
                                    let dv = child.cols[ci].default.clone().unwrap_or(Val::Null);
                                    for (_, r) in child.rows.iter_mut() {
                                        if dead.contains(&r[ci]) { r[ci] = dv.clone(); }
                                    }
                                }
                                _ => { // no action / RESTRICT: immediate violation
                                    if child.rows.iter().any(|(_, r)| dead.contains(&r[ci])) {
                                        return Err(FK_ERR.into());
                                    }
                                }
                            }
                        }
                    }
                    for o in &hits { fire_triggers(st, &name, 1, 2, Some(o), None)?; }
                }
                Stmt::Select { items, target, wh, order_by } => {
                    if target == "sqlite_master" {
                        let cnt = st.catalog.iter().filter(|(ty, n)| match &wh {
                            Some((k, v)) if k == "name" => Some(n.clone()) == v.render(),
                            Some((k, v)) if k == "type" => Some(ty.clone()) == v.render(),
                            _ => true,
                        }).count();
                        out.push(vec![Some(cnt.to_string())]);
                        continue;
                    }
                    let t = st.tables.iter().find(|(n, _)| *n == target).ok_or("no such table")?;
                    // eager name resolution (C reports unknown columns at prepare time)
                    for it in &items {
                        let base = it.strip_prefix(&format!("{target}.")).unwrap_or(it);
                        let special = base == "count(*)" || base == "changes()" || base == "total_changes()" || base == "rowid" || base == "*";
                        if !special && !t.1.cols.iter().any(|c| c.name == *base) {
                            return Err(format!("no such column: {base}"));
                        }
                    }
                    // user-table WHERE col=literal equality filter
                    let wh_ci = match &wh {
                        Some((k, _)) => Some(t.1.cols.iter().position(|c| c.name == *k).ok_or("no such column")?),
                        None => None,
                    };
                    let mut rows: Vec<&(i64, Vec<Val>)> = t.1.rows.iter().filter(|(_, r)| {
                        match (&wh, wh_ci) { (Some((_, v)), Some(ci)) => &r[ci] == v, _ => true }
                    }).collect();
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
            let (rc, msg) = if let Some(m) = e.strip_prefix("__RAISE__") { (19, m.to_string()) }
                else if e == FK_ERR || e == UNIQ_ERR || e.contains("constraint failed") { (19, e) }
                else { (1, e) };
            Outcome::Done { rows: Vec::new(), rc, err: Some(msg) }
        }
    }
}
