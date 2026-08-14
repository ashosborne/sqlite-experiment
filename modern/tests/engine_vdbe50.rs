//! Run-61 INSERT bytecode replay — mirrors /tmp/h61.c.
//! EXPLAIN of INSERT INTO t VALUES(...) matches C's program (OpenWrite real
//! root, NewRowid, MakeRecord with the affinity string, Insert p4=t p5=57,
//! WRITE Transaction p2=1) and stepping the same SQL dispatches those opcodes;
//! the cell is put through pager::insert_cell (cursor packing) and read back by
//! the v48/v49 cell scans. Anti-cheat: dispatch + insert counters move on the
//! VM INSERT and not on kitchen SQL; the runtime payload comes back from cells.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }

const DDL: &str = "CREATE TABLE t(a INTEGER, b TEXT);";

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn fresh(cid: &'static str, path: &str, ddl: &str) -> H { unsafe {
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(format!("{path}-journal"));
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        if !ddl.is_empty() {
            sqlite3_exec(db, CString::new(ddl).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
        }
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, sql: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(sql).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={}", self.cid, label, rc)); return; }
        let mut buf = String::new();
        while sqlite3_step(st) == 100 {
            if !buf.is_empty() { buf.push('/'); }
            for i in 0..sqlite3_column_count(st) {
                if i > 0 { buf.push('|'); }
                let p = sqlite3_column_text(st, i as c_int);
                if p.is_null() { buf.push('~'); } else { buf.push_str(&CStr::from_ptr(p as *const c_char).to_string_lossy()); }
            }
        }
        sqlite3_finalize(st);
        self.lines.push(format!("OBS {} {} {}", self.cid, label, buf));
    } }
    fn check_keep(&mut self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-vdbe50/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
}

#[test] fn vdbe50_explain() { let _g = lockg();
    let mut h = H::fresh("engine-vdbe50-001-C001", "/tmp/v50a.db", DDL);
    h.rows("x_ins_cols", "EXPLAIN INSERT INTO t(a,b) VALUES(1,'one')");
    h.check_keep();
    h.cid = "engine-vdbe50-001-C002";
    h.rows("x_ins_plain", "EXPLAIN INSERT INTO t VALUES(2,'two')");
    h.check_keep();
    h.cid = "engine-vdbe50-001-C003";
    h.rows("x_ins_bind", "EXPLAIN INSERT INTO t(a,b) VALUES(?,?)");
    h.check();
}

#[test] fn vdbe50_execute() { let _g = lockg();
    let (d0, i0) = (vdbe::dispatch_count(), vdbe::insert_count());
    let mut h = H::fresh("engine-vdbe50-002-C001", "/tmp/v50b.db", DDL);
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"INSERT INTO t(a,b) VALUES(1,'one')".as_ptr(), -1, &mut st, ptr::null_mut());
        let rc = sqlite3_step(st);
        let last = sqlite3_last_insert_rowid(h.db);
        h.lines.push(format!("OBS {} ins_step_rc {} last {}", h.cid, rc, last));
        sqlite3_finalize(st);
    }
    h.rows("after_one", "SELECT a, b FROM t");
    h.check_keep();
    assert!(vdbe::dispatch_count() > d0, "the INSERT must dispatch");
    assert!(vdbe::insert_count() > i0, "OP_Insert must put the cell");

    h.cid = "engine-vdbe50-002-C002";
    h.ex("INSERT INTO t VALUES(2,'two');");
    h.rows("after_two", "SELECT rowid, a, b FROM t");
    unsafe {
        h.lines.push(format!("OBS {} changes {} total {}", h.cid,
            sqlite3_changes(h.db), sqlite3_total_changes(h.db)));
    }
    h.check_keep();

    h.cid = "engine-vdbe50-002-C003";
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(h.db, c"INSERT INTO t(a,b) VALUES(9,'nine')".as_ptr(), -1, &mut st, ptr::null_mut());
        let r1 = sqlite3_step(st);
        sqlite3_reset(st);
        let r2 = sqlite3_step(st);
        h.lines.push(format!("OBS {} ins_twice_rc {} {} last {}", h.cid, r1, r2, sqlite3_last_insert_rowid(h.db)));
        sqlite3_finalize(st);
    }
    h.rows("final", "SELECT rowid, a FROM t");
    h.check_keep();

    h.cid = "engine-vdbe50-002-C004";
    let mut hb = H::fresh("engine-vdbe50-002-C004", "/tmp/v50c.db", DDL);
    unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(hb.db, c"INSERT INTO t(a,b) VALUES(?,?)".as_ptr(), -1, &mut st, ptr::null_mut());
        sqlite3_bind_int64(st, 1, 5);
        sqlite3_bind_text(st, 2, c"five".as_ptr(), -1, ptr::null_mut());
        let rc = sqlite3_step(st);
        let last = sqlite3_last_insert_rowid(hb.db);
        hb.lines.push(format!("OBS {} bound_ins_rc {} last {}", hb.cid, rc, last));
        sqlite3_finalize(st);
    }
    hb.rows("after_bound", "SELECT a, b FROM t");
    hb.check();

    h.cid = "engine-vdbe50-002-C005";
    let mut hd = H::fresh("engine-vdbe50-002-C005", "/tmp/v50d.db", DDL);
    hd.ex("INSERT INTO t(a,b) VALUES(1,'one');INSERT INTO t(a,b) VALUES(2,'two');");
    hd.rows("scan", "SELECT a, b FROM t");
    hd.rows("where_eq", "SELECT b FROM t WHERE a=2");
    hd.rows("seek", "SELECT b FROM t WHERE rowid=1");
    hd.check();
    drop(h);
}

// ---- anti-cheat: cursor-path cell, kitchen boundaries, runtime payload ----

#[test] fn anti_cheat_vdbe50_boundaries() { let _g = lockg(); unsafe {
    let path = "/tmp/v50ac.db";
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(format!("{path}-journal"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
    sqlite3_exec(db, CString::new(format!("{DDL} INSERT INTO t(a,b) VALUES(1,'one');")).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    let mut jdb: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut jdb);
    sqlite3_exec(jdb, c"CREATE TABLE a(x); CREATE TABLE b(y); INSERT INTO a VALUES(1); INSERT INTO b VALUES(1);".as_ptr(), None, ptr::null_mut(), ptr::null_mut());

    // kitchen SQL moves NEITHER dispatch NOR insert counter
    let (d0, i0) = (vdbe::dispatch_count(), vdbe::insert_count());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(jdb, c"SELECT x, y FROM a, b WHERE x = y".as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    sqlite3_prepare_v2(jdb, c"INSERT INTO a SELECT y FROM b".as_ptr(), -1, &mut st, ptr::null_mut());
    let _ = sqlite3_step(st); // kitchen DML — one step is enough
    sqlite3_finalize(st);
    sqlite3_prepare_v2(db, c"SELECT length(b) FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    assert_eq!(vdbe::dispatch_count(), d0, "kitchen SQL must not dispatch");
    assert_eq!(vdbe::insert_count(), i0, "kitchen INSERT SELECT must not tick OP_Insert");

    // a v49 WHERE SELECT (read-only) must not tick the insert or cursor-write counters
    let (i1, w1) = (vdbe::insert_count(), pager::cursor_ops());
    sqlite3_prepare_v2(db, c"SELECT b FROM t WHERE a=1".as_ptr(), -1, &mut st, ptr::null_mut());
    while sqlite3_step(st) == 100 {}
    sqlite3_finalize(st);
    assert_eq!(vdbe::insert_count(), i1, "a read must not tick OP_Insert");
    assert_eq!(pager::cursor_ops(), w1, "a read must not tick the cursor-write counter");

    // the VM INSERT ticks dispatch + OP_Insert + the pager cursor-write counter
    let (d2, i2, w2) = (vdbe::dispatch_count(), vdbe::insert_count(), pager::cursor_ops());
    sqlite3_prepare_v2(db, c"INSERT INTO t(a,b) VALUES(3,'three')".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 101);
    sqlite3_finalize(st);
    assert!(vdbe::dispatch_count() > d2, "the VM INSERT must dispatch");
    assert!(vdbe::insert_count() > i2, "OP_Insert must tick");
    assert!(pager::cursor_ops() > w2, "the cell must go through the pager cursor packing");

    sqlite3_close(db);
    sqlite3_close(jdb);
} }

#[test] fn anti_cheat_vdbe50_runtime_payload() { let _g = lockg(); unsafe {
    // a payload in NO SQL literal of any frozen case: inserted through the VM,
    // read back by the v48 cell scan (cursor-read moves) after close+reopen.
    let seed = (std::process::id() % 90000) as i64 + 300_000;
    let path = format!("/tmp/v50rt-{seed}.db");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{path}-journal"));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path.as_str()).unwrap().as_ptr(), &mut db);
    sqlite3_exec(db, CString::new(DDL).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());

    let i0 = vdbe::insert_count();
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let sql = CString::new(format!("INSERT INTO t(a,b) VALUES({seed},'p{seed}')")).unwrap();
    sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 101);
    sqlite3_finalize(st);
    assert!(vdbe::insert_count() > i0, "the runtime INSERT must go through OP_Insert");
    sqlite3_close(db);

    // reopen: the row must come from the FILE's cells, via the v48 scan
    let mut db2: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path.as_str()).unwrap().as_ptr(), &mut db2);
    let (d1, c1) = (vdbe::dispatch_count(), vdbe::cursor_read_count());
    sqlite3_prepare_v2(db2, c"SELECT a, b FROM t".as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(sqlite3_column_int64(st, 0), seed);
    let b = CStr::from_ptr(sqlite3_column_text(st, 1) as *const c_char).to_string_lossy().to_string();
    assert_eq!(b, format!("p{seed}"), "the runtime payload must come from the cell");
    assert_eq!(sqlite3_step(st), 101);
    sqlite3_finalize(st);
    assert!(vdbe::dispatch_count() > d1 && vdbe::cursor_read_count() > c1,
        "the read-back must be the v48 cell scan");
    sqlite3_close(db2);
    let _ = std::fs::remove_file(&path);
} }
