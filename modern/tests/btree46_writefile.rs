//! run-57 helper: write the deterministic SPLIT db through the modern cursor
//! path to BTREE46_OUT so the pinned C amalgamation (the RECORD harness binary)
//! can open it — that C read is frozen as engine-btree46-002. No-op without env.
use sqlite3_rust_spine::*;
use std::ffi::CString;
use std::ptr;

#[test]
fn btree46_write_split_file() {
    let out = match std::env::var("BTREE46_OUT") { Ok(v) => v, Err(_) => return };
    let _ = std::fs::remove_file(&out);
    let _ = std::fs::remove_file(format!("{out}-journal"));
    let s0 = pager::split_count();
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(CString::new(out.clone()).unwrap().as_ptr(), &mut db);
        let run = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
        run(db, "PRAGMA journal_mode=DELETE;");
        run(db, "CREATE TABLE t(k INTEGER PRIMARY KEY, v);");
        run(db, "INSERT INTO t VALUES(1,'aaaa'),(2,'bbbb'),(3,'cccc'),(4,'dddd');");
        for i in 10..22 {
            let pay: String = std::iter::repeat(char::from(b'a' + (i % 26) as u8)).take(500).collect();
            run(db, &format!("INSERT INTO t VALUES({i},'{pay}');"));
        }
        sqlite3_close(db);
    }
    assert!(pager::split_count() > s0, "the modern write must have taken the cursor SPLIT path");
    // sanity: root page of the modern file is an interior 0x05 with >= 2 leaves
    let img = std::fs::read(&out).unwrap();
    assert_eq!(img[4096], 0x05, "modern root page must be a table interior");
}
