//! Run-35 incremental blob I/O replay — mirrors /tmp/blob_harness.c, asserted
//! byte-identical against the frozen C goldens. Plain language: open a handle on
//! one cell, read/write bytes at an offset, and the handle dies (rc 4) if the
//! row changes underneath it.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::process::Command;
use std::ptr;

fn pin_cli() -> String { std::env::var("SQLITE_PIN_BIN").unwrap_or_else(|_| "/tmp/sqlite-build/sqlite3".into()) }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3, path: String }
impl H {
    fn open(cid: &'static str, path: &str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(path).unwrap().as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db, path: path.into() } } }
    fn new(cid: &'static str) -> H { H::open(cid, ":memory:") }
    fn fresh(cid: &'static str, path: &str) -> H {
        for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
        std::fs::create_dir_all("/tmp/eftest").unwrap();
        H::open(cid, path) }
    fn reopen(&mut self) { unsafe {
        sqlite3_close(self.db);
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(self.path.clone()).unwrap().as_ptr(), &mut db);
        self.db = db; } }
    fn ex(&mut self, s: &str) { unsafe { sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn exr(&mut self, label: &str, s: &str) { unsafe {
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let e = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={}", self.cid, label, rc, e));
        sqlite3_free(em as *mut c_void);
    } }
    fn oi(&mut self, n: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn os(&mut self, n: &str, v: &str) { self.lines.push(format!("OBS {} {} {}", self.cid, n, v)); }
    fn oerr(&mut self, label: &str, rc: c_int) { unsafe {
        let e = CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy().into_owned();
        self.lines.push(format!("OBS {} {} rc={} err={}", self.cid, label, rc, e));
    } }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 {
            let e = CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy().into_owned();
            self.lines.push(format!("OBS {} {} prep.rc={} err={}", self.cid, label, rc, e));
            return;
        }
        let mut buf = String::new();
        while sqlite3_step(st) == 100 {
            if !buf.is_empty() { buf.push('|'); }
            for i in 0..sqlite3_column_count(st) {
                if i > 0 { buf.push(','); }
                let p = sqlite3_column_text(st, i as c_int);
                if p.is_null() { buf.push('~'); } else { buf.push_str(&CStr::from_ptr(p as *const c_char).to_string_lossy()); }
            }
        }
        sqlite3_finalize(st);
        self.lines.push(format!("OBS {} {} {}", self.cid, label, buf));
    } }
    fn check(self) {
        unsafe { sqlite3_close(self.db); }
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let slice: String = feat.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
    }
}
unsafe fn bopen(db: *mut Sqlite3, tbl: &str, col: &str, row: i64, flags: c_int, b: *mut *mut Sqlite3Blob) -> c_int {
    sqlite3_blob_open(db, c"main".as_ptr(), CString::new(tbl).unwrap().as_ptr(), CString::new(col).unwrap().as_ptr(), row, flags, b)
}

#[test] fn a001() { unsafe { let mut h = H::new("engine-blob-001-C001");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'48454C4C4F');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    let rc = bopen(h.db, "t", "d", 1, 0, &mut b);
    h.oerr("open", rc);
    h.oi("bytes", sqlite3_blob_bytes(b) as i64);
    h.oi("close", sqlite3_blob_close(b) as i64);
    h.check(); } }

#[test] fn a002() { unsafe { let mut h = H::new("engine-blob-001-C002");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'0102030405');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    let rc = bopen(h.db, "t", "d", 1, 1, &mut b);
    h.oerr("open_rw", rc);
    h.oi("bytes", sqlite3_blob_bytes(b) as i64);
    h.oi("close", sqlite3_blob_close(b) as i64);
    h.check(); } }

#[test] fn a003() { unsafe { let mut h = H::new("engine-blob-001-C003");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB);INSERT INTO t VALUES(1, X'AAAA'),(2, X'BBBBBBBB');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 0, &mut b);
    h.oi("bytes1", sqlite3_blob_bytes(b) as i64);
    let rc = sqlite3_blob_reopen(b, 2);
    h.oerr("reopen", rc);
    h.oi("bytes2", sqlite3_blob_bytes(b) as i64);
    let mut buf = [0u8; 8];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 4, 0);
    h.oerr("read", rc);
    let hexs = format!("{:02X}{:02X}{:02X}{:02X}", buf[0], buf[1], buf[2], buf[3]);
    h.os("hex", &hexs);
    h.oi("close", sqlite3_blob_close(b) as i64);
    h.check(); } }

#[test] fn a004() { unsafe { let mut h = H::new("engine-blob-001-C004");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'00');");
    let mut b: *mut Sqlite3Blob = 1usize as *mut Sqlite3Blob;
    let rc = bopen(h.db, "t", "d", 99, 0, &mut b);
    h.oerr("open99", rc);
    h.oi("handle_null", b.is_null() as i64);
    bopen(h.db, "t", "d", 1, 0, &mut b);
    let rc = sqlite3_blob_reopen(b, 99);
    h.oerr("reopen99", rc);
    h.oi("close", sqlite3_blob_close(b) as i64);
    h.check(); } }

#[test] fn a005() { unsafe { let mut h = H::new("engine-blob-001-C005");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'00');CREATE VIEW v AS SELECT d FROM t;");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    let rc = bopen(h.db, "nope", "d", 1, 0, &mut b); h.oerr("no_table", rc);
    let rc = bopen(h.db, "t", "zz", 1, 0, &mut b); h.oerr("no_col", rc);
    let rc = bopen(h.db, "v", "d", 1, 0, &mut b); h.oerr("view", rc);
    h.check(); } }

#[test] fn a006() { unsafe { let mut h = H::new("engine-blob-001-C006");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB, u BLOB UNIQUE);INSERT INTO t VALUES(1, X'00', X'11');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    let rc = bopen(h.db, "t", "u", 1, 1, &mut b); h.oerr("rw_indexed", rc);
    let rc = bopen(h.db, "t", "u", 1, 0, &mut b); h.oerr("ro_indexed", rc);
    if rc == 0 { sqlite3_blob_close(b); }
    h.check(); } }

#[test] fn a007() { unsafe { let mut h = H::new("engine-blob-001-C007");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d); INSERT INTO t VALUES(1, 'texty');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    let rc = bopen(h.db, "t", "d", 1, 0, &mut b); h.oerr("open_text", rc);
    h.oi("bytes", sqlite3_blob_bytes(b) as i64);
    let mut buf = [0u8; 16];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 5, 0); h.oerr("read", rc);
    let s = String::from_utf8_lossy(&buf[..5]).into_owned();
    h.os("text", &s);
    h.oi("close", sqlite3_blob_close(b) as i64);
    h.check(); } }

#[test] fn a008() { unsafe { let mut h = H::new("engine-blob-001-C008");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, NULL);");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    let rc = bopen(h.db, "t", "d", 1, 0, &mut b); h.oerr("open_null", rc);
    h.check(); } }

#[test] fn b001() { unsafe { let mut h = H::new("engine-blob-002-C001");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'4142434445');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 0, &mut b);
    let mut buf = [0u8; 16];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 5, 0); h.oerr("read", rc);
    let s = String::from_utf8_lossy(&buf[..5]).into_owned();
    h.os("data", &s);
    sqlite3_blob_close(b);
    h.check(); } }

#[test] fn b002() { unsafe { let mut h = H::new("engine-blob-002-C002");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'414243444546');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 0, &mut b);
    let mut buf = [0u8; 16];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 3, 2); h.oerr("read", rc);
    let s = String::from_utf8_lossy(&buf[..3]).into_owned();
    h.os("slice", &s);
    sqlite3_blob_close(b);
    h.check(); } }

#[test] fn b003() { unsafe { let mut h = H::new("engine-blob-002-C003");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'4142');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 0, &mut b);
    let mut buf = [b'Z'; 16];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 5, 0); h.oerr("past_end", rc);
    h.oi("untouched", (buf[0] == b'Z') as i64);
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 1, -1); h.oerr("neg_off", rc);
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 2, 1); h.oerr("off_plus_n", rc);
    sqlite3_blob_close(b);
    h.check(); } }

#[test] fn b004() { unsafe { let mut h = H::new("engine-blob-002-C004");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'4141414141');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 1, &mut b);
    let rc = sqlite3_blob_write(b, c"xy".as_ptr() as *const c_void, 2, 1); h.oerr("write", rc);
    h.oi("bytes", sqlite3_blob_bytes(b) as i64);
    sqlite3_blob_close(b);
    h.rows("cell", "SELECT hex(d), length(d) FROM t");
    h.check(); } }

#[test] fn b005() { unsafe { let mut h = H::new("engine-blob-002-C005");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'414141');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 0, &mut b);
    let rc = sqlite3_blob_write(b, c"x".as_ptr() as *const c_void, 1, 0); h.oerr("ro_write", rc);
    sqlite3_blob_close(b);
    h.check(); } }

#[test] fn b006() { unsafe { let mut h = H::new("engine-blob-002-C006");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'4141');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 1, &mut b);
    let rc = sqlite3_blob_write(b, c"xyz".as_ptr() as *const c_void, 3, 0); h.oerr("too_long", rc);
    let rc = sqlite3_blob_write(b, c"x".as_ptr() as *const c_void, 1, 2); h.oerr("at_end", rc);
    sqlite3_blob_close(b);
    h.rows("cell", "SELECT hex(d) FROM t");
    h.check(); } }

#[test] fn b007() { unsafe { let mut h = H::new("engine-blob-002-C007");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, zeroblob(8));");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 1, &mut b);
    let rc = sqlite3_blob_write(b, c"MID".as_ptr() as *const c_void, 3, 3); h.oerr("write", rc);
    sqlite3_blob_close(b);
    h.rows("cell", "SELECT hex(d) FROM t");
    h.check(); } }

#[test] fn b008() { unsafe { let mut h = H::new("engine-blob-002-C008");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'41414141');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 1, &mut b);
    h.exr("upd", "UPDATE t SET d = X'42424242' WHERE id=1;");
    let mut buf = [0u8; 8];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 4, 0); h.oerr("read_after", rc);
    let rc = sqlite3_blob_write(b, c"x".as_ptr() as *const c_void, 1, 0); h.oerr("write_after", rc);
    h.oi("bytes_after", sqlite3_blob_bytes(b) as i64);
    sqlite3_blob_close(b);
    h.check(); } }

#[test] fn b009() { unsafe { let mut h = H::new("engine-blob-002-C009");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'4141');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 0, &mut b);
    h.exr("del", "DELETE FROM t WHERE id=1;");
    let mut buf = [0u8; 8];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 2, 0); h.oerr("read_after", rc);
    sqlite3_blob_close(b);
    h.check(); } }

#[test] fn b010() { unsafe { let mut h = H::new("engine-blob-002-C010");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, X'');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    let rc = bopen(h.db, "t", "d", 1, 0, &mut b); h.oerr("open", rc);
    h.oi("bytes", sqlite3_blob_bytes(b) as i64);
    let mut buf = [0u8; 4];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 0, 0); h.oerr("read0", rc);
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 1, 0); h.oerr("read1", rc);
    sqlite3_blob_close(b);
    h.check(); } }

#[test] fn c001_c002() { unsafe {
    let path = "/tmp/eftest/rw_blob_c1.db";
    let mut h = H::fresh("engine-blob-003-C001", path);
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO t VALUES(1, zeroblob(6));");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 1, &mut b);
    let rc = sqlite3_blob_write(b, c"HELLO!".as_ptr() as *const c_void, 6, 0); h.oerr("write", rc);
    sqlite3_blob_close(b);
    h.reopen();
    h.rows("reopen", "SELECT d, length(d) FROM t");
    h.rows("integ", "PRAGMA integrity_check");
    h.check();
    let mut h = H::open("engine-blob-003-C002", path);
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 0, &mut b);
    let mut buf = [0u8; 16];
    let rc = sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 6, 0); h.oerr("read", rc);
    let s = String::from_utf8_lossy(&buf[..6]).into_owned();
    h.os("data", &s);
    sqlite3_blob_close(b);
    h.check();
} }

#[test] fn c003() { unsafe { let mut h = H::new("engine-blob-003-C003");
    h.ex("CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB, tag TEXT);INSERT INTO t VALUES(1, X'6161616161', 'keep');");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 1, &mut b);
    sqlite3_blob_write(b, c"ZZ".as_ptr() as *const c_void, 2, 0);
    sqlite3_blob_close(b);
    h.rows("row", "SELECT d, tag FROM t");
    h.rows("agg", "SELECT count(*), length(d) FROM t");
    h.check(); } }

#[test] fn c004() { unsafe {
    let path = "/tmp/eftest/rw_blob_c4.db";
    let mut h = H::fresh("engine-blob-003-C004", path);
    h.ex("PRAGMA journal_mode=WAL; CREATE TABLE t(id INTEGER PRIMARY KEY, d BLOB);INSERT INTO t VALUES(1, zeroblob(4));");
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    bopen(h.db, "t", "d", 1, 1, &mut b);
    let rc = sqlite3_blob_write(b, c"WALb".as_ptr() as *const c_void, 4, 0); h.oerr("write", rc);
    sqlite3_blob_close(b);
    h.rows("mode", "PRAGMA journal_mode");
    h.reopen();
    h.rows("reopen", "SELECT d FROM t");
    h.check();
} }

// ---- MANDATORY anti-cheat: runtime payload / offsets / expiry ----
#[test] fn anti_cheat_blob_runtime() { unsafe {
    let seed = std::process::id() % 100000;
    let tname = format!("bt{seed}");
    let payload = format!("payload-{seed}-abcdefgh");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(":memory:").unwrap().as_ptr(), &mut db);
    let ex = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    ex(db, &format!("CREATE TABLE {tname}(id INTEGER PRIMARY KEY, d BLOB);"));
    ex(db, &format!("INSERT INTO {tname} VALUES(7, zeroblob({}));", payload.len()));
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    assert_eq!(sqlite3_blob_open(db, c"main".as_ptr(), CString::new(tname.clone()).unwrap().as_ptr(), c"d".as_ptr(), 7, 1, &mut b), 0);
    // runtime-chosen offset write then read-back
    let off = (seed % 5) as usize;
    let chunk = &payload.as_bytes()[off..off + 8];
    assert_eq!(sqlite3_blob_write(b, chunk.as_ptr() as *const c_void, 8, off as c_int), 0);
    let mut buf = vec![0u8; 8];
    assert_eq!(sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 8, off as c_int), 0);
    assert_eq!(&buf, chunk);
    // expiry anti-cheat: touching the row kills the live handle
    ex(db, &format!("UPDATE {tname} SET d = zeroblob(4) WHERE id=7;"));
    assert_eq!(sqlite3_blob_read(b, buf.as_mut_ptr() as *mut c_void, 1, 0), 4, "handle must expire");
    assert_eq!(sqlite3_blob_bytes(b), 0);
    sqlite3_blob_close(b);
    sqlite3_close(db);
} }

// ---- MANDATORY C interop: pinned C reads a Rust file after blob_write ----
#[test] fn rust_blob_write_c_read() { unsafe {
    let n = std::process::id() % 100000;
    let path = format!("/tmp/eftest/rbwc_{n}.db");
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
    let word = format!("BLOB{n:05}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(CString::new(path.clone()).unwrap().as_ptr(), &mut db);
    let ex = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    ex(db, &format!("CREATE TABLE f(id INTEGER PRIMARY KEY, d BLOB); INSERT INTO f VALUES(1, zeroblob({}));", word.len()));
    let mut b: *mut Sqlite3Blob = ptr::null_mut();
    assert_eq!(sqlite3_blob_open(db, c"main".as_ptr(), c"f".as_ptr(), c"d".as_ptr(), 1, 1, &mut b), 0);
    assert_eq!(sqlite3_blob_write(b, word.as_ptr() as *const c_void, word.len() as c_int, 0), 0);
    sqlite3_blob_close(b);
    sqlite3_close(db);
    let cli = pin_cli();
    let out = Command::new(&cli).arg(&path).arg("SELECT d FROM f;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), word,
               "pinned C must see the handle-written bytes (stderr: {})", String::from_utf8_lossy(&out.stderr));
    let ic = Command::new(&cli).arg(&path).arg("PRAGMA integrity_check;").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&ic.stdout).trim(), "ok");
    for sfx in ["", "-wal", "-shm"] { let _ = std::fs::remove_file(format!("{path}{sfx}")); }
} }

#[test] fn script_table_still_empty() {
    assert_eq!(sqlite3_rust_spine::script_table::SCRIPT_TABLE.len(), 0);
}
