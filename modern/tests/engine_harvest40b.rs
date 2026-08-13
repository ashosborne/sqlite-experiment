//! Run-50 wave-2 replay — mirrors /tmp/h40b.c.
//! json_each/json_tree full vtab columns with C's JSONB-offset ids; db_config
//! leftover toggles with real effects; live completion phases.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

unsafe extern "C" fn f_udf(c: *mut Sqlite3Context, _n: c_int, v: *mut *mut Sqlite3Value) {
    let x = sqlite3_value_int64(*v);
    sqlite3_result_int64(c, x * 2);
}

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
    fn exr(&mut self, label: &str, s: &str) { unsafe {
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), &mut em);
        let err = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        self.lines.push(format!("OBS {} {} rc={} err={}", self.cid, label, rc, err));
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
        self.lines.push(format!("OBS {} {} {}", self.cid, label, buf));
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
    fn oi(&mut self, label: &str, v: i64) { self.lines.push(format!("OBS {} {} {}", self.cid, label, v)); }
    fn check_keep(&mut self) {
        let feat: String = self.cid.rsplitn(2, '-').nth(1).unwrap().to_string();
        let cnum = self.cid.rsplit('-').next().unwrap();
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); p.pop();
        p.push(format!("tests/characterization/engine-harvest40/cases/{feat}/{cnum}.approved.txt"));
        assert_eq!(self.lines.join("\n") + "\n", std::fs::read_to_string(&p).unwrap(), "case {}", self.cid);
        self.lines.clear();
    }
    fn check(mut self) {
        self.check_keep();
        unsafe { if !self.db.is_null() { sqlite3_close(self.db); self.db = ptr::null_mut(); } }
    }
    fn dbc(&mut self, op: c_int, val: i64) { unsafe { sqlite3_db_config(self.db, op, val, 0, 0); } }
    fn dbc_query(&mut self, op: c_int) -> i64 { unsafe {
        let mut cur: c_int = -1;
        sqlite3_db_config(self.db, op, -1, &mut cur as *mut c_int as i64, 0);
        cur as i64
    } }
}

// ================= 005: json_each / json_tree full columns =================

#[test] fn h005() {
    let mut h = H::new("engine-harvest40-005-C001");
    h.cols("each_cols", "SELECT * FROM json_each('[1,2]')");
    h.rows("each_full", "SELECT key,value,type,atom,id,parent,fullkey,path FROM json_each('{\"a\":1,\"b\":[true,null]}')");
    h.rows("each_arr", "SELECT key,value,type,atom,id,fullkey FROM json_each('[7,\"x\",null]')");
    h.check_keep();
    h.cid = "engine-harvest40-005-C002";
    h.cols("tree_cols", "SELECT * FROM json_tree('[1]')");
    h.rows("tree_full", "SELECT key,value,type,atom,id,parent,fullkey,path FROM json_tree('{\"a\":[1,2],\"b\":{\"c\":3}}')");
    h.rows("tree_arr", "SELECT fullkey, type, atom FROM json_tree('[1,[2,3]]')");
    h.rows("tree_root", "SELECT key, type, fullkey, path FROM json_tree('7')");
    h.check_keep();
    h.cid = "engine-harvest40-005-C003";
    h.rows("each_second", "SELECT key, value FROM json_each('{\"a\":1}', '$')");
    h.rows("tree_second", "SELECT fullkey, value FROM json_tree('{\"a\":{\"b\":4}}', '$.a')");
    h.rows("each_scalar", "SELECT key, value, type, id FROM json_each('9')");
    h.rows("tree_count", "SELECT count(*) FROM json_tree('{\"a\":[1,2],\"b\":{\"c\":3}}')");
    h.rows("each_count", "SELECT count(*) FROM json_each('{\"a\":[1,2],\"b\":{\"c\":3}}')");
    h.check();
}

// ================= 006: db_config leftover toggles =================

#[test] fn h006_dqs_ddl() {
    let mut h = H::new("engine-harvest40-006-C001");
    h.exr("dqs_ddl_default", "CREATE TABLE c1(x CHECK(x <> \"nosuchcol\"));");
    h.dbc(1014, 1);
    h.exr("dqs_ddl_on", "CREATE TABLE c2(x CHECK(x <> \"nosuchcol\"));");
    h.dbc(1014, 0);
    h.exr("dqs_ddl_off", "CREATE TABLE c3(x CHECK(x <> \"nosuchcol\"));");
    h.check();
}

#[test] fn h006_writable_schema_defensive() {
    let mut h = H::new("engine-harvest40-006-C002");
    h.ex("CREATE TABLE t(a);");
    h.exr("ws_off", "UPDATE sqlite_master SET rootpage=rootpage WHERE name='t';");
    h.dbc(1011, 1);
    h.exr("ws_on", "UPDATE sqlite_master SET rootpage=rootpage WHERE name='t';");
    let v = h.dbc_query(1011); h.oi("ws_state", v);
    h.dbc(1011, 0);
    h.dbc(1010, 1);
    h.exr("def_pragma_ws", "PRAGMA writable_schema=ON;");
    let v = h.dbc_query(1011); h.oi("def_ws_state", v);
    h.exr("def_update", "UPDATE sqlite_master SET rootpage=rootpage WHERE name='t';");
    h.dbc(1010, 0);
    let v = h.dbc_query(1010); h.oi("def_state", v);
    h.exr("off_pragma_ws", "PRAGMA writable_schema=ON;");
    h.exr("off_update", "UPDATE sqlite_master SET rootpage=rootpage WHERE name='t';");
    h.check();
}

#[test] fn h006_legacy_alter() {
    let mut h = H::new("engine-harvest40-006-C003");
    h.ex("CREATE TABLE t(a); CREATE VIEW v AS SELECT a FROM t;");
    h.exr("alter_default", "ALTER TABLE t RENAME TO u;");
    h.rows("view_sql_default", "SELECT sql FROM sqlite_master WHERE name='v'");
    let db = h.db; h.db = ptr::null_mut();
    unsafe { sqlite3_close(db); }
    let mut h2 = H::new("engine-harvest40-006-C003");
    h2.lines = std::mem::take(&mut h.lines);
    h2.ex("CREATE TABLE t(a); CREATE VIEW v AS SELECT a FROM t;");
    h2.dbc(1012, 1);
    let v = h2.dbc_query(1012); h2.oi("legacy_state", v);
    h2.exr("alter_legacy", "ALTER TABLE t RENAME TO u;");
    h2.rows("view_sql_legacy", "SELECT sql FROM sqlite_master WHERE name='v'");
    h2.check();
}

#[test] fn h006_reset_database() {
    let mut h = H::new("engine-harvest40-006-C004");
    h.ex("CREATE TABLE t(a); INSERT INTO t VALUES(1); CREATE VIEW rv AS SELECT a FROM t;");
    let v = h.dbc_query(1009); h.oi("reset_state0", v);
    h.dbc(1009, 1);
    let v = h.dbc_query(1009); h.oi("reset_state1", v);
    h.exr("reset_vacuum", "VACUUM;");
    h.dbc(1009, 0);
    h.rows("after_reset", "SELECT count(*) FROM sqlite_master");
    h.exr("post_reset_use", "CREATE TABLE n(v); INSERT INTO n VALUES(3);");
    h.rows("post_reset_rows", "SELECT v FROM n");
    h.check();
}

#[test] fn h006_trusted_schema() { unsafe {
    let mut h = H::new("engine-harvest40-006-C005");
    sqlite3_create_function(h.db, c"dbl".as_ptr(), 1, 1 /* UTF8 */, ptr::null_mut(), Some(f_udf), None, None);
    sqlite3_create_function(h.db, c"inn".as_ptr(), 1, 1 | 0x20_0000 /* UTF8|INNOCUOUS */, ptr::null_mut(), Some(f_udf), None, None);
    h.ex("CREATE VIEW tv AS SELECT dbl(21) AS o; CREATE VIEW iv AS SELECT inn(4) AS o;");
    h.rows("trusted_default", "SELECT o FROM tv");
    h.dbc(1017, 0);
    h.rows("trusted_off", "SELECT o FROM tv");
    let v = h.dbc_query(1017); h.oi("ts_state", v);
    h.rows("direct_use", "SELECT dbl(5)");
    h.rows("innocuous_off", "SELECT o FROM iv");
    h.dbc(1017, 1);
    h.rows("trusted_back", "SELECT o FROM tv");
    h.check();
} }

#[test] fn h006_loadext_gates() { unsafe {
    let mut h = H::new("engine-harvest40-006-C006");
    {
        let mut em: *mut c_char = ptr::null_mut();
        let rc = sqlite3_load_extension(h.db, c"/nonexistent/ext.so".as_ptr(), ptr::null(), &mut em);
        let err = if em.is_null() { "-".to_string() } else { CStr::from_ptr(em).to_string_lossy().into_owned() };
        h.lines.push(format!("OBS {} capi_disabled rc={} err={}", h.cid, rc, err));
    }
    h.exr("sql_disabled", "SELECT load_extension('/nonexistent/ext.so');");
    h.dbc(1005, 1);
    let v = h.dbc_query(1005); h.oi("capi_state", v);
    h.exr("sql_capi_on", "SELECT load_extension('/nonexistent/ext.so');");
    h.check();
} }

// ================= 007: live completion phases =================

#[test] fn h007() {
    let mut h = H::new("engine-harvest40-007-C001");
    h.ex("CREATE TABLE widgets(wcol1, wcol2); CREATE TABLE gadgets(gcol); CREATE VIEW wview AS SELECT wcol1 FROM widgets;");
    h.rows("comp_sel", "SELECT candidate FROM completion('sel') ORDER BY candidate");
    h.rows("comp_wid", "SELECT candidate FROM completion('wid') ORDER BY candidate");
    h.rows("comp_wcol", "SELECT candidate FROM completion('wcol') ORDER BY candidate");
    h.rows("comp_mai", "SELECT candidate FROM completion('mai') ORDER BY candidate");
    h.rows("comp_wv", "SELECT candidate FROM completion('wv') ORDER BY candidate");
    h.check_keep();
    h.cid = "engine-harvest40-007-C002";
    h.rows("comp_count", "SELECT count(*) FROM completion('')");
    h.rows("comp_phases", "SELECT DISTINCT phase FROM completion('') ORDER BY phase");
    h.cols("comp_cols", "SELECT * FROM completion('x')");
    h.rows("comp_g", "SELECT candidate, phase FROM completion('g') ORDER BY candidate");
    h.check();
}

// ================= anti-cheat =================

#[test] fn anti_cheat_h40b_json_runtime() { unsafe {
    // a runtime-built document's json_tree ids must track its JSONB offsets
    let n = (std::process::id() % 4) as usize + 2;
    let doc = format!("[{}]", (0..n).map(|i| (i + 1).to_string()).collect::<Vec<_>>().join(","));
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(
        format!("SELECT id FROM json_each('{doc}')")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    // single-digit ints are 2 JSONB bytes each after the 1-byte array header
    let mut expect = 1i64;
    while sqlite3_step(st) == 100 {
        assert_eq!(sqlite3_column_int64(st, 0), expect, "runtime doc {doc}");
        expect += 2;
    }
    assert_eq!(expect, 1 + 2 * n as i64, "row count for runtime doc");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }

#[test] fn anti_cheat_h40b_completion_runtime() { unsafe {
    // a runtime-named table must complete under phase 8
    let seed = std::process::id() % 100000;
    let tname = format!("rttab{seed}");
    let mut db: *mut Sqlite3 = ptr::null_mut();
    sqlite3_open(c":memory:".as_ptr(), &mut db);
    sqlite3_exec(db, CString::new(format!("CREATE TABLE {tname}(x);")).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut());
    let mut st: *mut Sqlite3Stmt = ptr::null_mut();
    sqlite3_prepare_v2(db, CString::new(
        format!("SELECT candidate, phase FROM completion('rttab')")).unwrap().as_ptr(), -1, &mut st, ptr::null_mut());
    assert_eq!(sqlite3_step(st), 100);
    let cand = CStr::from_ptr(sqlite3_column_text(st, 0) as *const c_char).to_string_lossy().into_owned();
    assert_eq!(cand, tname);
    assert_eq!(sqlite3_column_int64(st, 1), 8, "tables complete in phase 8");
    sqlite3_finalize(st);
    sqlite3_close(db);
} }
