//! Run-12 bespoke pins: auto_extension registry, deserialize round-trip, backup write-between-steps.
use sqlite3_rust_spine::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

use std::sync::{Mutex, OnceLock};
fn inited_dbs() -> &'static Mutex<Vec<usize>> {
    static S: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(Vec::new()))
}
unsafe extern "C" fn my_auto_init(db: *mut Sqlite3, _e: *mut *mut c_char, _api: *const c_void) -> c_int {
    // record which connection got auto-inited (robust vs parallel opens from other tests)
    inited_dbs().lock().unwrap().push(db as usize);
    0
}

#[test]
fn loadext_002_auto_extension_registry() {
    unsafe {
        let f: unsafe extern "C" fn() = std::mem::transmute(my_auto_init as usize);
        assert_eq!(sqlite3_auto_extension(Some(f)), 0);      // register.rc
        let mut d1: *mut Sqlite3 = ptr::null_mut();
        let n = CString::new(":memory:").unwrap();
        sqlite3_open(n.as_ptr(), &mut d1);
        assert!(inited_dbs().lock().unwrap().contains(&(d1 as usize)), "auto-ext fired on d1");
        assert_eq!(sqlite3_cancel_auto_extension(Some(f)), 1); // cancel.rc (pinned 1)
        let mut d2: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(n.as_ptr(), &mut d2);
        assert!(!inited_dbs().lock().unwrap().contains(&(d2 as usize)), "auto-ext NOT fired on d2 after cancel");
        sqlite3_close(d1); sqlite3_close(d2);
    }
}

#[test]
fn serialize_002_deserialize_roundtrip() {
    unsafe {
        let mut src: *mut Sqlite3 = ptr::null_mut();
        let mut dst: *mut Sqlite3 = ptr::null_mut();
        let n = CString::new(":memory:").unwrap();
        let m = CString::new("main").unwrap();
        sqlite3_open(n.as_ptr(), &mut src);
        let mut sz: i64 = 0;
        let img = sqlite3_serialize(src, m.as_ptr(), &mut sz, 0);
        sqlite3_open(n.as_ptr(), &mut dst);
        assert_eq!(sqlite3_deserialize(dst, m.as_ptr(), img, sz, sz, 1 | 2), 0); // deserialize.rc
        let mut sz2: i64 = 0;
        let img2 = sqlite3_serialize(dst, m.as_ptr(), &mut sz2, 0);
        assert_eq!(sz2, 4096);          // reserialize.size
        assert_eq!(sz2, sz);            // roundtrip.same_size
        sqlite3_free(img2 as *mut c_void);
        sqlite3_close(src); sqlite3_close(dst);
    }
}

#[test]
fn backup_003_write_between_steps_rcs() {
    unsafe {
        let mut src: *mut Sqlite3 = ptr::null_mut();
        let mut dst: *mut Sqlite3 = ptr::null_mut();
        let n = CString::new(":memory:").unwrap();
        let m = CString::new("main").unwrap();
        sqlite3_open(n.as_ptr(), &mut src);
        // run-47: the backup engine is now a REAL copy — recreate the run-12 C
        // harness's written 2-page source instead of the old state-machine shim
        let sql = CString::new("CREATE TABLE t(a); INSERT INTO t VALUES(1);").unwrap();
        sqlite3_exec(src, sql.as_ptr(), None, ptr::null_mut(), ptr::null_mut());
        sqlite3_open(n.as_ptr(), &mut dst);
        let b = sqlite3_backup_init(dst, m.as_ptr(), src, m.as_ptr());
        assert!(!b.is_null());
        assert_eq!(sqlite3_backup_step(b, 1), 0);            // step1.rc (partial, pages remain)
        assert_eq!(sqlite3_backup_remaining(b), 1);          // pinned
        assert_eq!(sqlite3_backup_pagecount(b), 2);          // pinned
        assert_eq!(sqlite3_backup_step(b, -1), 101);         // step_rest.rc DONE
        assert_eq!(sqlite3_backup_finish(b), 0);
        sqlite3_close(src); sqlite3_close(dst);
    }
}
