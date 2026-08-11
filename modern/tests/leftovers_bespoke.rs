//! Run-12 bespoke pins: auto_extension registry, deserialize round-trip, backup write-between-steps.
use sqlite3_rust_spine::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

static mut AUTO_CALLS: i32 = 0;
unsafe extern "C" fn my_auto_init(_db: *mut Sqlite3, _e: *mut *mut c_char, _api: *const c_void) -> c_int {
    AUTO_CALLS += 1;
    0
}

#[test]
fn loadext_002_auto_extension_registry() {
    unsafe {
        let f: unsafe extern "C" fn() = std::mem::transmute(my_auto_init as usize);
        assert_eq!(sqlite3_auto_extension(Some(f)), 0);      // register.rc
        AUTO_CALLS = 0;
        let mut d1: *mut Sqlite3 = ptr::null_mut();
        let n = CString::new(":memory:").unwrap();
        sqlite3_open(n.as_ptr(), &mut d1);
        assert_eq!(AUTO_CALLS, 1);                            // calls.after_open
        assert_eq!(sqlite3_cancel_auto_extension(Some(f)), 1); // cancel.rc (pinned 1)
        let mut d2: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(n.as_ptr(), &mut d2);
        assert_eq!(AUTO_CALLS, 1);                            // calls.after_cancel_open
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
