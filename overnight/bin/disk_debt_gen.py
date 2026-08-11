#!/usr/bin/env python3
"""Run-22 disk-debt batch: overflow pages + on-disk UNIQUE/secondary indexes.
ONE case list -> C harness (goldens) + Rust replay + catalog for freeze."""
import json
from pathlib import Path

LONG6 = ("abcdefghij" * 600)                        # 6000 chars -> 1 overflow page+
LONG15 = ("KLMNOPQRST" * 1500)                      # 15000 chars -> multiple overflow pages
LONG8M = ("qrstuvwxyz" * 798) + "OVF900217MARKER" + "zz"  # 8000-ish with frozen marker near end
HEX12 = "DEADBEEF" * 1500                           # 6000-byte blob as X'...'

CASES = []
def case(cid, ops): CASES.append({"id": cid, "ops": ops})

# ---------- Batch A: engine-overflow-001 ----------
case("engine-overflow-001-C001", [
    ("w","write.rc", f"CREATE TABLE ov(a INTEGER, t TEXT); INSERT INTO ov VALUES(1,'{LONG6}');"),
    ("r", "SELECT length(t), substr(t,1,10), substr(t,5991,10) FROM ov;")])
case("engine-overflow-001-C002", [
    ("w","write.rc", f"CREATE TABLE ov(a INTEGER, t TEXT); INSERT INTO ov VALUES(1,'{LONG6}');"),
    ("r", "PRAGMA integrity_check; SELECT count(*) FROM ov;")])
case("engine-overflow-001-C003", [
    ("w","write.rc", f"CREATE TABLE ovb(b BLOB); INSERT INTO ovb VALUES(X'{HEX12}');"),
    ("r", "SELECT length(b), substr(hex(b),1,8), substr(hex(b),11993,8) FROM ovb;")])
case("engine-overflow-001-C004", [
    ("w","write.rc", "CREATE TABLE mix(a INTEGER, t TEXT); " +
        " ".join(f"INSERT INTO mix VALUES({i},'v{i}');" for i in range(1,51)) +
        f" INSERT INTO mix VALUES(99,'{LONG6}');"),
    ("r", "SELECT count(*), max(length(t)) FROM mix;")])
case("engine-overflow-001-C005", [
    ("w","write.rc", "CREATE TABLE gr(t TEXT); INSERT INTO gr VALUES('short');"),
    ("w","grow.rc",  f"UPDATE gr SET t='{LONG6}';"),
    ("r", "SELECT length(t), substr(t,1,5) FROM gr;")])
case("engine-overflow-001-C006", [
    ("w","write.rc", f"CREATE TABLE sh(t TEXT); INSERT INTO sh VALUES('{LONG6}');"),
    ("w","shrink.rc", "UPDATE sh SET t='tiny';"),
    ("r", "SELECT length(t), t FROM sh; PRAGMA integrity_check;")])
case("engine-overflow-001-C007", [
    ("w","write.rc", f"CREATE TABLE m3(t TEXT); INSERT INTO m3 VALUES('{LONG15}');"),
    ("r", "SELECT length(t), substr(t,14991,10) FROM m3; PRAGMA integrity_check;")])
case("engine-overflow-001-C008", [
    ("w","write.rc", f"CREATE TABLE mk(t TEXT); INSERT INTO mk VALUES('{LONG8M}');"),
    ("r", "SELECT length(t), substr(t, length(t)-16, 15) FROM mk;")])

# ---------- Batch B: engine-indexes-001 ----------
case("engine-indexes-001-C001", [
    ("w","write.rc", "CREATE TABLE q(a INTEGER UNIQUE); INSERT INTO q VALUES(1),(2);"),
    ("w","dup.rc",   "INSERT INTO q VALUES(1);"),
    ("r", "SELECT count(*) FROM q;")])
case("engine-indexes-001-C002", [
    ("w","write.rc", "CREATE TABLE q(a INTEGER); CREATE UNIQUE INDEX qa ON q(a); INSERT INTO q VALUES(1),(2);"),
    ("w","dup.rc",   "INSERT INTO q VALUES(1);"),
    ("r", "SELECT count(*) FROM q;")])
case("engine-indexes-001-C003", [
    ("w","write.rc", "CREATE TABLE s(a INTEGER); CREATE INDEX sa ON s(a); INSERT INTO s VALUES(1),(2),(2);"),
    ("r", "SELECT count(*) FROM sqlite_master WHERE type='index'; SELECT count(*) FROM pragma_index_list('s'); SELECT count(*) FROM s; PRAGMA integrity_check;")])
case("engine-indexes-001-C004", [
    ("w","write.rc", "CREATE TABLE q(a INTEGER UNIQUE); INSERT INTO q VALUES(1);"),
    ("w","ins.rc",   "INSERT OR IGNORE INTO q VALUES(1),(5);"),
    ("r", "SELECT a FROM q ORDER BY a;")])
case("engine-indexes-001-C005", [
    ("w","write.rc", "CREATE TABLE k(id INTEGER PRIMARY KEY, v INTEGER); INSERT INTO k VALUES(1,10);"),
    ("w","upsert.rc","INSERT INTO k VALUES(1,99) ON CONFLICT(id) DO UPDATE SET v=excluded.v;"),
    ("r", "SELECT v FROM k;")])
case("engine-indexes-001-C006", [
    ("w","write.rc", "CREATE TABLE m(id INTEGER PRIMARY KEY, u TEXT UNIQUE); INSERT INTO m VALUES(1,'x'),(2,'y');"),
    ("w","dup.rc",   "INSERT INTO m VALUES(3,'x');"),
    ("r", "SELECT count(*) FROM m; PRAGMA integrity_check;")])
case("engine-indexes-001-C007", [
    ("w","write.rc", "CREATE TABLE d(a INTEGER); CREATE INDEX da ON d(a); DROP INDEX da; INSERT INTO d VALUES(7);"),
    ("r", "SELECT count(*) FROM sqlite_master WHERE type='index'; SELECT a FROM d;")])
case("engine-indexes-001-C008", [
    ("w","write.rc", "CREATE TABLE mc(a INTEGER, b INTEGER, UNIQUE(a,b)); INSERT INTO mc VALUES(1,1),(1,2);"),
    ("w","dup.rc",   "INSERT INTO mc VALUES(1,1);"),
    ("w","ok.rc",    "INSERT INTO mc VALUES(2,1);"),
    ("r", "SELECT count(*) FROM mc; PRAGMA integrity_check;")])
case("engine-indexes-001-C009", [
    ("w","write.rc", "CREATE TABLE nu(a INTEGER UNIQUE); INSERT INTO nu VALUES(NULL); INSERT INTO nu VALUES(NULL);"),
    ("r", "SELECT count(*) FROM nu; PRAGMA integrity_check;")])
case("engine-indexes-001-C010", [
    ("w","write.rc", "CREATE TABLE q(a INTEGER UNIQUE); INSERT INTO q VALUES(1);"),
    ("w","more.rc",  "INSERT INTO q VALUES(5);"),
    ("w","dup.rc",   "INSERT INTO q VALUES(5);"),
    ("r", "SELECT count(*) FROM q;")])
case("engine-indexes-001-C011", [
    ("w","write.rc", "CREATE TABLE tu(t TEXT UNIQUE); INSERT INTO tu VALUES('aa'),('bb');"),
    ("w","dup.rc",   "INSERT INTO tu VALUES('aa');"),
    ("r", "SELECT count(*) FROM tu; PRAGMA integrity_check;")])
case("engine-indexes-001-C012", [
    ("w","write.rc", "CREATE TABLE q(a INTEGER UNIQUE); INSERT INTO q VALUES(1);"),
    ("w","rep.rc",   "INSERT OR REPLACE INTO q VALUES(1);"),
    ("r", "SELECT count(*) FROM q;")])

def cesc(s): return s.replace('\\','\\\\').replace('"','\\"')

def emit_c():
    lines = ['#include <stdio.h>','#include <unistd.h>','#include "sqlite3.h"',
             'static const char*CID; static int rows;',
             'static void oi(const char*n,long long v){printf("OBS %s %s %lld\\n",CID,n,v);}',
             'static int cb(void*p,int ac,char**av,char**az){(void)p;(void)az;int i;for(i=0;i<ac;i++)printf("OBS %s row%d.col%d %s\\n",CID,rows,i,av[i]?av[i]:"NULL");rows++;return 0;}',
             'static int wr(const char*P,const char*s){sqlite3*db;int rc=sqlite3_open(P,&db);if(rc==0)rc=sqlite3_exec(db,s,0,0,0);sqlite3_close(db);return rc;}',
             'static void rd(const char*P,const char*s){sqlite3*db;rows=0;int o=sqlite3_open(P,&db);oi("reopen.rc",o);int rc=sqlite3_exec(db,s,cb,0,0);oi("read.rc",rc);oi("cb.rows",rows);sqlite3_close(db);}',
             'int main(void){const char*P="/tmp/ddb.db";']
    for c in CASES:
        lines.append(f'  CID="{c["id"]}"; unlink(P);')
        for op in c["ops"]:
            if op[0] == "w": lines.append(f'  oi("{op[1]}", wr(P,"{cesc(op[2])}"));')
            else: lines.append(f'  rd(P,"{cesc(op[1])}");')
        lines.append('  unlink(P);')
    lines.append('  return 0;}')
    Path('/tmp/disk_debt_gen.c').write_text("\n".join(lines))

def emit_rust():
    out = ['//! GENERATED run-22 disk-debt replay: overflow + on-disk index cases through',
           '//! the Rust file engine, asserted equal to the frozen C goldens.',
           'use sqlite3_rust_spine::*;',
           'use std::ffi::{CStr, CString};',
           'use std::os::raw::{c_char, c_int, c_void};',
           'use std::path::PathBuf;',
           'use std::ptr;','',
           'fn golden(slice:&str,feat:&str,cnum:&str)->String{let mut p=PathBuf::from(env!("CARGO_MANIFEST_DIR"));p.pop();p.push(format!("tests/characterization/{slice}/cases/{feat}/{cnum}.approved.txt"));std::fs::read_to_string(&p).unwrap()}',
           'struct Cap<\'a>{cid:&\'a str,rows:i32,lines:&\'a mut Vec<String>}',
           'unsafe extern "C" fn cb(a:*mut c_void,ac:c_int,av:*mut *mut c_char,_z:*mut *mut c_char)->c_int{let c=&mut *(a as *mut Cap);for i in 0..ac as usize{let p=*av.add(i);let v=if p.is_null(){"NULL".to_string()}else{CStr::from_ptr(p).to_str().unwrap().to_string()};c.lines.push(format!("OBS {} row{}.col{} {}",c.cid,c.rows,i,v));}c.rows+=1;0}',
           'unsafe fn wr(l:&mut Vec<String>,cid:&str,label:&str,path:&str,sql:&str){let mut db:*mut Sqlite3=ptr::null_mut();let p=CString::new(path).unwrap();let mut rc=sqlite3_open(p.as_ptr(),&mut db);if rc==0{let c=CString::new(sql).unwrap();rc=sqlite3_exec(db,c.as_ptr(),None,ptr::null_mut(),ptr::null_mut());}sqlite3_close(db);l.push(format!("OBS {cid} {label} {rc}"));}',
           'unsafe fn rd(l:&mut Vec<String>,cid:&str,path:&str,sql:&str){let mut db:*mut Sqlite3=ptr::null_mut();let p=CString::new(path).unwrap();let o=sqlite3_open(p.as_ptr(),&mut db);l.push(format!("OBS {cid} reopen.rc {o}"));let mut cap=Cap{cid,rows:0,lines:l};let c=CString::new(sql).unwrap();let rc=sqlite3_exec(db,c.as_ptr(),Some(cb),&mut cap as *mut Cap as *mut c_void,ptr::null_mut());let rows=cap.rows;l.push(format!("OBS {cid} read.rc {rc}"));l.push(format!("OBS {cid} cb.rows {rows}"));sqlite3_close(db);}','']
    for i, c in enumerate(CASES):
        cid = c["id"]; feat = "-".join(cid.split("-")[:3]); cnum = cid.split("-")[-1]
        slice_ = "-".join(cid.split("-")[:2])
        fn = cid.replace("-", "_")
        body = [f'#[test]', f'fn {fn}() {{',
                f'  let path=format!("/tmp/ddtest_{i}_{{}}.db",std::process::id()); let _=std::fs::remove_file(&path);',
                f'  let mut l=Vec::new(); unsafe {{']
        for op in c["ops"]:
            if op[0] == "w": body.append(f'    wr(&mut l,"{cid}","{op[1]}",&path,"{cesc(op[2])}");')
            else: body.append(f'    rd(&mut l,"{cid}",&path,"{cesc(op[1])}");')
        body += [f'  }}', f'  assert_eq!(l.join("\\n")+"\\n", golden("{slice_}","{feat}","{cnum}"));',
                 f'  let _=std::fs::remove_file(&path);', f'}}','']
        out += body
    Path('/workspace/modern/tests/disk_debt.rs').write_text("\n".join(out))

def emit_catalog():
    cat = []
    for c in CASES:
        cid = c["id"]; feat = "-".join(cid.split("-")[:3]); slice_ = "-".join(cid.split("-")[:2])
        script = " || ".join((f"[{o[1]}] {o[2][:120]}" if o[0]=="w" else f"[read] {o[1][:120]}") for o in c["ops"])
        cat.append({"id": cid, "slice": slice_, "feature": feat, "kind": "script", "sql": script})
    Path('/workspace/overnight/oneshot/catalog7.json').write_text(json.dumps(cat, indent=0))

if __name__ == "__main__":
    emit_c(); emit_rust(); emit_catalog()
    print(f"emitted C + Rust + catalog for {len(CASES)} disk-debt cases")
