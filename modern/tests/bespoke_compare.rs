//! Run-11 bespoke API pins — asserts the frozen integers from the goldens.
//! (Golden files read where values are plain ints; derived booleans re-derived.)
mod util;
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

unsafe fn open_mem() -> *mut Sqlite3 {
    let mut db: *mut Sqlite3 = ptr::null_mut();
    let n = CString::new(":memory:").unwrap();
    assert_eq!(sqlite3_open(n.as_ptr(), &mut db), 0);
    db
}

#[test]
fn global_init_config_pins() {
    unsafe {
        assert_eq!(sqlite3_initialize(), 0);
        assert_eq!(sqlite3_initialize(), 0);                    // init twice -> 0,0
        assert_eq!(sqlite3_config(2 /*MULTITHREAD*/, 0, 0), 21); // after init -> MISUSE (run-47 generic arity)
        let db = open_mem();
        let mut v: c_int = -1;
        // run-45: db_config is a generic fixed-arity export (val, out*) for this verb
        sqlite3_db_config(db, SQLITE_DBCONFIG_ENABLE_FKEY, -1, &mut v as *mut c_int as i64, 0);
        assert_eq!(v, 0);                                        // pinned default
        sqlite3_db_config(db, SQLITE_DBCONFIG_ENABLE_FKEY, 1, &mut v as *mut c_int as i64, 0);
        assert_eq!(v, 1);                                        // pinned after set
        sqlite3_close(db);
    }
}

#[test]
fn error_status_002_limit_protocol() {
    unsafe {
        let db = open_mem();
        assert_eq!(sqlite3_limit(db, SQLITE_LIMIT_VARIABLE_NUMBER, -1), 32766);
        assert_eq!(sqlite3_limit(db, SQLITE_LIMIT_VARIABLE_NUMBER, 999), 32766); // returns prior
        assert_eq!(sqlite3_limit(db, SQLITE_LIMIT_VARIABLE_NUMBER, -1), 999);
        sqlite3_close(db);
    }
}

#[test]
fn prepare_004_column_coercion() {
    unsafe {
        let db = open_mem();
        let sql = CString::new("SELECT '42abc'").unwrap();
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        assert_eq!(sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut()), 0);
        sqlite3_step(st);
        assert_eq!(sqlite3_column_type(st, 0), 3);   // TEXT before coercion
        assert_eq!(sqlite3_column_int(st, 0), 42);   // pinned prefix coercion
        sqlite3_finalize(st);
        sqlite3_close(db);
    }
}

#[test]
fn prepare_006_stmt_introspection() {
    unsafe {
        let db = open_mem();
        let sql = CString::new("SELECT 1").unwrap();
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut());
        assert_eq!(sqlite3_stmt_readonly(st), 1);
        assert_eq!(sqlite3_stmt_busy(st), 0);
        sqlite3_step(st);
        assert_eq!(sqlite3_stmt_busy(st), 1);
        sqlite3_finalize(st);
        sqlite3_close(db);
    }
}

static mut AUTH_CALLS: i32 = 0;
unsafe extern "C" fn deny_select(_p: *mut c_void, code: c_int, _a: *const c_char, _b: *const c_char, _c: *const c_char, _d: *const c_char) -> c_int {
    AUTH_CALLS += 1;
    if code == SQLITE_SELECT_ACTION { 1 /*DENY*/ } else { 0 }
}

#[test]
fn auth_001_deny_select() {
    unsafe {
        let db = open_mem();
        AUTH_CALLS = 0;
        sqlite3_set_authorizer(db, Some(deny_select), ptr::null_mut());
        let sql = CString::new("SELECT 1").unwrap();
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut());
        assert_eq!(rc, 23);                    // pinned SQLITE_AUTH
        assert!(AUTH_CALLS > 0);               // cb.called
        assert!(st.is_null());
        sqlite3_set_authorizer(db, None, ptr::null_mut());
        sqlite3_close(db);
    }
}

#[test]
fn loadext_001_disabled() {
    unsafe {
        let db = open_mem();
        let f = CString::new("/nonexistent-ext").unwrap();
        let mut err: *mut c_char = ptr::null_mut();
        let rc = sqlite3_load_extension(db, f.as_ptr(), ptr::null(), &mut err);
        assert_eq!(rc, 1);                                                  // pinned
        assert_eq!(CStr::from_ptr(err).to_str().unwrap(), "not authorized"); // pinned bool observable
        sqlite3_free(err as *mut c_void);
        sqlite3_close(db);
    }
}

#[test]
fn backup_pins_empty_pair() {
    unsafe {
        let src = open_mem();
        let dst = open_mem();
        let m = CString::new("main").unwrap();
        let b = sqlite3_backup_init(dst, m.as_ptr(), src, m.as_ptr());
        assert!(!b.is_null());                          // init.nonnull
        assert_eq!(sqlite3_backup_step(b, -1), 101);    // step_all.rc DONE
        assert_eq!(sqlite3_backup_remaining(b), 0);
        assert_eq!(sqlite3_backup_pagecount(b), 0);
        assert_eq!(sqlite3_backup_finish(b), 0);
        sqlite3_close(src); sqlite3_close(dst);
    }
}

#[test]
fn serialize_001_empty_memory() {
    unsafe {
        let db = open_mem();
        let m = CString::new("main").unwrap();
        let mut sz: i64 = -1;
        let p = sqlite3_serialize(db, m.as_ptr(), &mut sz, 0);
        assert!(!p.is_null());        // ptr.nonnull
        assert_eq!(sz, 4096);         // pinned size
        sqlite3_free(p as *mut c_void);
        sqlite3_close(db);
    }
}

#[test]
fn malloc_mutex_random_pins() {
    unsafe {
        let p = sqlite3_malloc64(64);
        assert!(!p.is_null());
        assert!(sqlite3_msize(p) >= 64);
        sqlite3_free(p);
        let m = sqlite3_mutex_alloc(SQLITE_MUTEX_FAST);
        assert!(!m.is_null());
        sqlite3_mutex_enter(m); sqlite3_mutex_leave(m); sqlite3_mutex_free(m);
        let mut b1 = [0u8; 8]; let mut b2 = [0u8; 8];
        sqlite3_randomness(8, b1.as_mut_ptr() as *mut c_void);
        sqlite3_randomness(8, b2.as_mut_ptr() as *mut c_void);
        assert_ne!(b1, b2);           // draws.differ
    }
}

#[test]
fn printf_pins() {
    unsafe {
        let f = CString::new("%d-%Q").unwrap();
        let z = sqlite3_mprintf(f.as_ptr(), 5, ptr::null());
        assert_eq!(CStr::from_ptr(z).to_str().unwrap(), "5-NULL"); // pinned
        sqlite3_free(z as *mut c_void);
        let db = open_mem();
        let s = sqlite3_str_new(db);
        let f2 = CString::new("%d/%s").unwrap();
        let ab = CString::new("ab").unwrap();
        sqlite3_str_appendf(s, f2.as_ptr(), 3, ab.as_ptr());
        assert_eq!(sqlite3_str_errcode(s), 0);
        let out = sqlite3_str_finish(s);
        assert_eq!(CStr::from_ptr(out).to_str().unwrap(), "3/ab"); // pinned
        sqlite3_free(out as *mut c_void);
        sqlite3_close(db);
    }
}

#[test]
fn tokenizer_002_complete_pins() {
    unsafe {
        let a = CString::new("SELECT 1;").unwrap();
        let b = CString::new("SELECT 1").unwrap();
        let c = CString::new("CREATE TRIGGER t BEGIN SELECT 1;").unwrap();
        assert_eq!(sqlite3_complete(a.as_ptr()), 1);
        assert_eq!(sqlite3_complete(b.as_ptr()), 0);
        assert_eq!(sqlite3_complete(c.as_ptr()), 0);
    }
}
