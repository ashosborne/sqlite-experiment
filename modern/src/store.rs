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
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
    Null,
}
impl Val {
    fn render(&self) -> Option<String> {
        match self {
            Val::Int(i) => Some(i.to_string()),
            Val::Real(r) => Some(if r.is_finite() && *r == r.trunc() && r.abs() < 1e15 {
                format!("{:.1}", r)
            } else { format!("{}", r) }),
            Val::Text(t) => Some(t.clone()),
            Val::Blob(b) => Some(String::from_utf8_lossy(b).into_owned()),
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
    coll: Option<String>, // declared column collation (COLLATE <name>, stored lowercase)
    ref_deferred: bool,   // REFERENCES ... DEFERRABLE INITIALLY DEFERRED (run-33)
}

#[derive(Default, Clone)]
struct Table {
    cols: Vec<Col>,
    uniq_sets: Vec<Vec<String>>, // table-constraint UNIQUE(a,b,...) column lists
    checks: Vec<String>,         // table-constraint CHECK(<expr>) expressions
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

/// full-store snapshot (pack v15 transaction model: snapshot/undo, NOT a pager
/// journal and NOT WAL — documented plainly; counters are not rolled back, like C's
/// total_changes)
#[derive(Clone)]
pub(crate) struct IndexDef {
    pub name: String,
    pub table: String,
    pub exprs: Vec<String>,          // column names OR expressions (evaluated via eval)
    pub unique: bool,
    pub where_c: Option<String>,     // partial-index predicate
    pub sql: String,                 // raw CREATE INDEX text (persisted)
}

#[derive(Clone, Default)]
struct Snap {
    tables: Vec<(String, Table)>,
    catalog: Vec<(String, String)>,
    views: std::collections::HashMap<String, String>,
    triggers: Vec<(String, Trigger)>,
    indexes: Vec<IndexDef>,
    index_owner: HashMap<String, String>,
    fk_on: bool,
}
#[derive(Default)]
struct Txn {
    snap: Snap,
    implicit: bool, // started by SAVEPOINT (outermost RELEASE commits)
    savepoints: Vec<(String, Snap)>,
}

#[derive(Default)]
pub struct Store {
    pub conn: eval::Conn,
    txn: Option<Txn>,
    mutation_counter: u64,               // invalidates the index probe cache
    index_cache: HashMap<String, (u64, std::collections::BTreeMap<KeyVal, Vec<i64>>)>,
    pub index_probes: u64,               // anti-cheat: real probe counter
    pub views: std::collections::HashMap<String, String>, // view name -> SELECT body
    tables: Vec<(String, Table)>,
    catalog: Vec<(String, String)>, // (type: table|index|trigger, name) — creation order
    index_owner: HashMap<String, String>, // index name -> table
    indexes: Vec<IndexDef>, // explicit CREATE [UNIQUE] INDEX definitions
    triggers: Vec<(String, Trigger)>,     // (trigger name, def)
    fk_on: bool,
    changes: i64,
    total_changes: i64,
    file_ver_seen: u64,   // run-36: FILE_VERSIONS value this store last loaded/saved
    clean_changes: i64,   // run-36: total_changes at last load/save (no local writes since)
    commit_flush_pending: bool, // run-36: COMMIT ran (marker moved in an earlier exec)
}

pub enum Outcome {
    NotKitchen,
    Done { rows: Vec<Vec<Option<String>>>, rc: i32, err: Option<String> },
}

fn val_to_ev(v: &Val) -> eval::V {
    match v { Val::Null => eval::V::Null, Val::Int(i) => eval::V::Int(*i), Val::Real(r) => eval::V::Real(*r),
              Val::Text(t) => eval::V::Text(t.clone()), Val::Blob(b) => eval::V::Blob(b.clone()) }
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
    // v22: honour a persisted WAL journal mode (header versions == 2) and recover
    // committed frames from an existing -wal before parsing
    let mut buf = std::fs::read(&pb).unwrap_or_default();
    let wal_mode = buf.len() > 19 && buf[18] == 2;
    let walp = wal_sidecar(&pb, "-wal");
    if let Some(overlay) = dbfile::read_wal_overlay(&walp) {
        dbfile::apply_wal_overlay(&mut buf, &overlay);
    }
    if wal_mode || dbfile::read_wal_overlay(&walp).is_some() {
        with_store(db, |st| st.conn.journal = "wal".into());
    }
    if !buf.is_empty() {
        load_image(db, dbfile::read_db_bytes(&buf));
    }
}

thread_local! {
    // run-36: in-process file write locks (path -> owning connection). C's file
    // locking is cross-process; this models the pinned single-process regime only.
    static FILE_LOCKS: RefCell<HashMap<String, usize>> = RefCell::new(HashMap::new());
    // run-36: bumped whenever a connection flushes committed state to a path, so
    // sibling connections on the same file reload before their next statement
    static FILE_VERSIONS: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

fn bump_file_version(db: usize, path: &std::path::Path) {
    let key = path.display().to_string();
    let v = FILE_VERSIONS.with(|m| {
        let mut mm = m.borrow_mut();
        let e = mm.entry(key).or_insert(0);
        *e += 1;
        *e
    });
    with_store(db, |st| {
        st.file_ver_seen = v;
        st.clean_changes = st.total_changes;
    });
}

/// reload the store from the file when a sibling connection committed and this
/// connection has no local writes or open transaction (pinned busy scope)
pub fn maybe_refresh_from_file(db: usize) {
    let path = match PATHS.with(|m| m.borrow().get(&db).cloned()) { Some(p) => p, None => return };
    let key = path.display().to_string();
    let cur = FILE_VERSIONS.with(|m| m.borrow().get(&key).copied().unwrap_or(0));
    let stale = with_store(db, |st| {
        st.conn.is_file && st.txn.is_none()
            && st.file_ver_seen != cur && st.total_changes == st.clean_changes
    });
    if !stale { return; }
    let mut buf = std::fs::read(&path).unwrap_or_default();
    if let Some(overlay) = dbfile::read_wal_overlay(&wal_sidecar(&path, "-wal")) {
        dbfile::apply_wal_overlay(&mut buf, &overlay);
    }
    with_store(db, |st| {
        st.tables.clear();
        st.catalog.clear();
        st.views.clear();
        st.triggers.clear();
        st.indexes.clear();
        st.index_owner.clear();
        st.index_cache.clear();
        st.mutation_counter += 1;
    });
    if !buf.is_empty() {
        load_image(db, dbfile::read_db_bytes(&buf));
    }
    with_store(db, |st| {
        st.file_ver_seen = cur;
        st.clean_changes = st.total_changes;
    });
}

/// acquire (or verify) the write lock for this connection's file, consulting the
/// busy handler / timeout between retries; Err("database is locked") like C.
fn acquire_file_lock(db: usize, hold: bool) -> Result<(), String> {
    let path = match PATHS.with(|m| m.borrow().get(&db).cloned()) { Some(p) => p, None => return Ok(()) };
    let key = path.display().to_string();
    let mut count = 0i32;
    let mut slept = 0i32;
    loop {
        let owner = FILE_LOCKS.with(|m| m.borrow().get(&key).copied());
        match owner {
            None => {
                if hold { FILE_LOCKS.with(|m| { m.borrow_mut().insert(key.clone(), db); }); }
                return Ok(());
            }
            Some(o) if o == db => return Ok(()),
            Some(_) => {
                if !crate::busy_should_retry(db, count, &mut slept) {
                    return Err("database is locked".into());
                }
                count += 1;
            }
        }
    }
}

/// release this connection's file write lock (commit/rollback/close)
pub fn release_file_lock(db: usize) {
    let path = PATHS.with(|m| m.borrow().get(&db).cloned());
    if let Some(p) = path {
        let key = p.display().to_string();
        FILE_LOCKS.with(|m| {
            let mut mm = m.borrow_mut();
            if mm.get(&key) == Some(&db) { mm.remove(&key); }
        });
    }
}

fn wal_sidecar(pb: &std::path::Path, suffix: &str) -> PathBuf {
    PathBuf::from(format!("{}{}", pb.display(), suffix))
}

/// frames currently in the -wal (for wal_checkpoint counts); file-stat only
pub fn wal_frame_count(db: usize) -> i64 {
    let path = PATHS.with(|m| m.borrow().get(&db).cloned());
    match path {
        Some(pb) => {
            let sz = std::fs::metadata(wal_sidecar(&pb, "-wal")).map(|m| m.len()).unwrap_or(0);
            if sz <= 32 { 0 } else { ((sz - 32) / (24 + 4096)) as i64 }
        }
        None => 0,
    }
}

/// (total_changes, schema_version) marker for detecting write activity in one call
pub fn wal_marker(db: usize) -> (i64, i64) {
    with_store(db, |st| (st.total_changes, st.conn.schema_version))
}

/// post-exec / post-step sync of WAL sidecar files (runs OUTSIDE the store borrow):
/// flush committed state to -wal, service pending checkpoints, handle wal->delete.
pub fn wal_sync(db: usize, before: (i64, i64)) {
    let path = match PATHS.with(|m| m.borrow().get(&db).cloned()) { Some(p) => p, None => return };
    let (journal, in_txn_now, pending, marker, committed) = with_store(db, |st| {
        let c = st.commit_flush_pending;
        st.commit_flush_pending = false;
        (st.conn.journal.clone(), st.txn.is_some(), st.conn.pending_ckpt.take(), (st.total_changes, st.conn.schema_version), c)
    });
    let walp = wal_sidecar(&path, "-wal");
    let shmp = wal_sidecar(&path, "-shm");
    if journal == "wal" {
        if (marker != before || committed) && !in_txn_now {
            // commit visibility: full committed image as one WAL transaction
            let img = build_image(db);
            let mut dbbuf = dbfile::write_db_bytes(&img);
            dbfile::set_journal_versions(&mut dbbuf, true);
            if !path.exists() {
                // main file appears at first write (C pin): an empty WAL-mode db
                let mut empty = dbfile::write_db_bytes(&Default::default());
                dbfile::set_journal_versions(&mut empty, true);
                let _ = std::fs::write(&path, empty);
            }
            let _ = dbfile::write_wal(&walp, &dbbuf);
            if !shmp.exists() { let _ = std::fs::write(&shmp, []); }
            bump_file_version(db, &path);
        }
        if let Some(mode) = pending {
            // PASSIVE/FULL/RESTART backfill committed frames into the main db;
            // TRUNCATE additionally resets the -wal to zero bytes (C pins)
            let img = build_image(db);
            let mut dbbuf = dbfile::write_db_bytes(&img);
            dbfile::set_journal_versions(&mut dbbuf, true);
            let _ = std::fs::write(&path, dbbuf);
            if mode == "TRUNCATE" { let _ = std::fs::write(&walp, []); }
        }
    } else if walp.exists() {
        // journal_mode switched wal -> delete: backfill and drop the sidecars
        let img = build_image(db);
        let mut dbbuf = dbfile::write_db_bytes(&img);
        dbfile::set_journal_versions(&mut dbbuf, false);
        let _ = std::fs::write(&path, dbbuf);
        let _ = std::fs::remove_file(&walp);
        let _ = std::fs::remove_file(&shmp);
        bump_file_version(db, &path);
    } else if (marker != before || committed) && !in_txn_now {
        // run-36: delete-mode commits persist immediately (C durability point),
        // making committed state visible to sibling connections on the same file
        let img = build_image(db);
        let mut dbbuf = dbfile::write_db_bytes(&img);
        dbfile::set_journal_versions(&mut dbbuf, false);
        let _ = std::fs::write(&path, dbbuf);
        bump_file_version(db, &path);
    }
}

/// Load a parsed image into a connection's store (shared by open_file and deserialize).
pub fn load_image(db: usize, img: DbImage) {
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
            let mut tab = Table { cols, uniq_sets: parse_uniq_sets(&ti.sql), checks: parse_table_checks(&ti.sql), create_sql: ti.sql.clone(), ..Default::default() };
            let mut maxr = 0i64;
            for (rid, vals) in ti.rows { if rid > maxr { maxr = rid; } tab.rows.push((rid, vals)); }
            tab.next_rowid = maxr;
            st.catalog.push(("table".into(), ti.name.clone()));
            st.tables.push((ti.name, tab));
        }
        for ix in img.indexes {
            st.catalog.push(("index".into(), ix.name.clone()));
            st.index_owner.insert(ix.name.clone(), ix.tbl.clone());
            if let Some(isql) = ix.sql {
                if let Some(Stmt::CreateIndex { name, table, exprs, unique, where_c, sql }) = parse_stmt(&isql) {
                    if unique && exprs.len() == 1 && where_c.is_none() {
                        if let Some(t) = st.tables.iter_mut().find(|(n, _)| *n == table) {
                            if let Some(c) = t.1.cols.iter_mut().find(|c| c.name == exprs[0]) { c.unique = true; }
                        }
                    }
                    st.indexes.push(IndexDef { name, table, exprs, unique, where_c, sql });
                }
            }
        }
        for tg in img.triggers {
            if let Some(Stmt::CreateTrigger { name, def, .. }) = parse_stmt(&tg.sql) {
                st.catalog.push(("trigger".into(), name.clone()));
                st.triggers.push((name, def));
            }
        }
    });
}

pub fn drop_store(db: usize) {
    STORES.with(|m| { m.borrow_mut().remove(&db); });
}

/// Persist a file-backed connection to a real (C-readable) SQLite database file.
/// v12: UNIQUE is KEPT in the persisted sql and backed by real on-disk index
/// b-trees (column autoindexes, multi-column UNIQUE sets, explicit indexes), so
/// C enforces uniqueness against Rust-written files after reopen.
/// Build the on-disk image of a connection (shared by save_file and serialize).
// ---------------- run-35: incremental blob I/O (sqlite3_blob_*) ----------------

/// resolve + validate a blob-handle target in the pinned C order; returns the
/// column index. rowid aliases the INTEGER PRIMARY KEY when declared.
pub fn blob_target(db: usize, zdb: &str, table: &str, col: &str, rowid: i64, write: bool)
    -> Result<usize, String> {
    with_store(db, |st| {
        if st.views.contains_key(table) {
            return Err(format!("cannot open view: {table}"));
        }
        let t = match st.tables.iter().find(|(n, _)| *n == table) {
            Some((_, t)) => t,
            None => return Err(format!("no such table: {zdb}.{table}")),
        };
        let ci = match t.cols.iter().position(|c| c.name == col) {
            Some(ci) => ci,
            None => return Err(format!("no such column: \"{col}\"")),
        };
        if write {
            // indexed columns are refused for writing (pinned): inline UNIQUE, PK,
            // UNIQUE table constraints or any explicit index touching the column
            let indexed = t.cols[ci].unique
                || t.uniq_sets.iter().any(|s| s.iter().any(|c| c == col))
                || st.indexes.iter().any(|d| d.table == table
                    && d.exprs.iter().any(|e| e.trim().eq_ignore_ascii_case(col)));
            if indexed {
                return Err("cannot open indexed column for writing".into());
            }
        }
        let row = blob_row_of(t, rowid).ok_or(format!("no such rowid: {rowid}"))?;
        match t.rows[row].1.get(ci) {
            Some(Val::Blob(_)) | Some(Val::Text(_)) => Ok(ci),
            Some(Val::Null) | None => Err("cannot open value of type null".into()),
            Some(Val::Int(_)) => Err("cannot open value of type integer".into()),
            Some(Val::Real(_)) => Err("cannot open value of type real".into()),
        }
    })
}

fn blob_row_of(t: &Table, rowid: i64) -> Option<usize> {
    match dbfile::ipk_index(&t.create_sql) {
        Some(ipk) => t.rows.iter().position(|(_, r)| r.get(ipk) == Some(&Val::Int(rowid))),
        None => t.rows.iter().position(|(rid, _)| *rid == rowid),
    }
}

/// current byte length of the handle's cell (None when the row vanished)
pub fn blob_len(db: usize, table: &str, ci: usize, rowid: i64) -> Option<usize> {
    with_store(db, |st| {
        let t = st.tables.iter().find(|(n, _)| *n == table).map(|(_, t)| t)?;
        let row = blob_row_of(t, rowid)?;
        match t.rows[row].1.get(ci) {
            Some(Val::Blob(b)) => Some(b.len()),
            Some(Val::Text(s)) => Some(s.len()),
            _ => None,
        }
    })
}

/// read n bytes at offset (bounds already validated by the caller)
pub fn blob_read_bytes(db: usize, table: &str, ci: usize, rowid: i64, off: usize, n: usize)
    -> Option<Vec<u8>> {
    with_store(db, |st| {
        let t = st.tables.iter().find(|(nm, _)| *nm == table).map(|(_, t)| t)?;
        let row = blob_row_of(t, rowid)?;
        let bytes: &[u8] = match t.rows[row].1.get(ci) {
            Some(Val::Blob(b)) => b,
            Some(Val::Text(s)) => s.as_bytes(),
            _ => return None,
        };
        bytes.get(off..off + n).map(|s| s.to_vec())
    })
}

/// overwrite n bytes at offset in a Blob cell — the length NEVER changes and
/// change counters are NOT bumped (a live handle must not expire itself)
pub fn blob_write_bytes(db: usize, table: &str, ci: usize, rowid: i64, off: usize, data: &[u8])
    -> Result<(), String> {
    with_store(db, |st| {
        let t = st.tables.iter_mut().find(|(nm, _)| *nm == table).map(|(_, t)| t)
            .ok_or("no such table")?;
        let row = blob_row_of(t, rowid).ok_or("no such rowid")?;
        match t.rows[row].1.get_mut(ci) {
            Some(Val::Blob(b)) if off + data.len() <= b.len() => {
                b[off..off + data.len()].copy_from_slice(data);
                Ok(())
            }
            _ => Err("SQL logic error".into()),
        }
    })
}

/// refresh the freelist-model page counters after a mutation (run-34)
fn refresh_pages(st: &mut Store) {
    let pages = (dbfile::write_db_bytes(&image_of(st)).len() / 4096).max(1) as i64;
    st.conn.page_cur = pages;
    if pages > st.conn.page_hwm { st.conn.page_hwm = pages; }
}

pub fn build_image(db: usize) -> DbImage {
    with_store(db, |st| image_of(st))
}
/// image builder usable from INSIDE a with_store borrow (run-34: VACUUM / page counts)
fn image_of(st: &Store) -> DbImage {
    {
        let st = &*st;
        let tables: Vec<TableImage> = st.tables.iter().map(|(n, t)| {
            let sql = if t.create_sql.is_empty() {
                format!("CREATE TABLE {}({})", n, t.cols.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(","))
            } else { t.create_sql.clone() };
            TableImage { name: n.clone(), sql, rows: t.rows.clone() }
        }).collect();
        let triggers: Vec<TriggerImage> = st.triggers.iter()
            .map(|(n, d)| TriggerImage { name: n.clone(), tbl: d.table.clone(), sql: d.raw.clone() })
            .collect();
        let mut indexes: Vec<dbfile::IndexImage> = Vec::new();
        for (n, t) in &st.tables {
            let ipk = dbfile::ipk_index(&t.create_sql);
            let rowid_of = |rid: i64, vals: &Vec<Val>| -> i64 {
                match ipk { Some(i) => match vals.get(i) { Some(Val::Int(v)) => *v, _ => rid }, None => rid }
            };
            let mut auto_n = 1;
            let declared = {
                let (o, c) = (t.create_sql.find('('), t.create_sql.rfind(')'));
                match (o, c) { (Some(o), Some(c)) if c > o => parse_coldefs(&t.create_sql[o+1..c]).unwrap_or_default(), _ => Vec::new() }
            };
            for (ci, dc) in declared.iter().enumerate() {
                if !dc.unique { continue; }
                if ipk == Some(ci) { continue; }
                let entries: Vec<(Vec<Val>, i64)> = t.rows.iter()
                    .map(|(rid, vals)| (vec![vals.get(ci).cloned().unwrap_or(Val::Null)], rowid_of(*rid, vals))).collect();
                indexes.push(dbfile::IndexImage { name: format!("sqlite_autoindex_{}_{}", n, auto_n),
                    tbl: n.clone(), sql: None, entries });
                auto_n += 1;
            }
            for set in parse_uniq_sets(&t.create_sql) {
                let cis: Vec<usize> = set.iter().filter_map(|c| t.cols.iter().position(|cc| cc.name == *c)).collect();
                if cis.len() != set.len() { continue; }
                let entries: Vec<(Vec<Val>, i64)> = t.rows.iter().map(|(rid, vals)| {
                    (cis.iter().map(|&ci| vals.get(ci).cloned().unwrap_or(Val::Null)).collect(), rowid_of(*rid, vals))
                }).collect();
                indexes.push(dbfile::IndexImage { name: format!("sqlite_autoindex_{}_{}", n, auto_n),
                    tbl: n.clone(), sql: None, entries });
                auto_n += 1;
            }
        }
        for idef in &st.indexes {
            if let Some((_, t)) = st.tables.iter().find(|(n, _)| *n == idef.table) {
                let ipk = dbfile::ipk_index(&t.create_sql);
                let mut entries: Vec<(Vec<Val>, i64)> = Vec::new();
                for (rid, vals) in &t.rows {
                    if let Ok(Some(key)) = index_key_for(&t.cols, idef, vals) {
                        let rowid = match ipk { Some(i) => match vals.get(i) { Some(Val::Int(v)) => *v, _ => *rid }, None => *rid };
                        entries.push((key, rowid));
                    }
                }
                indexes.push(dbfile::IndexImage { name: idef.name.clone(), tbl: idef.table.clone(),
                    sql: Some(idef.sql.clone()), entries });
            }
        }
        DbImage { tables, triggers, indexes }
    }
}

/// current schema-change counter (statement auto-reprepare)
pub fn schema_version(db: usize) -> i64 {
    with_store(db, |st| st.conn.schema_version)
}
/// does this connection hold any tables (serialize builds a real image then)?
pub fn has_tables(db: usize) -> bool {
    with_store(db, |st| !st.tables.is_empty())
}

pub fn save_file(db: usize) {
    with_store(db, |st| { txn_rollback(st); }); // C: closing with an open txn rolls back
    let path = PATHS.with(|m| m.borrow().get(&db).cloned());
    if let Some(pb) = path {
        let journal = with_store(db, |st| st.conn.journal.clone());
        if journal == "wal" {
            // C clean close: checkpoint into the main db, keep WAL mode persisted in the
            // header (versions=2), delete -wal/-shm
            let img = build_image(db);
            let mut buf = dbfile::write_db_bytes(&img);
            dbfile::set_journal_versions(&mut buf, true);
            let _ = std::fs::write(&pb, buf);
            let _ = std::fs::remove_file(wal_sidecar(&pb, "-wal"));
            let _ = std::fs::remove_file(wal_sidecar(&pb, "-shm"));
        } else {
            let img = build_image(db);
            let _ = dbfile::write_db(&pb, &img);
        }
    }
    PATHS.with(|m| { m.borrow_mut().remove(&db); });
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
    if (t.starts_with("X'") || t.starts_with("x'")) && t.ends_with('\'') {
        let hx = &t[2..t.len()-1];
        if hx.len() % 2 == 0 && hx.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Some(Val::Blob((0..hx.len()).step_by(2)
                .map(|i| u8::from_str_radix(&hx[i..i+2], 16).unwrap()).collect()));
        }
        return None;
    }
    if let Ok(i) = t.parse::<i64>() {
        return Some(Val::Int(i));
    }
    if (t.contains('.') || t.contains('e') || t.contains('E')) && !t.starts_with('\'') {
        if let Ok(f) = t.parse::<f64>() {
            return Some(Val::Real(f));
        }
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
enum Policy { Abort, Ignore, Replace, DoNothing, DoUpdate, TxnRollback }

enum Stmt {
    PragmaFkOn,
    Create { name: String, cols: Vec<Col>, sql: String },
    CreateIndex { name: String, table: String, exprs: Vec<String>, unique: bool, where_c: Option<String>, sql: String },
    DropIndex { name: String },
    Begin { immediate: bool },
    Commit,
    Rollback,
    Savepoint { name: String },
    Release { name: String },
    RollbackTo { name: String },
    CreateTrigger { name: String, def: Trigger, sql: String },
    CreateView { name: String, body: String, sql: String },
    DropView { name: String },
    DropTrigger { name: String },
    RenameColumn { table: String, from: String, to: String },
    DropColumn { table: String, col: String },
    Drop { name: String },
    RenameTable { from: String, to: String },
    AddColumn { table: String, col: String, default: Option<Val> },
    Vacuum { into: Option<String> },
    Analyze { target: Option<String> },
    // Begin.immediate: BEGIN IMMEDIATE/EXCLUSIVE takes the file write lock now (run-36)
    // whx: raw WHERE expression fallback (evaluated per row via eval_standalone, run-34)
    Insert { name: String, collist: Option<Vec<String>>, rows: Vec<Vec<Val>>, policy: Policy,
             target: Option<(Vec<String>, Option<String>)>, // ON CONFLICT (<expr-list>) [WHERE <pred>]
             upd_sets: Vec<(String, String)>, /* DO UPDATE SET col=<expr>, ... */
             upd_where: Option<String> /* DO UPDATE ... WHERE <expr> */ },
    Update { name: String, col: String, add: Option<i64>, set: Option<Val>, wh: Option<(String, i64)>, or_mode: u8 /* 0=abort 1=ignore 2=fail 3=rollback */ },
    Delete { name: String, wh: Option<(String, i64)>, whx: Option<String> },
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
        let first = d.split_whitespace().next()?.to_ascii_uppercase();
        let first = first.split('(').next().unwrap_or(&first).to_string();
        if matches!(first.as_str(), "UNIQUE" | "PRIMARY" | "CHECK" | "FOREIGN" | "CONSTRAINT") {
            continue; // table-level constraint, handled by parse_uniq_sets
        }
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
        if let Some(cp) = up.find("COLLATE ") {
            if let Some(w) = d[cp + "COLLATE ".len()..].split_whitespace().next() {
                col.coll = Some(w.trim_matches('"').to_ascii_lowercase());
            }
        }
        if up.contains("DEFERRABLE INITIALLY DEFERRED") { col.ref_deferred = true; }
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
    if up == "BEGIN" || up.starts_with("BEGIN ") || up == "BEGIN TRANSACTION" {
        let rest = up.trim_start_matches("BEGIN").trim();
        if rest.is_empty() || matches!(rest, "TRANSACTION" | "DEFERRED" | "IMMEDIATE" | "EXCLUSIVE")
            || rest.starts_with("DEFERRED") || rest.starts_with("IMMEDIATE") || rest.starts_with("EXCLUSIVE") {
            return Some(Stmt::Begin { immediate: rest.starts_with("IMMEDIATE") || rest.starts_with("EXCLUSIVE") });
        }
        return None;
    }
    if up == "COMMIT" || up == "COMMIT TRANSACTION" || up == "END" || up == "END TRANSACTION" {
        return Some(Stmt::Commit);
    }
    if up == "ROLLBACK" || up == "ROLLBACK TRANSACTION" {
        return Some(Stmt::Rollback);
    }
    if let Some(r) = up.strip_prefix("ROLLBACK") {
        let r = r.trim().trim_start_matches("TRANSACTION").trim();
        if let Some(nm) = r.strip_prefix("TO ") {
            let nm = nm.trim().trim_start_matches("SAVEPOINT ").trim();
            let orig = &s[s.len() - nm.len()..];
            return Some(Stmt::RollbackTo { name: ident(orig)? });
        }
        return None;
    }
    if up.starts_with("SAVEPOINT ") {
        return Some(Stmt::Savepoint { name: ident(&s["SAVEPOINT ".len()..])? });
    }
    if up.starts_with("RELEASE ") {
        let rest = s["RELEASE ".len()..].trim();
        let rest = if rest.to_ascii_uppercase().starts_with("SAVEPOINT ") { &rest["SAVEPOINT ".len()..] } else { rest };
        return Some(Stmt::Release { name: ident(rest)? });
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
        // balanced close paren (expression indexes contain nested parens)
        let mut depth = 0; let mut close = None;
        for (i, ch) in tail.char_indices().skip(open) {
            match ch { '(' => depth += 1, ')' => { depth -= 1; if depth == 0 { close = Some(i); break; } }, _ => {} }
        }
        let close = close?;
        let inner = &tail[open + 1..close];
        let mut exprs = Vec::new();
        { let mut d = 0; let mut cur = String::new();
          for ch in inner.chars() {
              match ch { '(' => { d += 1; cur.push(ch); } ')' => { d -= 1; cur.push(ch); }
                        ',' if d == 0 => { exprs.push(cur.trim().to_string()); cur.clear(); } _ => cur.push(ch) }
          }
          if !cur.trim().is_empty() { exprs.push(cur.trim().to_string()); } }
        if exprs.is_empty() { return None; }
        // optional partial-index predicate
        let after = tail[close + 1..].trim();
        let where_c = if after.to_ascii_uppercase().starts_with("WHERE ") {
            Some(after["WHERE ".len()..].trim().to_string())
        } else { None };
        return Some(Stmt::CreateIndex { name, table, exprs, unique, where_c, sql: s.trim().to_string() });
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
                let verb = inner.split(',').next().unwrap_or("").trim().to_ascii_uppercase();
                if verb == "IGNORE" {
                    body.push(("#raise_ignore".to_string(), Vec::new()));
                    continue;
                }
                if verb == "ROLLBACK" {
                    let msg = inner.split_once(',').map(|(_, m)| m.trim().trim_matches('\'').to_string())
                        .unwrap_or_else(|| "RAISE".into());
                    body.push(("#raise_txnrb".to_string(), vec![format!("'{}'", msg.replace('\'', "''"))]));
                    continue;
                }
                let msg = inner.split_once(',').map(|(_, m)| m.trim().trim_matches('\'').to_string())
                    .unwrap_or_else(|| "RAISE".into());
                // ABORT / FAIL / ROLLBACK all fail the statement with rc 19 here (no txn stack)
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
    if up.starts_with("DROP INDEX ") {
        return Some(Stmt::DropIndex { name: ident(&s["DROP INDEX ".len()..])? });
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
        } else if up.starts_with("INSERT OR ROLLBACK INTO ") {
            (Policy::TxnRollback, &s["INSERT OR ROLLBACK INTO ".len()..])
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
        let mut upd_sets: Vec<(String, String)> = Vec::new();
        let mut upd_where = None;
        let vup = vals.to_ascii_uppercase();
        let mut target: Option<(Vec<String>, Option<String>)> = None;
        if let Some(oc) = vup.find(" ON CONFLICT") {
            let clause = &vals[oc..];
            let cup = clause.to_ascii_uppercase();
            // optional conflict target: ON CONFLICT (<expr-list>) [WHERE <pred>] DO ...
            {
                let after_kw = clause[" ON CONFLICT".len()..].trim_start();
                if after_kw.starts_with('(') {
                    let mut depth = 0; let mut end = None;
                    for (i, ch) in after_kw.char_indices() {
                        match ch { '(' => depth += 1, ')' => { depth -= 1; if depth == 0 { end = Some(i); break; } }, _ => {} }
                    }
                    let end = end?;
                    let inner = &after_kw[1..end];
                    let mut exprs = Vec::new();
                    { let mut d = 0; let mut cur = String::new();
                      for ch in inner.chars() {
                          match ch { '(' => { d += 1; cur.push(ch); } ')' => { d -= 1; cur.push(ch); }
                                    ',' if d == 0 => { exprs.push(cur.trim().to_string()); cur.clear(); } _ => cur.push(ch) }
                      }
                      if !cur.trim().is_empty() { exprs.push(cur.trim().to_string()); } }
                    let rest = after_kw[end + 1..].trim_start();
                    let rup = rest.to_ascii_uppercase();
                    let twhere = if rup.starts_with("WHERE ") {
                        let dp = rup.find(" DO ")?;
                        Some(rest["WHERE ".len()..dp].trim().to_string())
                    } else { None };
                    if exprs.is_empty() { return None; }
                    target = Some((exprs, twhere));
                }
            }
            if cup.contains("DO NOTHING") {
                policy = Policy::DoNothing;
            } else if let Some(du) = cup.find("DO UPDATE SET ") {
                policy = Policy::DoUpdate;
                let mut assign = clause[du + "DO UPDATE SET ".len()..].to_string();
                if let Some(wp) = assign.to_ascii_uppercase().find(" WHERE ") {
                    upd_where = Some(assign[wp + 7..].trim().to_string());
                    assign = assign[..wp].to_string();
                }
                let mut depth = 0; let mut cur = String::new(); let mut parts = Vec::new();
                for ch in assign.chars() {
                    match ch { '(' => { depth += 1; cur.push(ch); } ')' => { depth -= 1; cur.push(ch); }
                              ',' if depth == 0 => { parts.push(cur.clone()); cur.clear(); } _ => cur.push(ch) }
                }
                if !cur.trim().is_empty() { parts.push(cur); }
                for p in parts {
                    let eq = p.find('=')?;
                    let c = ident(&p[..eq])?;
                    upd_sets.push((c, p[eq + 1..].trim().to_string()));
                }
                if upd_sets.is_empty() { return None; }
            } else {
                return None;
            }
            vals = vals[..oc].trim();
        }
        let mut rows = Vec::new();
        for grp in vals.split("),") {
            let grp = grp.trim();
            let grp = grp.strip_prefix('(').unwrap_or(grp);
            let grp = grp.strip_suffix(')').unwrap_or(grp);
            let mut row = Vec::new();
            for tok in grp.split(',') {
                match parse_literal(tok) {
                    Some(v) => row.push(v),
                    // constant expression (zeroblob(1000), 1+2, ...) — computed for real
                    None => {
                        let env = std::collections::HashMap::new();
                        row.push(ev_to_val(eval::eval_standalone(tok.trim(), &env).ok()?));
                    }
                }
            }
            rows.push(row);
        }
        return Some(Stmt::Insert { name, collist, rows, policy, target, upd_sets, upd_where });
    }
    if up == "VACUUM" {
        return Some(Stmt::Vacuum { into: None });
    }
    if up == "ANALYZE" {
        return Some(Stmt::Analyze { target: None });
    }
    if up.starts_with("ANALYZE ") {
        let name = ident(s["ANALYZE ".len()..].trim())?;
        return Some(Stmt::Analyze { target: Some(name) });
    }
    if up.starts_with("VACUUM INTO ") {
        let arg = s["VACUUM INTO ".len()..].trim();
        let path = arg.strip_prefix('\'')?.strip_suffix('\'')?.to_string();
        return Some(Stmt::Vacuum { into: Some(path) });
    }
    if up.starts_with("UPDATE ") {
        let (or_mode, after): (u8, &str) = if up.starts_with("UPDATE OR IGNORE ") { (1, &s["UPDATE OR IGNORE ".len()..]) }
            else if up.starts_with("UPDATE OR FAIL ") { (2, &s["UPDATE OR FAIL ".len()..]) }
            else if up.starts_with("UPDATE OR ROLLBACK ") { (3, &s["UPDATE OR ROLLBACK ".len()..]) }
            else if up.starts_with("UPDATE OR ABORT ") { (0, &s["UPDATE OR ABORT ".len()..]) }
            else { (0, &s["UPDATE ".len()..]) };
        let _ = &after;
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
            // literal or constant expression (zeroblob(4), ... — run-35, mirrors INSERT)
            let v = match parse_literal(rhs) {
                Some(v) => v,
                None => {
                    let env = std::collections::HashMap::new();
                    ev_to_val(eval::eval_standalone(rhs, &env).ok()?)
                }
            };
            (None, Some(v))
        };
        let wh = match wh_txt { Some(w) => Some(parse_where_int(w)?), None => None };
        return Some(Stmt::Update { name, col, add, set, wh, or_mode });
    }
    if up.starts_with("DELETE FROM ") {
        let after = &s["DELETE FROM ".len()..];
        let (nm, wh_txt) = match after.to_ascii_uppercase().find(" WHERE ") {
            Some(w) => (&after[..w], Some(&after[w + 7..])),
            None => (after, None),
        };
        // typed col=int filter when possible; otherwise keep the raw expression and
        // evaluate it per row (run-34: DELETE ... WHERE n > 5 / v % 2 = 0 / IN (...))
        let (wh, whx) = match wh_txt {
            Some(w) => match parse_where_int(w) {
                Some(p) => (Some(p), None),
                None => (None, Some(w.trim().to_string())),
            },
            None => (None, None),
        };
        return Some(Stmt::Delete { name: ident(nm)?, wh, whx });
    }
    if up.starts_with("SELECT ") {
        let after = &s["SELECT ".len()..];
        let fpos = after.to_ascii_uppercase().find(" FROM ")?;
        let items: Vec<String> = after[..fpos].split(',').map(|i| i.trim().to_string()).collect();
        let mut rest = after[fpos + 6..].trim().to_string();
        let mut order_by = None;
        let mut order_raw = false;
        if let Some(o) = rest.to_ascii_uppercase().find(" ORDER BY ") {
            let raw = rest[o + 10..].trim().to_string();
            order_by = ident(&raw);
            if order_by.is_none() {
                // multi-key stays kitchen ONLY for sqlite_master (run-34); else evaluator
                order_by = Some(raw);
                order_raw = true;
            }
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
        if order_raw && target != "sqlite_master" { return None; } // COLLATE/NULLS/multi-key -> evaluator
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

/// table-constraint UNIQUE(a,b,...) column sets from a CREATE TABLE statement
fn parse_uniq_sets(sql: &str) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let (open, close) = match (sql.find('('), sql.rfind(')')) { (Some(o), Some(c)) if c > o => (o, c), _ => return out };
    let inner = &sql[open + 1..close];
    let mut depth = 0; let mut cur = String::new(); let mut defs = Vec::new();
    for ch in inner.chars() {
        match ch { '(' => { depth += 1; cur.push(ch); } ')' => { depth -= 1; cur.push(ch); }
                   ',' if depth == 0 => { defs.push(cur.clone()); cur.clear(); } _ => cur.push(ch) }
    }
    if !cur.trim().is_empty() { defs.push(cur); }
    for d in defs {
        let d = d.trim();
        let up = d.to_ascii_uppercase();
        if up.starts_with("UNIQUE") && d.contains('(') {
            let o = d.find('(').unwrap();
            let c = match d.rfind(')') { Some(c) => c, None => continue };
            let cols: Vec<String> = d[o+1..c].split(',').filter_map(|x| ident(x)).collect();
            if !cols.is_empty() { out.push(cols); }
        }
    }
    out
}

/// table-constraint CHECK(expr) expressions from a CREATE TABLE statement
fn parse_table_checks(sql: &str) -> Vec<String> {
    let mut out = Vec::new();
    let (open, close) = match (sql.find('('), sql.rfind(')')) { (Some(o), Some(c)) if c > o => (o, c), _ => return out };
    let inner = &sql[open + 1..close];
    let mut depth = 0; let mut cur = String::new(); let mut defs = Vec::new();
    for ch in inner.chars() {
        match ch { '(' => { depth += 1; cur.push(ch); } ')' => { depth -= 1; cur.push(ch); }
                   ',' if depth == 0 => { defs.push(cur.clone()); cur.clear(); } _ => cur.push(ch) }
    }
    if !cur.trim().is_empty() { defs.push(cur); }
    for d in defs {
        let d = d.trim();
        if d.to_ascii_uppercase().starts_with("CHECK") {
            if let (Some(o), Some(c)) = (d.find('('), d.rfind(')')) {
                if c > o { out.push(d[o + 1..c].trim().to_string()); }
            }
        }
    }
    out
}

/// evaluate all CHECKs (column + table level) against a full row image;
/// NULL results pass (SQL semantics); returns the failing message if any
fn check_row(name: &str, cols: &[Col], checks: &[String], row: &[Val]) -> Result<Option<String>, String> {
    let mut env: std::collections::HashMap<String, eval::V> = Default::default();
    for (cj, cc) in cols.iter().enumerate() {
        env.insert(cc.name.clone(), val_to_ev(row.get(cj).unwrap_or(&Val::Null)));
    }
    for (ci, col) in cols.iter().enumerate() {
        if col.not_null && matches!(row.get(ci), Some(Val::Null) | None) {
            return Ok(Some(format!("NOT NULL constraint failed: {}.{}", name, col.name)));
        }
        if let Some(chk) = &col.check {
            let r = eval::eval_standalone(chk, &env)?;
            if !matches!(r, eval::V::Null) && !ev_truthy(&r) {
                return Ok(Some(format!("CHECK constraint failed: {}", chk)));
            }
        }
    }
    for chk in checks {
        let r = eval::eval_standalone(chk, &env)?;
        if !matches!(r, eval::V::Null) && !ev_truthy(&r) {
            return Ok(Some(format!("CHECK constraint failed: {}", name)));
        }
    }
    Ok(None)
}

/// total-ordered index key (val_ord over each component)
#[derive(Clone, PartialEq)]
pub(crate) struct KeyVal(pub Vec<Val>);
impl Eq for KeyVal {} // Val holds f64; ordering is total via val_ord (NaN never stored by pins)
impl PartialOrd for KeyVal { fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(o)) } }
impl Ord for KeyVal {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        for i in 0..self.0.len().min(o.0.len()) {
            let c = dbfile::val_ord(&self.0[i], &o.0[i]);
            if c != std::cmp::Ordering::Equal { return c; }
        }
        self.0.len().cmp(&o.0.len())
    }
}

/// evaluate an index key tuple for a row; None when a partial predicate excludes it
fn index_key_for(cols: &[Col], idx: &IndexDef, row: &[Val]) -> Result<Option<Vec<Val>>, String> {
    let mut env: std::collections::HashMap<String, eval::V> = Default::default();
    for (cj, cc) in cols.iter().enumerate() {
        env.insert(cc.name.clone(), val_to_ev(row.get(cj).unwrap_or(&Val::Null)));
    }
    if let Some(w) = &idx.where_c {
        let r = eval::eval_standalone(w, &env)?;
        if matches!(r, eval::V::Null) || !ev_truthy(&r) { return Ok(None); }
    }
    let mut key = Vec::new();
    for e in &idx.exprs {
        // plain column name: direct fetch; anything else: real expression evaluation
        if let Some(ci) = cols.iter().position(|c| c.name == *e) {
            key.push(row.get(ci).cloned().unwrap_or(Val::Null));
        } else {
            key.push(ev_to_val(eval::eval_standalone(e, &env)?));
        }
    }
    Ok(Some(key))
}

/// conflict against explicit UNIQUE indexes (multi-column, partial, expression):
/// NULL key components keep rows distinct, exactly like SQL UNIQUE.
fn unique_index_conflict(t: &Table, idefs: &[IndexDef], vals: &[Val]) -> Result<Option<usize>, String> {
    for idx in idefs {
        if !idx.unique { continue; }
        let key = match index_key_for(&t.cols, idx, vals)? { Some(k) => k, None => continue };
        if key.iter().any(|v| matches!(v, Val::Null)) { continue; }
        for (pos, (_, r)) in t.rows.iter().enumerate() {
            if let Some(rk) = index_key_for(&t.cols, idx, r)? {
                if rk == key { return Ok(Some(pos)); }
            }
        }
    }
    Ok(None)
}

/// normalize an index/target expression for structural comparison
/// (lowercase, whitespace removed — the same spirit as sqlite3UpsertAnalyzeTarget's
/// sqlite3ExprCompare for the pinned scope)
fn norm_expr(e: &str) -> String {
    e.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_lowercase().trim_matches('"').to_string()
}

/// resolved ON CONFLICT target
enum UpsertTk {
    Cols(Vec<usize>),   // PK / UNIQUE column(s) or table-constraint UNIQUE set
    Idx(IndexDef),      // explicit UNIQUE index (column / multi-column / expression / partial)
}

/// C sqlite3UpsertAnalyzeTarget equivalent for the frozen scope: match the target
/// expression list (and optional WHERE) against PK/UNIQUE columns, UNIQUE table
/// constraints and UNIQUE indexes; mismatch -> the pinned C error
fn resolve_upsert_target(t: &Table, idefs: &[IndexDef], exprs: &[String], twhere: &Option<String>)
    -> Result<UpsertTk, String> {
    const NOMATCH: &str = "ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint";
    let want: Vec<String> = exprs.iter().map(|e| norm_expr(e)).collect();
    let wwant = twhere.as_ref().map(|w| norm_expr(w));
    // explicit UNIQUE indexes first (covers expression / multi-column / partial)
    for idx in idefs {
        if !idx.unique { continue; }
        let have: Vec<String> = idx.exprs.iter().map(|e| norm_expr(e)).collect();
        let whave = idx.where_c.as_ref().map(|w| norm_expr(w));
        if have == want && whave == wwant { return Ok(UpsertTk::Idx(idx.clone())); }
    }
    if wwant.is_some() { return Err(NOMATCH.into()); } // WHERE only matches a partial index
    // single PK / UNIQUE column
    if want.len() == 1 {
        if let Some(ci) = t.cols.iter().position(|c| c.name.to_ascii_lowercase() == want[0] && c.unique) {
            return Ok(UpsertTk::Cols(vec![ci]));
        }
    }
    // multi-column UNIQUE(a,b,...) table constraint
    for set in &t.uniq_sets {
        let have: Vec<String> = set.iter().map(|c| c.to_ascii_lowercase()).collect();
        if have == want {
            let cis: Vec<usize> = set.iter().filter_map(|c| t.cols.iter().position(|cc| cc.name == *c)).collect();
            if cis.len() == set.len() { return Ok(UpsertTk::Cols(cis)); }
        }
    }
    Err(NOMATCH.into())
}

/// conflict row for a resolved target only
fn targeted_conflict(t: &Table, tk: &UpsertTk, vals: &[Val]) -> Result<Option<usize>, String> {
    match tk {
        UpsertTk::Cols(cis) => {
            if cis.iter().any(|&ci| matches!(vals.get(ci), Some(Val::Null) | None)) { return Ok(None); }
            Ok(t.rows.iter().position(|(_, r)| cis.iter().all(|&ci| r.get(ci) == vals.get(ci))))
        }
        UpsertTk::Idx(idx) => unique_index_conflict(t, std::slice::from_ref(idx), vals),
    }
}

/// qualified UNIQUE-violation message for conflicts on constraints OTHER than the
/// upsert target (C aborts with rc 19 and names the constraint)
fn other_conflict_msg(name: &str, t: &Table, idefs: &[IndexDef], tk: &UpsertTk, vals: &[Val])
    -> Result<Option<String>, String> {
    let skip_ci: Vec<usize> = match tk { UpsertTk::Cols(c) => c.clone(), _ => Vec::new() };
    let skip_idx: Option<&str> = match tk { UpsertTk::Idx(i) => Some(&i.name), _ => None };
    for (ci, col) in t.cols.iter().enumerate() {
        if !col.unique || skip_ci == vec![ci] { continue; }
        if let Some(v) = vals.get(ci) {
            if *v != Val::Null && t.rows.iter().any(|(_, r)| r.get(ci) == Some(v)) {
                return Ok(Some(format!("UNIQUE constraint failed: {name}.{}", col.name)));
            }
        }
    }
    for set in &t.uniq_sets {
        let cis: Vec<usize> = set.iter().filter_map(|c| t.cols.iter().position(|cc| cc.name == *c)).collect();
        if cis.len() != set.len() || (matches!(tk, UpsertTk::Cols(c) if *c == cis)) { continue; }
        if cis.iter().any(|&ci| matches!(vals.get(ci), Some(Val::Null) | None)) { continue; }
        if t.rows.iter().any(|(_, r)| cis.iter().all(|&ci| r.get(ci) == vals.get(ci))) {
            let qual: Vec<String> = set.iter().map(|c| format!("{name}.{c}")).collect();
            return Ok(Some(format!("UNIQUE constraint failed: {}", qual.join(", "))));
        }
    }
    for idx in idefs {
        if !idx.unique || Some(idx.name.as_str()) == skip_idx { continue; }
        if unique_index_conflict(t, std::slice::from_ref(idx), vals)?.is_some() {
            return Ok(Some(format!("UNIQUE constraint failed: index '{}'", idx.name)));
        }
    }
    Ok(None)
}

/// any child row whose non-NULL reference lacks a parent (deferred-FK COMMIT check)
fn fk_violation_exists(st: &Store) -> bool {
    for (_tn, t) in &st.tables {
        for (ci, col) in t.cols.iter().enumerate() {
            if let Some((p, pc, _, _)) = &col.references {
                let parent = match st.tables.iter().find(|(n, _)| n == p) { Some(x) => &x.1, None => return true };
                let pci = match parent.cols.iter().position(|c| c.name == *pc) { Some(x) => x, None => return true };
                for (_rid, r) in &t.rows {
                    match r.get(ci) {
                        Some(Val::Null) | None => {}
                        Some(v) => {
                            if !parent.rows.iter().any(|(_, pr)| pr.get(pci) == Some(v)) { return true; }
                        }
                    }
                }
            }
        }
    }
    false
}

// ---------------- run-37: ANALYZE -> sqlite_stat1 ----------------

/// PRIMARY KEY column names (for the WITHOUT ROWID pseudo-index in stat1)
fn pk_cols(create_sql: &str) -> Vec<String> {
    let (o, c) = match (create_sql.find('('), create_sql.rfind(')')) {
        (Some(o), Some(c)) if c > o => (o, c),
        _ => return Vec::new(),
    };
    let inner = &create_sql[o + 1..c];
    let mut out = Vec::new();
    let mut depth = 0; let mut cur = String::new(); let mut defs = Vec::new();
    for ch in inner.chars() {
        match ch { '(' => { depth += 1; cur.push(ch); } ')' => { depth -= 1; cur.push(ch); }
                  ',' if depth == 0 => { defs.push(cur.clone()); cur.clear(); } _ => cur.push(ch) }
    }
    if !cur.trim().is_empty() { defs.push(cur); }
    for d in &defs {
        let up = d.to_ascii_uppercase();
        if up.trim_start().starts_with("PRIMARY KEY") {
            if let (Some(o2), Some(c2)) = (d.find('('), d.rfind(')')) {
                for c in d[o2 + 1..c2].split(',') {
                    if let Some(n) = ident(c.trim()) { out.push(n); }
                }
            }
        } else if up.contains("PRIMARY KEY") {
            if let Some(n) = d.split_whitespace().next().and_then(ident) { out.push(n); }
        }
    }
    out
}

/// C's stat1 selectivity: ceil(nRow/nDistinct), with the near-1.0 quirk pinned by
/// engine-analyze-001-C014 (iVal 2 collapses to 1 when nRow*10 <= nDistinct*11)
fn stat1_ival(n_row: u64, n_distinct: u64) -> u64 {
    let iv = (n_row + n_distinct - 1) / n_distinct;
    if iv == 2 && n_row * 10 <= n_distinct * 11 { 1 } else { iv }
}

/// stat text for one index: "{nRow} {v1} {v2} ..." over real evaluated key tuples
fn stat1_text(cols: &[Col], idx: &IndexDef, rows: &[(i64, Vec<Val>)]) -> Result<String, String> {
    let mut keys: Vec<Vec<Val>> = Vec::new();
    for (_rid, r) in rows {
        if let Some(k) = index_key_for(cols, idx, r)? { keys.push(k); }
    }
    let n = keys.len() as u64;
    let mut out = n.to_string();
    let ncols = idx.exprs.len();
    for k in 1..=ncols {
        let mut set: std::collections::HashSet<String> = Default::default();
        for key in &keys {
            let pfx: Vec<String> = key[..k].iter().map(|v| format!("{:?}", v.render())).collect();
            set.insert(pfx.join("\u{1}"));
        }
        let d = set.len().max(1) as u64;
        out.push(' ');
        out.push_str(&stat1_ival(n, d).to_string());
    }
    Ok(out)
}

/// ensure the sqlite_stat1 catalog table exists (ANALYZE always creates it)
fn ensure_stat1(st: &mut Store) {
    if st.tables.iter().any(|(n, _)| n == "sqlite_stat1") { return; }
    let sql = "CREATE TABLE sqlite_stat1(tbl,idx,stat)";
    let cols = parse_coldefs("tbl,idx,stat").unwrap_or_default();
    st.tables.push(("sqlite_stat1".into(), Table { cols, create_sql: sql.into(), ..Default::default() }));
    st.catalog.push(("table".into(), "sqlite_stat1".into()));
}

/// delete stat1 rows matching (tbl [, idx]) — re-ANALYZE / DROP maintenance
fn stat1_delete(st: &mut Store, tbl: Option<&str>, idx: Option<&str>) {
    if let Some((_, s)) = st.tables.iter_mut().find(|(n, _)| n == "sqlite_stat1") {
        s.rows.retain(|(_, r)| {
            let rt = match r.first() { Some(Val::Text(t)) => t.clone(), _ => String::new() };
            let ri = match r.get(1) { Some(Val::Text(t)) => Some(t.clone()), _ => None };
            let tbl_match = tbl.map_or(true, |t| rt == t);
            let idx_match = idx.map_or(true, |i| ri.as_deref() == Some(i));
            !(tbl_match && idx_match)
        });
    }
}

fn stat1_insert(st: &mut Store, tbl: &str, idx: Option<&str>, stat: &str) {
    if let Some((_, s)) = st.tables.iter_mut().find(|(n, _)| n == "sqlite_stat1") {
        s.next_rowid += 1;
        let rid = s.next_rowid;
        s.rows.push((rid, vec![
            Val::Text(tbl.into()),
            match idx { Some(i) => Val::Text(i.into()), None => Val::Null },
            Val::Text(stat.into()),
        ]));
    }
}

fn conflict_row(t: &Table, vals: &[Val]) -> Option<usize> {
    for (ci, col) in t.cols.iter().enumerate() {
        if !col.unique { continue; }
        if let Some(v) = vals.get(ci) {
            if *v == Val::Null { continue; } // SQL UNIQUE: NULLs are all distinct
            if let Some(pos) = t.rows.iter().position(|(_, r)| r.get(ci) == Some(v)) {
                return Some(pos);
            }
        }
    }
    // multi-column UNIQUE(a,b,...) table constraints
    for set in &t.uniq_sets {
        let cis: Vec<usize> = set.iter().filter_map(|c| t.cols.iter().position(|cc| cc.name == *c)).collect();
        if cis.len() != set.len() { continue; }
        if cis.iter().any(|&ci| matches!(vals.get(ci), Some(Val::Null) | None)) { continue; }
        if let Some(pos) = t.rows.iter().position(|(_, r)| cis.iter().all(|&ci| r.get(ci) == vals.get(ci))) {
            return Some(pos);
        }
    }
    None
}

/// rows of a simple single-table projection view
fn view_rows(st: &Store, view: &str) -> Result<(Vec<Vec<Val>>, Vec<String>), String> {
    let body = st.views.get(view).ok_or("no such view")?.clone();
    let vcols = view_colnames(&body);
    let up = body.to_ascii_uppercase();
    let fpos = up.find(" FROM ").ok_or("unsupported view shape")?;
    let base = body[fpos + 6..].trim().split_whitespace().next().unwrap_or("").to_string();
    let t = st.tables.iter().find(|(n, _)| *n == base).ok_or("no such table")?;
    let cis: Vec<usize> = vcols.iter()
        .map(|c| t.1.cols.iter().position(|cc| cc.name == *c).ok_or(format!("no such column: {c}")))
        .collect::<Result<_, _>>()?;
    let rows = t.1.rows.iter().map(|(_, r)| cis.iter().map(|&ci| r[ci].clone()).collect()).collect();
    Ok((rows, vcols))
}

/// execute INSTEAD OF trigger bodies against an old./new. env
fn run_trigger_bodies(st: &mut Store, trigs: &[Trigger], env: &std::collections::HashMap<String, eval::V>) -> Result<(), String> {
    for tg in trigs {
        if let Some(w) = &tg.when {
            if !ev_truthy(&eval::eval_standalone(w, env)?) { continue; }
        }
        for (target, exprs) in &tg.body {
            if target == "#raise_ignore" { return Ok(()); }
            if target == "#raise_txnrb" {
                let msg = eval::eval_standalone(&exprs[0], env)?;
                return Err(format!("__TXNROLLBACK__{}", msg.render().unwrap_or_default()));
            }
            if target == "#raise" {
                let msg = eval::eval_standalone(&exprs[0], env)?;
                return Err(format!("__RAISE__{}", msg.render().unwrap_or_default()));
            }
            let mut row = Vec::new();
            for e in exprs { row.push(ev_to_val(eval::eval_standalone(e, env)?)); }
            let tt = st.tables.iter_mut().find(|(n, _)| n == target).ok_or("no such table")?;
            tt.1.next_rowid += 1;
            let rid = tt.1.next_rowid;
            tt.1.rows.push((rid, row));
            st.total_changes += 1;
        }
    }
    Ok(())
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

thread_local! {
    static PROBE_CELL: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}
/// last-statement index-probe count (anti-cheat proof lookups use the index b-tree)
pub fn index_probe_count() -> u64 { PROBE_CELL.with(|c| c.get()) }

/// declared types for a SELECT's output columns (column_decltype/_16). A bare column
/// of a single FROM table yields its CREATE-TABLE declared type; expressions -> None.
pub fn stmt_decltypes(db: usize, sql: &str) -> Vec<Option<String>> {
    let s = sql.trim().trim_end_matches(';').trim();
    let up = s.to_ascii_uppercase();
    if !up.starts_with("SELECT") { return Vec::new(); }
    let rest = &s[6..];
    let fpos = match up[6..].find(" FROM ") { Some(p) => p, None => rest.len() };
    let items_str = &rest[..fpos];
    let table = if fpos < rest.len() {
        rest[fpos + 6..].trim().split(|c: char| c.is_whitespace() || c == ',' || c == ';').next().unwrap_or("").to_string()
    } else { String::new() };
    with_store(db, |st| {
        let cols_decl: Vec<(String, String)> = st.tables.iter().find(|(n, _)| *n == table)
            .map(|(_, t)| {
                let (o, c) = (t.create_sql.find('('), t.create_sql.rfind(')'));
                match (o, c) { (Some(o), Some(c)) if c > o =>
                    parse_coldefs_decl(&t.create_sql[o + 1..c]), _ => Vec::new() }
            }).unwrap_or_default();
        let mut items: Vec<String> = Vec::new();
        { let mut depth = 0; let mut cur = String::new(); let mut inq = false;
          for ch in items_str.chars() {
              match ch { '\'' => { inq = !inq; cur.push(ch); } '(' if !inq => { depth += 1; cur.push(ch); }
                        ')' if !inq => { depth -= 1; cur.push(ch); }
                        ',' if depth == 0 && !inq => { items.push(cur.trim().to_string()); cur.clear(); } _ => cur.push(ch) }
          }
          if !cur.trim().is_empty() { items.push(cur.trim().to_string()); } }
        items.iter().map(|item| {
            let it = item.trim();
            let base = it.rsplit('.').next().unwrap_or(it).trim();
            cols_decl.iter().find(|(n, _)| n == base).map(|(_, d)| d.clone())
        }).collect()
    })
}
/// (col name, declared type) pairs from a CREATE TABLE column list
fn parse_coldefs_decl(inner: &str) -> Vec<(String, String)> {
    let mut out = Vec::new(); let mut depth = 0; let mut cur = String::new(); let mut defs = Vec::new();
    for ch in inner.chars() {
        match ch { '(' => { depth += 1; cur.push(ch); } ')' => { depth -= 1; cur.push(ch); }
                   ',' if depth == 0 => { defs.push(cur.clone()); cur.clear(); } _ => cur.push(ch) }
    }
    if !cur.trim().is_empty() { defs.push(cur); }
    for d in defs {
        let d = d.trim();
        let up = d.to_ascii_uppercase();
        let first = d.split_whitespace().next().unwrap_or("");
        if matches!(up.split(['(', ' ']).next().unwrap_or(""), "UNIQUE"|"PRIMARY"|"CHECK"|"FOREIGN"|"CONSTRAINT") { continue; }
        if let Some(name) = ident(first) {
            // declared type = tokens after the name up to a constraint keyword
            let after = d[first.len()..].trim();
            let mut ty = String::new();
            for w in after.split_whitespace() {
                let wu = w.to_ascii_uppercase();
                if matches!(wu.as_str(), "PRIMARY"|"NOT"|"UNIQUE"|"CHECK"|"DEFAULT"|"REFERENCES"|"COLLATE"|"GENERATED"|"AS") { break; }
                if !ty.is_empty() { ty.push(' '); }
                ty.push_str(w);
            }
            if !ty.is_empty() { out.push((name, ty)); }
        }
    }
    out
}

/// name of an explicit index whose first column is `col` on `table` (honest EQP)
pub fn index_for(db: usize, table: &str, col: &str) -> Option<String> {
    with_store(db, |st| st.indexes.iter()
        .find(|d| d.table == table && d.where_c.is_none() && d.exprs.first().map(|e| e == col).unwrap_or(false))
        .map(|d| d.name.clone()))
}

/// single-column index maps for the eval probe path: table -> [(col, key_str -> row positions)].
/// key positions index into the snapshot's row vector (built alongside `snap`).
/// declared column collations (lower colname -> lower collation name) for eval's
/// COLLATE resolution; pinned scope assumes unambiguous column names across tables
fn build_coll_snapshot(st: &Store) -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    for (_n, t) in &st.tables {
        for c in &t.cols {
            if let Some(cl) = &c.coll { m.insert(c.name.to_ascii_lowercase(), cl.clone()); }
        }
    }
    m
}
fn build_index_snapshot(st: &Store)
    -> std::collections::HashMap<String, Vec<(String, std::collections::BTreeMap<String, Vec<usize>>)>> {
    let mut out = std::collections::HashMap::new();
    for (tname, t) in &st.tables {
        let mut cols_with_index: Vec<String> = Vec::new();
        // explicit single-column, non-partial indexes on plain columns
        for d in &st.indexes {
            if d.table == *tname && d.where_c.is_none() && d.exprs.len() == 1
                && t.cols.iter().any(|c| c.name == d.exprs[0]) {
                cols_with_index.push(d.exprs[0].clone());
            }
        }
        // column UNIQUE autoindexes probe too
        for c in &t.cols { if c.unique { cols_with_index.push(c.name.clone()); } }
        cols_with_index.sort(); cols_with_index.dedup();
        let mut maps = Vec::new();
        for col in cols_with_index {
            let ci = match t.cols.iter().position(|c| c.name == col) { Some(i) => i, None => continue };
            let mut m: std::collections::BTreeMap<String, Vec<usize>> = std::collections::BTreeMap::new();
            for (pos, (_, r)) in t.rows.iter().enumerate() {
                if let Some(v) = r.get(ci) {
                    if matches!(v, Val::Null) { continue; }
                    m.entry(v.render().unwrap_or_default()).or_default().push(pos);
                }
            }
            maps.push((col, m));
        }
        if !maps.is_empty() { out.insert(tname.clone(), maps); }
    }
    out
}

fn take_snap(st: &Store) -> Snap {
    Snap {
        tables: st.tables.clone(),
        catalog: st.catalog.clone(),
        views: st.views.clone(),
        triggers: st.triggers.clone(),
        indexes: st.indexes.clone(),
        index_owner: st.index_owner.clone(),
        fk_on: st.fk_on,
    }
}
fn restore_snap(st: &mut Store, s: Snap) {
    st.tables = s.tables;
    st.catalog = s.catalog;
    st.views = s.views;
    st.triggers = s.triggers;
    st.indexes = s.indexes;
    st.index_owner = s.index_owner;
    st.fk_on = s.fk_on;
}
/// roll the whole transaction back (OR ROLLBACK / explicit ROLLBACK / close)
fn txn_rollback(st: &mut Store) -> bool {
    match st.txn.take() {
        Some(tx) => { restore_snap(st, tx.snap); true }
        None => false,
    }
}
/// is this connection inside an explicit/savepoint transaction? (sqlite3_get_autocommit)
pub fn in_txn(db: usize) -> bool {
    with_store(db, |st| st.txn.is_some())
}

fn ev_truthy(v: &eval::V) -> bool {
    match v { eval::V::Null => false, eval::V::Int(i) => *i != 0, eval::V::Real(r) => *r != 0.0,
              eval::V::Text(t) => t.parse::<f64>().map(|f| f != 0.0).unwrap_or(false), eval::V::Blob(_) => true }
}
fn ev_to_val(v: eval::V) -> Val {
    match v { eval::V::Null => Val::Null, eval::V::Int(i) => Val::Int(i), eval::V::Real(r) => Val::Real(r),
              eval::V::Text(t) => Val::Text(t), eval::V::Blob(b) => Val::Blob(b) }
}
/// Fire matching triggers for one row event. WHEN + body value expressions are
/// evaluated for real (eval::eval_standalone) against old.*/new.* bindings.
fn fire_triggers(st: &mut Store, table: &str, timing: u8, event: u8,
                 old: Option<&Vec<Val>>, new: Option<&Vec<Val>>) -> Result<bool, String> {
    fire_triggers_d(st, table, timing, event, old, new, None, 0)
}
fn fire_triggers_d(st: &mut Store, table: &str, timing: u8, event: u8,
                 old: Option<&Vec<Val>>, new: Option<&Vec<Val>>,
                 upd_col: Option<&str>, depth: u32) -> Result<bool, String> {
    if depth > 16 { return Err("too many levels of trigger recursion".into()); }
    let trigs: Vec<Trigger> = st.triggers.iter()
        .filter(|(_, d)| d.table == table && d.timing == timing && d.event == event)
        .filter(|(_, d)| match (&d.of_col, upd_col) {
            (Some(oc), Some(uc)) => oc == uc,
            (Some(_), None) => event != 1, // UPDATE OF requires a matching updated column
            _ => true,
        })
        .map(|(_, d)| d.clone()).collect();
    if trigs.is_empty() { return Ok(true); }
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
            if target == "#raise_ignore" {
                return Ok(false); // RAISE(IGNORE): skip this row's operation silently
            }
            if target == "#raise_txnrb" {
                let msg = eval::eval_standalone(&exprs[0], &env)?;
                // RAISE(ROLLBACK): the whole transaction unwinds (v15 real txns)
                return Err(format!("__TXNROLLBACK__{}", match msg { eval::V::Text(m) => m, v => v.render().unwrap_or_default() }));
            }
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
    Ok(true)
}

/// Typed single-statement query for the statement API (pack v13): SELECT/PRAGMA run
/// through the SAME eval engine as sqlite3_exec, returning typed rows + column names.
/// Kitchen-owned targets (sqlite_master) fall back to the rendered kitchen path.
/// prepare-time syntax gate: does the engine recognize this statement shape at all?
pub fn stmt_prepare_check(_db: usize, sql: &str) -> bool {
    let s = sql.trim().trim_end_matches(';').trim();
    if parse_stmt(s).is_some() { return true; }
    let up = s.to_ascii_uppercase();
    ["PRAGMA", "ATTACH", "DETACH", "SELECT", "CREATE", "DROP", "ALTER", "INSERT", "UPDATE", "DELETE"]
        .iter().any(|kw| crate::eval::kw_bound(&up, kw))
}
/// prepare-time resolution: DML against a missing table (C reports at compile time)
pub fn stmt_missing_table(db: usize, sql: &str) -> Option<String> {
    let s = sql.trim().trim_end_matches(';').trim();
    match parse_stmt(s) {
        Some(Stmt::Insert { name, .. }) | Some(Stmt::Update { name, .. }) | Some(Stmt::Delete { name, .. }) => {
            with_store(db, |st| {
                if st.tables.iter().any(|(n, _)| *n == name) || st.views.contains_key(&name) { None }
                else { Some(name) }
            })
        }
        _ => None,
    }
}

pub fn stmt_query_typed(db: usize, sql: &str) -> Result<(Vec<String>, Vec<Vec<eval::V>>), String> {
    maybe_refresh_from_file(db); // run-36: pick up sibling connections' commits
    let s = sql.trim().trim_end_matches(';').trim();
    let up = s.to_ascii_uppercase();
    let master = up.contains("SQLITE_MASTER") || up.contains("SQLITE_SCHEMA");
    // run-34: simple rowid projections/sorts go to the kitchen (eval rows carry no rowids)
    let rowid_kitchen = up.contains("ROWID") && matches!(parse_stmt(s), Some(Stmt::Select { .. }));
    if up.starts_with("SELECT") && !master && !rowid_kitchen {
        return with_store(db, |st| {
            let snap: std::collections::HashMap<String, (Vec<String>, Vec<Vec<eval::V>>)> =
                st.tables.iter().map(|(n, t)| (n.clone(),
                    (t.cols.iter().map(|c| c.name.clone()).collect(),
                     t.rows.iter().map(|(_, r)| r.iter().map(val_to_ev).collect()).collect()))).collect();
            let fk: std::collections::HashMap<String, usize> = st.tables.iter()
                .map(|(n, t)| (n.clone(), t.cols.iter().filter(|c| c.references.is_some()).count())).collect();
            let idx: std::collections::HashMap<String, usize> = {
                let mut m = std::collections::HashMap::new();
                for tn in st.index_owner.values() { *m.entry(tn.clone()).or_insert(0) += 1; }
                for (n, _) in &st.tables { m.entry(n.clone()).or_insert(0); }
                m
            };
            let views = st.views.clone();
            let idxmaps = build_index_snapshot(st);
            let colls = build_coll_snapshot(st);
            PROBE_CELL.with(|c| c.set(0));
            let r = PROBE_CELL.with(|probes| {
                let mut ctx = eval::Ctx { db, conn: &mut st.conn, tables: &snap, fk_counts: &fk, index_counts: &idx, views: &views, indexes: &idxmaps, probes, col_colls: &colls };
                eval::stmt_select_typed(&mut ctx, s)
            });
            r
        });
    }
    // non-SELECT (DML/DDL/pragma/sqlite_master): run through the shared script engine
    match execute_script(db, s) {
        Outcome::Done { rows, rc: 0, .. } => {
            let n = rows.first().map(|r| r.len()).unwrap_or(0);
            let names = (0..n).map(|i| format!("column{i}")).collect();
            Ok((names, rows.into_iter().map(|r| r.into_iter().map(|c| match c {
                Some(s) => eval::V::Text(s), None => eval::V::Null }).collect()).collect()))
        }
        Outcome::Done { err, .. } => Err(err.unwrap_or_else(|| "SQL error".into())),
        Outcome::NotKitchen => Err("SQL error".into()),
    }
}

pub fn execute_script(db: usize, script: &str) -> Outcome {
    maybe_refresh_from_file(db); // run-36: pick up sibling connections' commits
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
                    let idxmaps = build_index_snapshot(st);
                    let views2 = st.views.clone();
                    let colls2 = build_coll_snapshot(st);
                    let res = PROBE_CELL.with(|probes| {
                        let mut ctx = eval::Ctx { db, conn: &mut st.conn, tables: &snap, fk_counts: &fk, index_counts: &idx, views: &views2, indexes: &idxmaps, probes, col_colls: &colls2 };
                        eval::run_stmt(&mut ctx, s)
                    });
                    match res {
                        Ok(Some(rows)) => { out.extend(rows); continue; }
                        Ok(None) => return Err(format!("unsupported statement: {}", s)),
                        Err(e) => return Err(e),
                    }
                }
            };
            // run-36: write statements respect the in-process file lock and fire
            // commit hooks when they auto-commit
            let write_kind = matches!(&stmt,
                Stmt::Insert { .. } | Stmt::Update { .. } | Stmt::Delete { .. }
                | Stmt::Create { .. } | Stmt::CreateIndex { .. } | Stmt::DropIndex { .. }
                | Stmt::CreateTrigger { .. } | Stmt::CreateView { .. } | Stmt::DropView { .. }
                | Stmt::DropTrigger { .. } | Stmt::RenameColumn { .. } | Stmt::DropColumn { .. }
                | Stmt::Drop { .. } | Stmt::RenameTable { .. } | Stmt::AddColumn { .. });
            if write_kind {
                // holding a txn: acquire and keep; autocommit: just verify nobody else holds it
                acquire_file_lock(db, st.txn.is_some())?;
            }
            // autocommit abort support: only snapshot when a commit hook is registered
            let hook_snap = if write_kind && st.txn.is_none() && crate::commit_hook_present(db) {
                Some(take_snap(st))
            } else { None };
            match stmt {
                Stmt::PragmaFkOn => { st.fk_on = true; st.conn.pragmas.insert("foreign_keys".into(), 1); }
                Stmt::Analyze { target } => {
                    // run-37: real scans -> sqlite_stat1 rows (pinned C text format)
                    ensure_stat1(st);
                    // resolve scope: whole db (None / schema name), one table, or one index
                    let mut only_index: Option<(String, String)> = None; // (table, index)
                    let mut tables: Vec<String> = Vec::new();
                    match &target {
                        None => {
                            tables = st.tables.iter().map(|(n, _)| n.clone())
                                .filter(|n| n != "sqlite_stat1").collect();
                        }
                        Some(name) if name == "main" || name == "temp" => {
                            tables = st.tables.iter().map(|(n, _)| n.clone())
                                .filter(|n| n != "sqlite_stat1").collect();
                        }
                        Some(name) => {
                            if st.tables.iter().any(|(n, _)| n == name) {
                                tables = vec![name.clone()];
                            } else if let Some(d) = st.indexes.iter().find(|d| d.name == *name) {
                                only_index = Some((d.table.clone(), d.name.clone()));
                            } else {
                                return Err(format!("no such table: {name}"));
                            }
                        }
                    }
                    if let Some((tbl, idxname)) = only_index {
                        // ANALYZE <index>: replace exactly that row (pinned C007)
                        let (cols, rows, def) = {
                            let t = st.tables.iter().find(|(n, _)| *n == tbl).ok_or("no such table")?;
                            let def = st.indexes.iter().find(|d| d.name == idxname).cloned().ok_or("no such index")?;
                            (t.1.cols.clone(), t.1.rows.clone(), def)
                        };
                        stat1_delete(st, Some(&tbl), Some(&idxname));
                        if !rows.is_empty() {
                            let stat = stat1_text(&cols, &def, &rows)?;
                            stat1_insert(st, &tbl, Some(&idxname), &stat);
                        }
                    } else {
                        for tbl in tables {
                            let (cols, rows, create_sql) = {
                                let t = st.tables.iter().find(|(n, _)| *n == tbl).ok_or("no such table")?;
                                (t.1.cols.clone(), t.1.rows.clone(), t.1.create_sql.clone())
                            };
                            let mut idefs: Vec<IndexDef> = st.indexes.iter()
                                .filter(|d| d.table == tbl).cloned().collect();
                            // WITHOUT ROWID: the PRIMARY KEY is a stat1 index named like the table
                            if create_sql.to_ascii_uppercase().contains("WITHOUT ROWID") {
                                let pks = pk_cols(&create_sql);
                                if !pks.is_empty() {
                                    idefs.push(IndexDef { name: tbl.clone(), table: tbl.clone(),
                                        exprs: pks, unique: true, where_c: None, sql: String::new() });
                                }
                            }
                            stat1_delete(st, Some(&tbl), None);
                            if rows.is_empty() { continue; } // pinned: empty tables write nothing
                            if idefs.is_empty() {
                                stat1_insert(st, &tbl, None, &rows.len().to_string());
                            } else {
                                for def in &idefs {
                                    let stat = stat1_text(&cols, def, &rows)?;
                                    stat1_insert(st, &tbl, Some(&def.name), &stat);
                                }
                            }
                        }
                    }
                }
                Stmt::Vacuum { into } => {
                    // run-34: a real rebuild — not a script answer. C refuses inside a txn.
                    if st.txn.is_some() {
                        return Err("cannot VACUUM from within a transaction".into());
                    }
                    match into {
                        Some(target) => {
                            // VACUUM INTO: write a fresh compact C-readable db at target;
                            // source is untouched. Errors match the pinned C shapes.
                            if std::path::Path::new(&target).exists() {
                                return Err("output file already exists".into());
                            }
                            let mut buf = dbfile::write_db_bytes(&image_of(st));
                            dbfile::set_journal_versions(&mut buf, false);
                            if std::fs::write(&target, buf).is_err() {
                                return Err(format!("unable to open database: {target}"));
                            }
                        }
                        None => {
                            // rebuild: implicit rowids renumber 1..n (pinned 1,3,5 -> 1,2,3);
                            // INTEGER PRIMARY KEY and WITHOUT ROWID tables keep their keys
                            for (_n, tab) in st.tables.iter_mut() {
                                let up = tab.create_sql.to_ascii_uppercase();
                                if dbfile::ipk_index(&tab.create_sql).is_some() || up.contains("WITHOUT ROWID") {
                                    continue;
                                }
                                let mut rid = 0i64;
                                for row in tab.rows.iter_mut() {
                                    rid += 1;
                                    row.0 = rid;
                                }
                                tab.next_rowid = rid;
                            }
                            // freelist model: the rebuild reclaims free pages
                            refresh_pages(st);
                            st.conn.page_hwm = st.conn.page_cur;
                            // file-backed: rewrite the main db now (and the -wal in WAL
                            // mode, so stale pre-rebuild frames cannot resurrect old rowids)
                            let path = PATHS.with(|m| m.borrow().get(&db).cloned());
                            if let Some(pb) = path {
                                let wal = st.conn.journal == "wal";
                                let mut buf = dbfile::write_db_bytes(&image_of(st));
                                dbfile::set_journal_versions(&mut buf, wal);
                                let _ = std::fs::write(&pb, &buf);
                                if wal {
                                    let _ = dbfile::write_wal(&wal_sidecar(&pb, "-wal"), &buf);
                                }
                            }
                        }
                    }
                }
                Stmt::Begin { immediate } => {
                    if st.txn.is_some() {
                        return Err("cannot start a transaction within a transaction".into());
                    }
                    if immediate { acquire_file_lock(db, true)?; } // run-36: RESERVED now
                    let snap = take_snap(st);
                    st.txn = Some(Txn { snap, implicit: false, savepoints: Vec::new() });
                }
                Stmt::Commit => {
                    if st.txn.is_none() {
                        return Err("cannot commit - no transaction is active".into());
                    }
                    // run-33: deferred FK validation happens at COMMIT; a violation fails
                    // the COMMIT and the transaction STAYS OPEN (pinned)
                    if st.fk_on && fk_violation_exists(st) {
                        return Err(FK_ERR.into());
                    }
                    // run-36: a non-zero commit hook turns COMMIT into a rollback (pinned)
                    if let Some(hrc) = crate::consult_commit_hook(db) {
                        if hrc != 0 {
                            txn_rollback(st);
                            release_file_lock(db);
                            return Err("constraint failed".into());
                        }
                    }
                    st.txn = None;
                    st.conn.pragmas.insert("defer_foreign_keys".into(), 0); // resets at txn end (pinned)
                    st.commit_flush_pending = true; // run-36: flush at the post-exec sync
                    release_file_lock(db);
                }
                Stmt::Rollback => {
                    if !txn_rollback(st) {
                        return Err("cannot rollback - no transaction is active".into());
                    }
                    st.conn.pragmas.insert("defer_foreign_keys".into(), 0);
                    release_file_lock(db);
                }
                Stmt::Savepoint { name } => {
                    let snap = take_snap(st);
                    match st.txn.as_mut() {
                        Some(tx) => tx.savepoints.push((name.to_ascii_lowercase(), snap)),
                        None => {
                            // savepoint outside a txn opens an implicit one (C semantics)
                            st.txn = Some(Txn { snap: snap.clone(), implicit: true,
                                                savepoints: vec![(name.to_ascii_lowercase(), snap)] });
                        }
                    }
                }
                Stmt::RollbackTo { name } => {
                    let key = name.to_ascii_lowercase();
                    let hit = st.txn.as_ref().and_then(|tx|
                        tx.savepoints.iter().rposition(|(n, _)| *n == key));
                    match hit {
                        Some(i) => {
                            let snap = st.txn.as_ref().unwrap().savepoints[i].1.clone();
                            restore_snap(st, snap);
                            let tx = st.txn.as_mut().unwrap();
                            tx.savepoints.truncate(i + 1); // the named savepoint survives
                        }
                        None => return Err(format!("no such savepoint: {name}")),
                    }
                }
                Stmt::Release { name } => {
                    let key = name.to_ascii_lowercase();
                    let hit = st.txn.as_ref().and_then(|tx|
                        tx.savepoints.iter().rposition(|(n, _)| *n == key));
                    match hit {
                        Some(i) => {
                            let tx = st.txn.as_mut().unwrap();
                            tx.savepoints.truncate(i);
                            if tx.savepoints.is_empty() && tx.implicit {
                                st.txn = None; // releasing the outermost implicit savepoint commits
                            }
                        }
                        None => return Err(format!("no such savepoint: {name}")),
                    }
                }
                Stmt::Create { name, cols, sql } => {
                    st.catalog.push(("table".into(), name.clone()));
                    let uniq_sets = parse_uniq_sets(&sql);
                    let checks = parse_table_checks(&sql);
                    st.tables.push((name, Table { cols, uniq_sets, checks, create_sql: sql, ..Default::default() }));
                    st.conn.schema_version += 1;
                }
                Stmt::CreateIndex { name, table, exprs, unique, where_c, sql } => {
                    st.catalog.push(("index".into(), name.clone()));
                    st.index_owner.insert(name.clone(), table.clone());
                    if unique && exprs.len() == 1 && where_c.is_none() {
                        let t = st.tables.iter_mut().find(|(n, _)| *n == table)
                            .ok_or("no such table")?;
                        if let Some(c) = t.1.cols.iter_mut().find(|c| c.name == exprs[0]) {
                            c.unique = true;
                        }
                    }
                    st.indexes.push(IndexDef { name, table, exprs, unique, where_c, sql });
                    st.conn.schema_version += 1;
                }
                Stmt::DropIndex { name } => {
                    stat1_delete(st, None, Some(&name)); // run-37: C clears the index's stat1 row
                    let dropped = st.indexes.iter().find(|d| d.name == name).cloned();
                    st.indexes.retain(|d| d.name != name);
                    st.index_owner.remove(&name);
                    st.catalog.retain(|(ty, n)| !(ty == "index" && *n == name));
                    if let Some(d) = dropped {
                        if d.unique && d.exprs.len() == 1 && d.where_c.is_none() {
                            let col = d.exprs[0].clone();
                            let table = d.table.clone();
                            let still = st.indexes.iter().any(|d2| d2.table == table && d2.unique
                                && d2.where_c.is_none() && d2.exprs.len() == 1 && d2.exprs[0] == col);
                            if let Some(t) = st.tables.iter_mut().find(|(n, _)| *n == table) {
                                let decl_unique = {
                                    let sql = &t.1.create_sql;
                                    match (sql.find('('), sql.rfind(')')) {
                                        (Some(o), Some(c2)) if c2 > o => parse_coldefs(&sql[o+1..c2])
                                            .and_then(|cs| cs.iter().find(|cc| cc.name == col).map(|cc| cc.unique)).unwrap_or(false),
                                        _ => false,
                                    }
                                };
                                if let Some(c) = t.1.cols.iter_mut().find(|c| c.name == col) {
                                    c.unique = still || decl_unique;
                                }
                            }
                        }
                    }
                    st.conn.schema_version += 1;
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
                    stat1_delete(st, Some(&name), None); // run-37: C clears the table's stat1 rows
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
                Stmt::Insert { name, collist, rows, policy, target, upd_sets, upd_where } => {
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
                                        viol = Some(format!("CHECK constraint failed: {}", chk));
                                        break;
                                    }
                                }
                            }
                            if viol.is_none() {
                                // table-level CHECK(a < b) constraints on insert too
                                let tchecks = st.tables.iter().find(|(n2, _)| *n2 == name)
                                    .map(|(_, t)| t.checks.clone()).unwrap_or_default();
                                if !tchecks.is_empty() {
                                    viol = check_row(&name, &cols_meta, &tchecks, &full)?;
                                }
                            }
                            if let Some(msg) = viol {
                                if matches!(policy, Policy::Ignore | Policy::DoNothing) { continue; }
                                if matches!(policy, Policy::TxnRollback) {
                                    return Err(format!("__TXNROLLBACK__{msg}"));
                                }
                                return Err(msg);
                            }
                            if fk_on {
                                let fk_fail = |e: String| -> String {
                                    if matches!(policy, Policy::TxnRollback) { format!("__TXNROLLBACK__{e}") } else { e }
                                };
                                let _ = &fk_fail;
                                // run-33: deferred constraints (or defer_foreign_keys=1) are
                                // checked at COMMIT when inside an explicit transaction
                                let defer_prag = st.conn.pragmas.get("defer_foreign_keys").copied().unwrap_or(0) != 0;
                                let in_txn_now = st.txn.is_some();
                                for (ci, col) in cols_meta.iter().enumerate() {
                                    if in_txn_now && (col.ref_deferred || defer_prag) { continue; }
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
                    {
                        let mut kept = Vec::with_capacity(pending.len());
                        for full in pending {
                            if fire_triggers(st, &name, 0, 0, None, Some(&full))? { kept.push(full); } // RAISE(IGNORE) drops the row
                        }
                        pending = kept;
                    }
                    let ti = st.tables.iter().position(|(n, _)| *n == name).ok_or("no such table")?;
                    let mut inserted: Vec<(String, Vec<Val>)> = Vec::new(); // for triggers
                    let mut n_changes = 0i64;
                    {
                        let idefs: Vec<IndexDef> = st.indexes.iter()
                            .filter(|d| d.table == name && d.unique).cloned().collect();
                        let t = &mut st.tables[ti].1;
                        // ON CONFLICT (<expr-list>) [WHERE <pred>]: resolve the target like
                        // sqlite3UpsertAnalyzeTarget; mismatch is the pinned C error
                        let tk: Option<UpsertTk> = match &target {
                            Some((exprs, twhere)) => Some(resolve_upsert_target(t, &idefs, exprs, twhere)?),
                            None => None,
                        };
                        for full in pending {
                            // a conflict on a constraint OTHER than the resolved target
                            // aborts with the qualified UNIQUE message (rc 19), like C
                            if let Some(tk) = &tk {
                                if let Some(msg) = other_conflict_msg(&name, t, &idefs, tk, &full)? {
                                    if matches!(policy, Policy::TxnRollback) {
                                        return Err(format!("__TXNROLLBACK__{msg}"));
                                    }
                                    return Err(msg);
                                }
                            }
                            let conflict = match &tk {
                                Some(tk) => targeted_conflict(t, tk, &full)?,
                                None => match conflict_row(t, &full) {
                                    Some(p) => Some(p),
                                    None => unique_index_conflict(t, &idefs, &full)?,
                                },
                            };
                            match conflict {
                                Some(pos) => match policy {
                                    Policy::Abort => {
                                        // run-33: C names the failing constraint (pinned)
                                        let msg = other_conflict_msg(&name, t, &idefs, &UpsertTk::Cols(Vec::new()), &full)?
                                            .unwrap_or_else(|| UNIQ_ERR.into());
                                        return Err(msg);
                                    }
                                    Policy::TxnRollback => {
                                        // OR ROLLBACK: the conflict aborts the WHOLE transaction
                                        // (rolled back by execute_script's error handler via marker)
                                        return Err(format!("__TXNROLLBACK__{UNIQ_ERR}"));
                                    }
                                    Policy::Ignore | Policy::DoNothing => continue,
                                    Policy::Replace => {
                                        t.rows.remove(pos);
                                        t.next_rowid += 1;
                                        let rid = t.next_rowid;
                                        t.rows.push((rid, full.clone()));
                                        n_changes += 1;
                                        let hrow = match dbfile::ipk_index(&t.create_sql) {
                                            Some(i) => match full.get(i) { Some(Val::Int(v)) => *v, _ => rid },
                                            None => rid,
                                        };
                                        crate::fire_update_hook(db, 18, &name, hrow); // REPLACE inserts
                                        inserted.push((name.clone(), full));
                                    }
                                    Policy::DoUpdate => {
                                        // env: excluded.* = incoming row, bare cols = existing row
                                        let mut env: std::collections::HashMap<String, eval::V> = Default::default();
                                        for (cj, cc) in t.cols.iter().enumerate() {
                                            env.insert(format!("excluded.{}", cc.name), val_to_ev(full.get(cj).unwrap_or(&Val::Null)));
                                            env.insert(cc.name.clone(), val_to_ev(t.rows[pos].1.get(cj).unwrap_or(&Val::Null)));
                                        }
                                        if let Some(w) = &upd_where {
                                            if !ev_truthy(&eval::eval_standalone(w, &env)?) { continue; }
                                        }
                                        for (uc, uexpr) in &upd_sets {
                                            let ci = t.cols.iter().position(|c| c.name == *uc)
                                                .ok_or("no such column")?;
                                            t.rows[pos].1[ci] = ev_to_val(eval::eval_standalone(uexpr, &env)?);
                                        }
                                        n_changes += 1;
                                    }
                                },
                                None => {
                                    t.next_rowid += 1;
                                    let rid = t.next_rowid;
                                    t.rows.push((rid, full.clone()));
                                    n_changes += 1;
                                    // run-36: update_hook(SQLITE_INSERT) with the IPK-aliased rowid
                                    let hrow = match dbfile::ipk_index(&t.create_sql) {
                                        Some(i) => match full.get(i) { Some(Val::Int(v)) => *v, _ => rid },
                                        None => rid,
                                    };
                                    crate::fire_update_hook(db, 18, &name, hrow);
                                    inserted.push((name.clone(), full));
                                }
                            }
                        }
                    }
                    st.changes = n_changes;
                    st.total_changes += n_changes;
                    refresh_pages(st);
                    for (_tn, full) in &inserted {
                        let _ = fire_triggers(st, &name, 1, 0, None, Some(full))?; // AFTER INSERT
                    }
                }
                Stmt::Update { name, col, add, set, wh, or_mode } => {
                    if st.views.contains_key(&name) {
                        let has_instead = st.triggers.iter().any(|(_, d)| d.table == name && d.timing == 2 && d.event == 1);
                        if !has_instead {
                            return Err(format!("cannot modify {name} because it is a view"));
                        }
                        let (vrows, vcols) = view_rows(st, &name)?;
                        let trigs: Vec<Trigger> = st.triggers.iter()
                            .filter(|(_, d)| d.table == name && d.timing == 2 && d.event == 1)
                            .map(|(_, d)| d.clone()).collect();
                        let ci = vcols.iter().position(|c| *c == col).ok_or("no such column")?;
                        let wi = wh.as_ref().and_then(|(wc, _)| vcols.iter().position(|c| c == wc));
                        for vrow in &vrows {
                            if let (Some((_, wv)), Some(wi)) = (&wh, wi) {
                                if vrow[wi] != Val::Int(*wv) { continue; }
                            }
                            let mut newr = vrow.clone();
                            newr[ci] = match (&add, &set) {
                                (Some(d), _) => match &vrow[ci] { Val::Int(i) => Val::Int(i + d), v => v.clone() },
                                (None, Some(v)) => v.clone(),
                                _ => vrow[ci].clone(),
                            };
                            let mut env: std::collections::HashMap<String, eval::V> = Default::default();
                            for (k, c) in vcols.iter().enumerate() {
                                env.insert(format!("old.{c}"), val_to_ev(&vrow[k]));
                                env.insert(format!("new.{c}"), val_to_ev(&newr[k]));
                            }
                            run_trigger_bodies(st, &trigs, &env)?;
                        }
                        continue;
                    }
                    // plan updates first so BEFORE/AFTER UPDATE triggers can fire per row.
                    // CHECK constraints are evaluated against the POST-update row image here,
                    // honouring the statement's OR-mode (v16 law):
                    //   ABORT (default): whole statement undone; FAIL: earlier rows kept;
                    //   IGNORE: violating row skipped; ROLLBACK: whole txn unwound.
                    let mut check_err: Option<String> = None;
                    let planned: Vec<(usize, Vec<Val>, Vec<Val>)> = {
                        let t = st.tables.iter().find(|(n, _)| *n == name).ok_or("no such table")?;
                        let ci = t.1.cols.iter().position(|c| c.name == col).ok_or("no such column")?;
                        let wi = wh.as_ref().and_then(|(wc, _)| t.1.cols.iter().position(|c| c.name == *wc));
                        let mut plan: Vec<(usize, Vec<Val>, Vec<Val>)> = Vec::new();
                        for (ri, (_, row)) in t.1.rows.iter().enumerate() {
                            if let (Some((_, wv)), Some(wi)) = (&wh, wi) {
                                if row[wi] != Val::Int(*wv) { continue; }
                            }
                            let mut newr = row.clone();
                            newr[ci] = match (&add, &set) {
                                (Some(d), _) => match &row[ci] { Val::Int(i) => Val::Int(i + d), v => v.clone() },
                                (None, Some(v)) => v.clone(),
                                _ => row[ci].clone(),
                            };
                            match check_row(&name, &t.1.cols, &t.1.checks, &newr)? {
                                None => plan.push((ri, row.clone(), newr)),
                                Some(msg) => match or_mode {
                                    1 => continue,                       // OR IGNORE: skip this row
                                    2 => { check_err = Some(msg); break; } // OR FAIL: keep earlier rows
                                    3 => return Err(format!("__TXNROLLBACK__{msg}")), // OR ROLLBACK
                                    _ => return Err(msg),                // ABORT: nothing applied
                                },
                            }
                        }
                        plan
                    };
                    for (_, o, nw) in &planned { let _ = fire_triggers_d(st, &name, 0, 1, Some(o), Some(nw), Some(&col), 0)?; }
                    {
                        let t = st.tables.iter_mut().find(|(n, _)| *n == name).ok_or("no such table")?;
                        let ipk_upd = dbfile::ipk_index(&t.1.create_sql);
                        for (ri, _, nw) in &planned {
                            t.1.rows[*ri].1 = nw.clone();
                            let rid = t.1.rows[*ri].0;
                            let hrow = match ipk_upd { Some(i) => match nw.get(i) { Some(Val::Int(v)) => *v, _ => rid }, None => rid };
                            crate::fire_update_hook(db, 23, &name, hrow); // run-36 SQLITE_UPDATE
                        }
                    }
                    let n = planned.len() as i64;
                    st.changes = n;
                    st.total_changes += n;
                    refresh_pages(st);
                    if let Some(msg) = check_err {
                        return Err(msg); // OR FAIL: earlier row changes stay applied
                    }
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
                    for (_, o, nw) in &planned { let _ = fire_triggers_d(st, &name, 1, 1, Some(o), Some(nw), Some(&col), 0)?; }
                }
                Stmt::Delete { name, wh, whx } => {
                    // per-row hit test shared by the trigger / delete passes
                    let expr_hit = |cols: &[Col], r: &Vec<Val>, rid: i64| -> Result<bool, String> {
                        match &whx {
                            None => Ok(true),
                            Some(w) => {
                                let mut env: std::collections::HashMap<String, eval::V> = Default::default();
                                for (cj, cc) in cols.iter().enumerate() {
                                    env.insert(cc.name.clone(), val_to_ev(r.get(cj).unwrap_or(&Val::Null)));
                                }
                                env.insert("rowid".into(), eval::V::Int(rid));
                                Ok(ev_truthy(&eval::eval_standalone(w, &env)?))
                            }
                        }
                    };
                    if st.views.contains_key(&name) {
                        let has_instead = st.triggers.iter().any(|(_, d)| d.table == name && d.timing == 2 && d.event == 2);
                        if !has_instead {
                            return Err(format!("cannot modify {name} because it is a view"));
                        }
                        let (vrows, vcols) = view_rows(st, &name)?;
                        let trigs: Vec<Trigger> = st.triggers.iter()
                            .filter(|(_, d)| d.table == name && d.timing == 2 && d.event == 2)
                            .map(|(_, d)| d.clone()).collect();
                        let wi = wh.as_ref().and_then(|(wc, _)| vcols.iter().position(|c| c == wc));
                        for vrow in &vrows {
                            if let (Some((_, wv)), Some(wi)) = (&wh, wi) {
                                if vrow[wi] != Val::Int(*wv) { continue; }
                            }
                            let mut env: std::collections::HashMap<String, eval::V> = Default::default();
                            for (k, c) in vcols.iter().enumerate() {
                                env.insert(format!("old.{c}"), val_to_ev(&vrow[k]));
                            }
                            run_trigger_bodies(st, &trigs, &env)?;
                        }
                        continue;
                    }
                    // pre-compute hit rows so BEFORE DELETE triggers can fire per row
                    let hits: Vec<Vec<Val>> = {
                        let t = st.tables.iter().find(|(n, _)| *n == name).ok_or("no such table")?;
                        let wi = wh.as_ref().and_then(|(wc, _)| t.1.cols.iter().position(|c| c.name == *wc));
                        let mut out = Vec::new();
                        for (rid, r) in &t.1.rows {
                            let hit = match (&wh, wi) {
                                (Some((_, wv)), Some(wi)) => r[wi] == Val::Int(*wv),
                                _ => expr_hit(&t.1.cols, r, *rid)?,
                            };
                            if hit { out.push(r.clone()); }
                        }
                        out
                    };
                    for o in &hits { let _ = fire_triggers(st, &name, 0, 2, Some(o), None)?; }
                    // collect deleted parent key values for cascade
                    let (deleted_keys, n): (Vec<(String, Vec<Val>)>, i64) = {
                        let t = st.tables.iter_mut().find(|(n, _)| *n == name).ok_or("no such table")?;
                        let wi = wh.as_ref().and_then(|(wc, _)| t.1.cols.iter().position(|c| c.name == *wc));
                        let before = t.1.rows.len();
                        let mut deleted: Vec<Vec<Val>> = Vec::new();
                        let cols_c = t.1.cols.clone();
                        let mut keep: Vec<(i64, Vec<Val>)> = Vec::new();
                        let ipk_del = dbfile::ipk_index(&t.1.create_sql);
                        for (rid, r) in std::mem::take(&mut t.1.rows) {
                            let hit = match (&wh, wi) {
                                (Some((_, wv)), Some(wi)) => r[wi] == Val::Int(*wv),
                                _ => expr_hit(&cols_c, &r, rid)?,
                            };
                            if hit {
                                let hrow = match ipk_del { Some(i) => match r.get(i) { Some(Val::Int(v)) => *v, _ => rid }, None => rid };
                                crate::fire_update_hook(db, 9, &name, hrow); // run-36 SQLITE_DELETE
                                deleted.push(r);
                            } else { keep.push((rid, r)); }
                        }
                        t.1.rows = keep;
                        let cols: Vec<String> = t.1.cols.iter().map(|c| c.name.clone()).collect();
                        (deleted.into_iter().map(|r| (cols.join(","), r)).collect(),
                         (before - t.1.rows.len()) as i64)
                    };
                    st.changes = n;
                    st.total_changes += n;
                    refresh_pages(st);
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
                    for o in &hits { let _ = fire_triggers(st, &name, 1, 2, Some(o), None)?; }
                }
                Stmt::Select { items, target, wh, order_by } => {
                    if target == "sqlite_master" {
                        let mut ents: Vec<(String, String)> = st.catalog.iter().filter(|(ty, n)| match &wh {
                            Some((k, v)) if k == "name" => Some(n.clone()) == v.render(),
                            Some((k, v)) if k == "type" => Some(ty.clone()) == v.render(),
                            _ => true,
                        }).cloned().collect();
                        if items.iter().any(|i| i == "count(*)") {
                            out.push(vec![Some(ents.len().to_string())]);
                            continue;
                        }
                        // run-34: type/name projections with (multi-key) ORDER BY
                        if let Some(ob) = &order_by {
                            let keys: Vec<String> = ob.split(',').map(|k| k.trim().to_ascii_lowercase()).collect();
                            ents.sort_by(|a, b| {
                                for k in &keys {
                                    let o = match k.as_str() {
                                        "type" => a.0.cmp(&b.0),
                                        "name" => a.1.cmp(&b.1),
                                        _ => std::cmp::Ordering::Equal,
                                    };
                                    if o != std::cmp::Ordering::Equal { return o; }
                                }
                                std::cmp::Ordering::Equal
                            });
                        }
                        for (ty, n) in &ents {
                            let mut row = Vec::new();
                            for it in &items {
                                match it.as_str() {
                                    "type" => row.push(Some(ty.clone())),
                                    "name" => row.push(Some(n.clone())),
                                    "tbl_name" => row.push(Some(st.index_owner.get(n).cloned().unwrap_or_else(|| n.clone()))),
                                    _ => return Err(format!("no such column: {it}")),
                                }
                            }
                            out.push(row);
                        }
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
                        if ob == "rowid" {
                            match dbfile::ipk_index(&t.1.create_sql) {
                                Some(ipk) => rows.sort_by_key(|(_, r)| match r.get(ipk) { Some(Val::Int(i)) => *i, _ => 0 }),
                                None => rows.sort_by_key(|(rid, _)| *rid), // run-34: VACUUM renumbering pins
                            }
                        } else {
                            let oi = t.1.cols.iter().position(|c| c.name == *ob).ok_or("no such column")?;
                            rows.sort_by_key(|(_, r)| match &r[oi] { Val::Int(i) => *i, _ => 0 });
                        }
                    }
                    let aggregate = items.iter().any(|i| i == "count(*)");
                    let render_item = |it: &str, rid: i64, r: &Vec<Val>| -> Result<Option<String>, String> {
                        let base = it.strip_prefix(&format!("{target}.")).unwrap_or(it);
                        if base == "changes()" { return Ok(Some(st.changes.to_string())); }
                        if base == "total_changes()" { return Ok(Some(st.total_changes.to_string())); }
                        if base == "count(*)" { return Ok(Some(t.1.rows.len().to_string())); }
                        if base == "rowid" {
                            // rowid aliases INTEGER PRIMARY KEY when declared (run-34 pin)
                            if let Some(ipk) = dbfile::ipk_index(&t.1.create_sql) {
                                return Ok(r.get(ipk).and_then(|v| v.render()));
                            }
                            return Ok(Some(rid.to_string()));
                        }
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
            // run-36: an auto-committing write consults the commit hook; a non-zero
            // return undoes the statement (C aborts the implicit txn's commit)
            if let Some(snap) = hook_snap {
                if st.txn.is_none() {
                    if let Some(hrc) = crate::consult_commit_hook(db) {
                        if hrc != 0 {
                            restore_snap(st, snap);
                            return Err("constraint failed".into());
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
            let (rc, msg) = if let Some(m) = e.strip_prefix("__TXNROLLBACK__") {
                    with_store(db, |st| { txn_rollback(st); }); // OR ROLLBACK: kill the txn
                    (19, m.to_string())
                }
                else if let Some(m) = e.strip_prefix("__RAISE__") { (19, m.to_string()) }
                else if e == FK_ERR || e == UNIQ_ERR || e.contains("constraint failed") { (19, e) }
                else if e.starts_with("unable to open database") { (14, e) } // run-34 VACUUM INTO path
                else if e == "database is locked" { (5, e) } // run-36 busy path
                else { (1, e) };
            Outcome::Done { rows: Vec::new(), rc, err: Some(msg) }
        }
    }
}
