//! Run-53 kitchen-12h replay — mirrors /tmp/h43.c.
//! JSON array paths; pragma_index_list/foreign_key_list projections; TEMP schema
//! isolation + triggers; auth TEMP outer codes (filtered logs).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());
fn lockg() -> std::sync::MutexGuard<'static, ()> { LOCK.lock().unwrap_or_else(|e| e.into_inner()) }
static ALOG: Mutex<String> = Mutex::new(String::new());
static FILTER: Mutex<Vec<i32>> = Mutex::new(Vec::new());
static DENY_CODE: Mutex<i32> = Mutex::new(0);

fn fs(p: *const c_char) -> String {
    if p.is_null() { "~".into() }
    else { let s = unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() };
        if s.is_empty() { "{}".into() } else { s } }
}
unsafe extern "C" fn auth_cb(_c: *mut c_void, code: c_int, s1: *const c_char, s2: *const c_char,
        s3: *const c_char, s4: *const c_char) -> c_int {
    let filt = FILTER.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if filt.is_empty() || filt.contains(&code) {
        ALOG.lock().unwrap_or_else(|e| e.into_inner())
            .push_str(&format!("[{}|{}|{}|{}|{}]", code, fs(s1), fs(s2), fs(s3), fs(s4)));
    }
    if code == *DENY_CODE.lock().unwrap_or_else(|e| e.into_inner()) { return 1; }
    0
}
fn take_log() -> String { std::mem::take(&mut *ALOG.lock().unwrap_or_else(|e| e.into_inner())) }
fn set_filter(c: &[i32]) { *FILTER.lock().unwrap_or_else(|e| e.into_inner()) = c.to_vec(); }
fn set_deny(c: i32) { *DENY_CODE.lock().unwrap_or_else(|e| e.into_inner()) = c; }

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn exr(&mut self, label: &str, s: &str) { unsafe {
        take_log();
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let err = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={} LOG={}", self.cid, label, rc, err, take_log()));
    } }
    fn rows(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={} err={}", self.cid, label, rc, CStr::from_ptr(sqlite3_errmsg(self.db)).to_string_lossy())); return; }
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
        self.lines.push(format!("OBS {} {} rows={}", self.cid, label, buf));
    } }
    fn cols(&mut self, label: &str, sql: &str) { unsafe {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        let rc = sqlite3_prepare_v2(self.db, CString::new(sql).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        if rc != 0 { self.lines.push(format!("OBS {} {} prep.rc={}", self.cid, label, rc)); return; }
        let mut buf = String::new();
        for i in 0..sqlite3_column_count(st) {
            if i > 0 { buf.push(','); }
            buf.push_str(&CStr::from_ptr(sqlite3_column_name(st, i as c_int)).to_string_lossy());
        }
        sqlite3_finalize(st);
        self.lines.push(format!("OBS {} {} {}", self.cid, label, buf));
    } }
    fn rowsq(&mut self, label: &str, sql: &str) { self.rows(label, sql); }
    fn check_keep(&mut self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        // slice dir: engine-harvest43 or engine-temp43
        let slice = if feat.starts_with("engine-temp43") { "engine-temp43" } else { "engine-harvest43" };
        p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
}

// ================= engine-harvest43-001: JSON array paths =================

#[test] fn h001_json() {
    let mut h = H::new("engine-harvest43-001-C001");
    h.rows("set_idx", "SELECT json_set('{\"a\":[1,2,3]}', '$.a[1]', 99)");
    h.rows("set_oob", "SELECT json_set('{\"a\":[1,2]}', '$.a[5]', 9)");
    h.rows("set_top", "SELECT json_set('[10,20,30]', '$[2]', 5)");
    h.rows("ins_idx", "SELECT json_insert('{\"a\":[1,2,3]}', '$.a[1]', 99)");
    h.rows("ins_oob", "SELECT json_insert('{\"a\":[1,2]}', '$.a[5]', 9)");
    h.rows("ins_empty", "SELECT json_insert('[]', '$[0]', 7)");
    h.rows("repl_idx", "SELECT json_replace('{\"a\":[1,2,3]}', '$.a[1]', 99)");
    h.rows("repl_oob", "SELECT json_replace('{\"a\":[1,2]}', '$.a[5]', 9)");
    h.check_keep();
    h.cid = "engine-harvest43-001-C002";
    h.rows("rm_idx", "SELECT json_remove('{\"a\":[1,2,3]}', '$.a[1]')");
    h.rows("rm_top", "SELECT json_remove('[10,20,30]', '$[0]')");
    h.rows("rm_nested", "SELECT json_remove('{\"a\":[{\"b\":1},{\"b\":2}]}', '$.a[0].b')");
    h.rows("nested_set", "SELECT json_set('{\"a\":[{\"b\":1},{\"b\":2}]}', '$.a[1].b', 42)");
    h.rows("extract_idx", "SELECT json_extract('{\"a\":[5,6,7]}', '$.a[2]')");
    h.rows("hash_append", "SELECT json_set('{\"a\":[1,2]}', '$.a[#]', 9)");
    h.rows("hash_minus", "SELECT json_set('{\"a\":[1,2,3]}', '$.a[#-1]', 9)");
    h.check_keep();
    h.cid = "engine-harvest43-001-C003";
    h.rows("patch_arr", "SELECT json_patch('{\"a\":[1,2]}', '{\"a\":[9]}')");
    h.rows("null_json", "SELECT json_set(NULL, '$[0]', 1)");
    h.rows("chain", "SELECT json_remove(json_set('{\"a\":[1,2,3]}', '$.a[0]', 0), '$.a[2]')");
    h.check();
}

// ================= engine-harvest43-002: pragma projections =================

#[test] fn h002_pragma() {
    let mut h = H::new("engine-harvest43-002-C001");
    h.ex("CREATE TABLE t(a UNIQUE, b); CREATE INDEX tb ON t(b); CREATE INDEX tpart ON t(a) WHERE a > 0;");
    h.ex("CREATE TABLE p(id INTEGER PRIMARY KEY, q); CREATE TABLE ch(x REFERENCES p(id) ON DELETE CASCADE ON UPDATE SET NULL, y, FOREIGN KEY(y) REFERENCES t(a));");
    h.ex("CREATE TABLE ck(m, n, FOREIGN KEY(m, n) REFERENCES t(a, b));");
    h.cols("il_cols", "SELECT * FROM pragma_index_list('t')");
    h.rows("il_rows", "SELECT * FROM pragma_index_list('t')");
    h.rows("il_pragma", "PRAGMA index_list(t);");
    h.rows("il_none", "SELECT count(*) FROM pragma_index_list('p')");
    h.check_keep();
    h.cid = "engine-harvest43-002-C002";
    h.cols("fk_cols", "SELECT * FROM pragma_foreign_key_list('ch')");
    h.rows("fk_rows", "SELECT * FROM pragma_foreign_key_list('ch')");
    h.rows("fk_pragma", "PRAGMA foreign_key_list(ch);");
    h.rows("fk_comp", "SELECT * FROM pragma_foreign_key_list('ck')");
    h.rows("fk_none", "SELECT count(*) FROM pragma_foreign_key_list('t')");
    h.check();
}

// ================= engine-temp43-001: TEMP schema =================

#[test] fn t001_temp() {
    let mut h = H::new("engine-temp43-001-C001");
    h.ex("CREATE TABLE shared(v); INSERT INTO shared VALUES('main');");
    h.exr("ct_temp", "CREATE TEMP TABLE tt(x);");
    h.exr("ct_temporary", "CREATE TEMPORARY TABLE tt2(x);");
    h.exr("ins_temp", "INSERT INTO tt VALUES(7);");
    h.rows("sel_temp", "SELECT x FROM tt");
    h.rows("sel_qual", "SELECT x FROM temp.tt");
    h.rows("temp_master", "SELECT name, type FROM sqlite_temp_master ORDER BY name");
    h.rows("main_master", "SELECT name FROM sqlite_master ORDER BY name");
    h.check_keep();
    h.cid = "engine-temp43-001-C002";
    h.exr("shadow", "CREATE TEMP TABLE shared(v);");
    h.exr("ins_shadow", "INSERT INTO shared VALUES('temp');");
    h.rows("resolve_unqual", "SELECT v FROM shared");
    h.rows("resolve_main", "SELECT v FROM main.shared");
    h.rows("resolve_temp", "SELECT v FROM temp.shared");
    h.exr("drop_unqual", "DROP TABLE shared;");
    h.rows("after_drop_unqual", "SELECT v FROM shared");
    h.rows("after_drop_temp_master", "SELECT name FROM sqlite_temp_master ORDER BY name");
    h.exr("drop_qual_temp", "DROP TABLE temp.tt2;");
    h.rows("temp_master2", "SELECT name FROM sqlite_temp_master ORDER BY name");
    h.check_keep();
    h.cid = "engine-temp43-001-C003";
    h.ex("CREATE TEMP TABLE tlog(m);");
    h.exr("temp_trig", "CREATE TEMP TRIGGER ttr AFTER INSERT ON tt BEGIN INSERT INTO tlog VALUES('fired:' || new.x); END;");
    h.exr("trig_fire", "INSERT INTO tt VALUES(55);");
    h.rows("trig_log", "SELECT m FROM tlog");
    h.rows("temp_master3", "SELECT name, type FROM sqlite_temp_master ORDER BY name");
    h.check();
}

// ================= engine-temp43-002: auth TEMP outer codes =================

#[test] fn t002_auth() { let _g = lockg(); unsafe {
    let mut h = H::new("engine-temp43-002-C001");
    sqlite3_set_authorizer(h.db, Some(auth_cb), ptr::null_mut());
    set_filter(&[4, 13]);
    h.exr("a_ct_temp", "CREATE TEMP TABLE at(x);");
    h.exr("a_drop_temp", "DROP TABLE at;");
    set_filter(&[]);
    sqlite3_set_authorizer(h.db, None, ptr::null_mut());
    h.check_keep();
    h.cid = "engine-temp43-002-C002";
    h.ex("CREATE TEMP TABLE at2(x);");
    sqlite3_set_authorizer(h.db, Some(auth_cb), ptr::null_mut());
    set_filter(&[4, 13]); set_deny(4);
    h.exr("a_deny_ct", "CREATE TEMP TABLE at3(x);");
    set_deny(13);
    h.exr("a_deny_drop", "DROP TABLE at2;");
    set_deny(0); set_filter(&[]);
    sqlite3_set_authorizer(h.db, None, ptr::null_mut());
    h.rows("at2_survives", "SELECT count(*) FROM sqlite_temp_master WHERE name='at2'");
    h.check();
} }

// ================= anti-cheat =================

#[test] fn anti_cheat_h43_json_runtime() { unsafe {
    let idx = (std::process::id() % 3) as usize; // 0..2
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    let sql = CString::new(format!("SELECT json_set('{{\"a\":[10,20,30]}}', '$.a[{idx}]', 99)")).unwrap();
    sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    let got = CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy().into_owned();
    let mut arr = [10, 20, 30]; arr[idx] = 99;
    assert_eq!(got, format!("{{\"a\":[{},{},{}]}}", arr[0], arr[1], arr[2]), "runtime idx {idx}");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h43_pragma_runtime() { unsafe {
    let seed = std::process::id() % 100000;
    let tn = format!("rt{seed}");
    let ix = format!("ix{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, &format!("CREATE TABLE {tn}(a,b); CREATE INDEX {ix} ON {tn}(a);"));
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(format!("SELECT name FROM pragma_index_list('{tn}')")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    assert_eq!(CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy(), ix);
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h43_temp_isolation() { unsafe {
    let seed = std::process::id() % 100000;
    let name = format!("iso{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let exs = |db: *mut Sqlite3, s: &str| { sqlite3_exec(db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); };
    exs(db, &format!("CREATE TABLE {name}(v); INSERT INTO {name} VALUES('main');"));
    exs(db, &format!("CREATE TEMP TABLE {name}(v); INSERT INTO {name} VALUES('temp');"));
    let read = |db: *mut Sqlite3, q: &str| -> String {
        let mut st: *mut Sqlite3Stmt = ptr::null_mut();
        sqlite3_prepare_v2(db, CString::new(q).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
        sqlite3_step(st);
        let v = CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy().into_owned();
        sqlite3_finalize(st); v };
    assert_eq!(read(db, &format!("SELECT v FROM {name}")), "temp", "unqualified resolves temp");
    assert_eq!(read(db, &format!("SELECT v FROM main.{name}")), "main", "main.-qualified resolves main");
    sqlite3_close(db);
} }
