//! run-55 helper: write a committed db file via the modern pager to a path given
//! by PAGER44_OUT, so an external pinned-C reader can prove the file is
//! C-readable. Only runs when PAGER44_OUT is set (no-op otherwise).
use sqlite3_rust_spine::*;
use std::ffi::CString;
use std::ptr;

#[test]
fn pager44_write_committed_file() {
    let out = match std::env::var("PAGER44_OUT") { Ok(v) => v, Err(_) => return };
    let _ = std::fs::remove_file(&out);
    let _ = std::fs::remove_file(format!("{out}-journal"));
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(out.clone()).unwrap().as_ptr(), &mut db);
        let run = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
        run(db, "PRAGMA journal_mode=DELETE;");
        run(db, "CREATE TABLE t(a INTEGER PRIMARY KEY, b);");
        run(db, "INSERT INTO t VALUES(1,'one'),(2,'two'),(3,'three');");
        run(db, "BEGIN IMMEDIATE;");
        run(db, "INSERT INTO t VALUES(4,'pager-made-me');");
        run(db, "COMMIT;");
        sqlite3_close(db);
    }
    assert!(!std::path::Path::new(&format!("{out}-journal")).exists(), "journal gone after commit");
}
