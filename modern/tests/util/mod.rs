//! Shared helper for the generated run-11 compare tests: executes a frozen SQL
//! script through the Rust exec and re-derives the exact OBS lines, comparing
//! to the read-only golden.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

struct Cap { rows: Vec<Vec<Option<String>>> }

unsafe extern "C" fn capture_cb(arg: *mut c_void, argc: c_int, argv: *mut *mut c_char, _az: *mut *mut c_char) -> c_int {
    let cap = &mut *(arg as *mut Cap);
    let mut row = Vec::new();
    for i in 0..argc as usize {
        let p = *argv.add(i);
        row.push(if p.is_null() { None } else { Some(CStr::from_ptr(p).to_str().unwrap().to_string()) });
    }
    cap.rows.push(row);
    0
}

pub fn compare_script(slice: &str, feature: &str, cnum: &str, sql: &str) {
    let mut gp = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    gp.pop();
    gp.push(format!("tests/characterization/{slice}/cases/{feature}/{cnum}.approved.txt"));
    let golden = std::fs::read_to_string(&gp).unwrap_or_else(|e| panic!("golden {gp:?}: {e}"));
    let case_id = format!("{feature}-{cnum}");

    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        let name = CString::new(":memory:").unwrap();
        assert_eq!(sqlite3_open(name.as_ptr(), &mut db), 0);
        let csql = CString::new(sql).unwrap();
        let mut cap = Cap { rows: Vec::new() };
        let mut err: *mut c_char = ptr::null_mut();
        let rc = sqlite3_exec(db, csql.as_ptr(), Some(capture_cb), &mut cap as *mut Cap as *mut c_void, &mut err);
        let mut lines: Vec<String> = Vec::new();
        for (ri, row) in cap.rows.iter().enumerate() {
            for (ci, v) in row.iter().enumerate() {
                lines.push(format!("OBS {case_id} row{ri}.col{ci} {}", v.as_deref().unwrap_or("NULL")));
            }
        }
        lines.push(format!("OBS {case_id} exec.rc {rc}"));
        lines.push(format!("OBS {case_id} cb.rows {}", cap.rows.len()));
        if golden.contains("errmsg.nonempty") {
            let nonempty = !err.is_null() && *err != 0;
            lines.push(format!("OBS {case_id} errmsg.nonempty {}", nonempty as i32));
        }
        sqlite3_free(err as *mut c_void);
        sqlite3_close(db);
        let derived = lines.join("\n") + "\n";
        assert_eq!(derived, golden, "case {case_id}: Rust derivation != frozen golden");
    }
}
