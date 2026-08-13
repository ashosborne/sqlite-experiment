//! Run-50 tier-B replay — mirrors /tmp/h40c.c.
//! pragma_index_xinfo TVF (deepens pragma-surface-002; the card stays partial).
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;

struct H { cid: &'static str, lines: Vec<String>, db: *mut Sqlite3 }
impl H {
    fn new(cid: &'static str) -> H { unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        sqlite3_open(c":memory:".as_ptr(), &mut db);
        H { cid, lines: Vec::new(), db } } }
    fn ex(&mut self, s: &str) { unsafe {
        sqlite3_exec(self.db, CString::new(s).unwrap().as_ptr(), None, ptr::null_mut(), ptr::null_mut()); } }
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
}

#[test] fn h008() {
    let mut h = H::new("engine-harvest40-008-C001");
    h.ex("CREATE TABLE t(a, b, c); CREATE INDEX idx_ab ON t(a, b DESC);");
    h.cols("xinfo_cols", "SELECT * FROM pragma_index_xinfo('idx_ab')");
    h.rows("xinfo_rows", "SELECT seqno, cid, name, \"desc\", coll, key FROM pragma_index_xinfo('idx_ab')");
    h.rows("xinfo_missing", "SELECT count(*) FROM pragma_index_xinfo('nosuch')");
    h.check_keep();
    h.cid = "engine-harvest40-008-C002";
    h.ex("CREATE TABLE u(x INTEGER PRIMARY KEY, y); CREATE UNIQUE INDEX uy ON u(y COLLATE NOCASE);");
    h.rows("xinfo_coll", "SELECT seqno, cid, name, \"desc\", coll, key FROM pragma_index_xinfo('uy')");
    h.rows("info_vs_xinfo", "SELECT (SELECT count(*) FROM pragma_index_info('uy')), (SELECT count(*) FROM pragma_index_xinfo('uy'))");
    h.check();
}
