#!/usr/bin/env python3
"""Run-16 file-path batch: build cases, emit a C harness + a matching Rust replay
from ONE case list, so C freezes the golden and Rust must reproduce it byte-for-byte.
Op kinds: ('w', label, sql) = open+exec+close capturing '<label> <rc>';
          ('r', sql)        = open(reopen.rc)+exec-with-cb(rows)+read.rc+cb.rows.
"""
import json, sys
from pathlib import Path

ROOT = Path('/workspace')

def bulk(vals):  # "(v1),(v2),..."
    return ",".join(f"({v})" for v in vals)

CASES = []

# ---------- Batch A (engine-files-002): size / pages ----------
big = [i * 3 for i in range(1, 1201)]  # 1200 rows -> multi-leaf
CASES.append({"id": "engine-files-002-C001", "ops": [
    ("w", "write.rc", f"CREATE TABLE big(a INTEGER); INSERT INTO big VALUES {bulk(big)};"),
    ("r", "SELECT count(*) FROM big;")]})
CASES.append({"id": "engine-files-002-C002", "ops": [
    ("w", "write.rc", f"CREATE TABLE big(a INTEGER); INSERT INTO big VALUES {bulk(big)};"),
    ("r", "SELECT a FROM big WHERE a=1503;")]})
txt = "abcdefghij" * 20  # 200 chars, well within a leaf (no overflow)
wide_rows = [f"({i},'{txt}')" for i in range(1, 41)]  # 40 * ~210 -> multi-leaf
CASES.append({"id": "engine-files-002-C003", "ops": [
    ("w", "write.rc", f"CREATE TABLE wide(id INTEGER, t TEXT); INSERT INTO wide VALUES {','.join(wide_rows)};"),
    ("r", "SELECT count(*) FROM wide;")]})
CASES.append({"id": "engine-files-002-C004", "ops": [
    ("w", "write.rc", f"CREATE TABLE wide(id INTEGER, t TEXT); INSERT INTO wide VALUES {','.join(wide_rows)};"),
    ("r", "SELECT t FROM wide WHERE id=20;")]})
mix40 = [f"({i},'v{i}')" for i in range(1, 41)]
CASES.append({"id": "engine-files-002-C005", "ops": [
    ("w", "write.rc", f"CREATE TABLE mix(a INTEGER, b TEXT); INSERT INTO mix VALUES {','.join(mix40)};"),
    ("r", "SELECT count(*) FROM mix;")]})
CASES.append({"id": "engine-files-002-C006", "ops": [  # fresh literal 888002
    ("w", "write.rc", "CREATE TABLE s(a INTEGER, b TEXT); INSERT INTO s VALUES (1,'p'),(888002,'q'),(3,'r');"),
    ("r", "SELECT a,b FROM s ORDER BY a;")]})

# ---------- Batch B (engine-files-003): schema objects on disk ----------
FK = "PRAGMA foreign_keys=ON; "
CASES += [
 {"id":"engine-files-003-C001","ops":[  # FK orphan insert after reopen -> 19
    ("w","write.rc", FK+"CREATE TABLE par(id INTEGER PRIMARY KEY); CREATE TABLE chi(pid REFERENCES par(id)); INSERT INTO par VALUES(5); INSERT INTO chi VALUES(5);"),
    ("w","orphan.rc", FK+"INSERT INTO chi VALUES(6);"),
    ("r","SELECT count(*) FROM chi;")]},
 {"id":"engine-files-003-C002","ops":[  # FK valid insert after reopen -> 0
    ("w","write.rc", FK+"CREATE TABLE par(id INTEGER PRIMARY KEY); CREATE TABLE chi(pid REFERENCES par(id)); INSERT INTO par VALUES(5),(6);"),
    ("w","valid.rc", FK+"INSERT INTO chi VALUES(6);"),
    ("r","SELECT pid FROM chi WHERE pid=6;")]},
 {"id":"engine-files-003-C003","ops":[  # FK CASCADE after reopen
    ("w","write.rc", FK+"CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE CASCADE); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1);"),
    ("w","delete.rc", FK+"DELETE FROM p2 WHERE id=1;"),
    ("r","SELECT count(*) FROM c2;")]},
 {"id":"engine-files-003-C004","ops":[  # trigger persists + fires after reopen
    ("w","write.rc", "CREATE TABLE tr(a INTEGER); CREATE TABLE tlog(v INTEGER); CREATE TRIGGER trg AFTER INSERT ON tr BEGIN INSERT INTO tlog VALUES(new.a); END;"),
    ("w","fire.rc", "INSERT INTO tr VALUES(7);"),
    ("r","SELECT v FROM tlog;")]},
 {"id":"engine-files-003-C005","ops":[  # sqlite_master lists trigger after reopen
    ("w","write.rc", "CREATE TABLE tr(a INTEGER); CREATE TABLE tlog(v INTEGER); CREATE TRIGGER trg AFTER INSERT ON tr BEGIN INSERT INTO tlog VALUES(new.a); END;"),
    ("r","SELECT count(*) FROM sqlite_master WHERE type='trigger';")]},
 {"id":"engine-files-003-C006","ops":[  # ALTER RENAME persisted
    ("w","write.rc", "CREATE TABLE t3(a INTEGER); INSERT INTO t3 VALUES(9);"),
    ("w","alter.rc", "ALTER TABLE t3 RENAME TO t3x;"),
    ("r","SELECT a FROM t3x;")]},
 {"id":"engine-files-003-C007","ops":[  # ALTER ADD COLUMN DEFAULT persisted
    ("w","write.rc", "CREATE TABLE t4(a INTEGER); INSERT INTO t4 VALUES(9);"),
    ("w","alter.rc", "ALTER TABLE t4 ADD COLUMN b DEFAULT 5;"),
    ("r","SELECT a,b FROM t4;")]},
 {"id":"engine-files-003-C008","ops":[  # PK upsert DO NOTHING durable
    ("w","write.rc", "CREATE TABLE up(a INTEGER PRIMARY KEY, b TEXT); INSERT INTO up VALUES(1,'x'); INSERT INTO up VALUES(1,'y') ON CONFLICT(a) DO NOTHING;"),
    ("r","SELECT b FROM up;")]},
 {"id":"engine-files-003-C009","ops":[  # PK upsert DO UPDATE durable
    ("w","write.rc", "CREATE TABLE up2(a INTEGER PRIMARY KEY, b TEXT); INSERT INTO up2 VALUES(1,'x'); INSERT INTO up2 VALUES(1,'y') ON CONFLICT(a) DO UPDATE SET b=excluded.b;"),
    ("r","SELECT b FROM up2;")]},
 {"id":"engine-files-003-C010","ops":[  # IPK value survives reopen
    ("w","write.rc", "CREATE TABLE k(id INTEGER PRIMARY KEY, v TEXT); INSERT INTO k VALUES(42,'hi');"),
    ("r","SELECT id,v FROM k WHERE id=42;")]},
 {"id":"engine-files-003-C011","ops":[  # multi-column after reopen
    ("w","write.rc", "CREATE TABLE m3(a INTEGER, b TEXT, c INTEGER); INSERT INTO m3 VALUES(2,'y',20),(1,'x',10);"),
    ("r","SELECT a,b,c FROM m3 ORDER BY a;")]},
 {"id":"engine-files-003-C012","ops":[  # two FK tables both readable after reopen
    ("w","write.rc", FK+"CREATE TABLE pp(id INTEGER PRIMARY KEY); CREATE TABLE cc(pid REFERENCES pp(id)); INSERT INTO pp VALUES(3); INSERT INTO cc VALUES(3);"),
    ("r","SELECT count(*) FROM pp;")]},
]

# ---------- Batch C (engine-files-004): catalogue file-twins ----------
CASES += [
 {"id":"engine-files-004-C001","ops":[("w","write.rc","CREATE TABLE ek(a INTEGER); INSERT INTO ek VALUES(7);"),("r","SELECT a FROM ek;")]},
 {"id":"engine-files-004-C002","ops":[("w","write.rc","CREATE TABLE ek2(a INTEGER, b TEXT); INSERT INTO ek2 VALUES(1,'x'); INSERT INTO ek2 VALUES(2,'y');"),("r","SELECT a,b FROM ek2 ORDER BY a;")]},
 {"id":"engine-files-004-C003","ops":[("w","write.rc","CREATE TABLE ek3(a INTEGER); INSERT INTO ek3 VALUES(10);"),("w","upd.rc","UPDATE ek3 SET a=11;"),("r","SELECT a FROM ek3;")]},
 {"id":"engine-files-004-C004","ops":[("w","write.rc","CREATE TABLE ek4(a INTEGER); INSERT INTO ek4 VALUES(1),(2); DELETE FROM ek4 WHERE a=1;"),("r","SELECT a FROM ek4;")]},
 {"id":"engine-files-004-C005","ops":[("w","write.rc","CREATE TABLE ek5(a INTEGER); INSERT INTO ek5 VALUES(424243);"),("r","SELECT a FROM ek5;")]},
 {"id":"engine-files-004-C006","ops":[("w","write.rc","CREATE TABLE n1(a INTEGER); INSERT INTO n1 VALUES(5);"),("r","SELECT n1.a, a, rowid FROM n1;")]},
 {"id":"engine-files-004-C007","ops":[("w","write.rc","CREATE TABLE t2(a INTEGER); CREATE UNIQUE INDEX i2 ON t2(a); INSERT INTO t2 VALUES(1); INSERT OR IGNORE INTO t2 VALUES(1);"),("r","SELECT count(*) FROM t2;")]},
 {"id":"engine-files-004-C008","ops":[("w","write.rc","CREATE TABLE u(a INTEGER UNIQUE); INSERT INTO u VALUES(1); INSERT OR REPLACE INTO u VALUES(1); INSERT OR IGNORE INTO u VALUES(1);"),("r","SELECT count(*) FROM u;")]},
 {"id":"engine-files-004-C009","ops":[("w","write.rc",FK+"CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE CASCADE); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1); DELETE FROM p2;"),("r","SELECT count(*) FROM c2;")]},
 {"id":"engine-files-004-C010","ops":[("w","write.rc","CREATE TABLE tr2(a INTEGER); CREATE TABLE tlog2(v INTEGER); CREATE TRIGGER trg2 AFTER INSERT ON tr2 BEGIN INSERT INTO tlog2 VALUES(new.a*2); END; INSERT INTO tr2 VALUES(7);"),("r","SELECT v FROM tlog2;")]},
]

def cesc(s): return s.replace('\\','\\\\').replace('"','\\"')

def emit_c():
    lines = ['#include <stdio.h>','#include <unistd.h>','#include "sqlite3.h"',
             'static const char*CID; static int rows;',
             'static void oi(const char*n,long long v){printf("OBS %s %s %lld\\n",CID,n,v);}',
             'static int cb(void*p,int ac,char**av,char**az){(void)p;(void)az;int i;for(i=0;i<ac;i++)printf("OBS %s row%d.col%d %s\\n",CID,rows,i,av[i]?av[i]:"NULL");rows++;return 0;}',
             'static int wr(const char*P,const char*s){sqlite3*db;int rc=sqlite3_open(P,&db);if(rc==0)rc=sqlite3_exec(db,s,0,0,0);sqlite3_close(db);return rc;}',
             'static void rd(const char*P,const char*s){sqlite3*db;rows=0;int o=sqlite3_open(P,&db);oi("reopen.rc",o);int rc=sqlite3_exec(db,s,cb,0,0);oi("read.rc",rc);oi("cb.rows",rows);sqlite3_close(db);}',
             'int main(void){const char*P="/tmp/efb.db";']
    for c in CASES:
        lines.append(f'  CID="{c["id"]}"; unlink(P);')
        for op in c["ops"]:
            if op[0] == "w":
                lines.append(f'  oi("{op[1]}", wr(P,"{cesc(op[2])}"));')
            else:
                lines.append(f'  rd(P,"{cesc(op[1])}");')
        lines.append('  unlink(P);')
    lines.append('  return 0;}')
    Path('/tmp/files_gen.c').write_text("\n".join(lines))

def emit_rust():
    out = ['//! GENERATED run-16 file-batch replay: each durable case reproduced through',
           '//! the Rust file engine and asserted equal to the frozen C golden.',
           'use sqlite3_rust_spine::*;',
           'use std::ffi::{CStr, CString};',
           'use std::os::raw::{c_char, c_int, c_void};',
           'use std::path::PathBuf;',
           'use std::ptr;','',
           'fn golden(feat:&str,cnum:&str)->String{let mut p=PathBuf::from(env!("CARGO_MANIFEST_DIR"));p.pop();p.push(format!("tests/characterization/engine-files/cases/{feat}/{cnum}.approved.txt"));std::fs::read_to_string(&p).unwrap()}',
           'struct Cap<\'a>{cid:&\'a str,rows:i32,lines:&\'a mut Vec<String>}',
           'unsafe extern "C" fn cb(a:*mut c_void,ac:c_int,av:*mut *mut c_char,_z:*mut *mut c_char)->c_int{let c=&mut *(a as *mut Cap);for i in 0..ac as usize{let p=*av.add(i);let v=if p.is_null(){"NULL".to_string()}else{CStr::from_ptr(p).to_str().unwrap().to_string()};c.lines.push(format!("OBS {} row{}.col{} {}",c.cid,c.rows,i,v));}c.rows+=1;0}',
           'unsafe fn wr(l:&mut Vec<String>,cid:&str,label:&str,path:&str,sql:&str){let mut db:*mut Sqlite3=ptr::null_mut();let p=CString::new(path).unwrap();let mut rc=sqlite3_open(p.as_ptr(),&mut db);if rc==0{let c=CString::new(sql).unwrap();rc=sqlite3_exec(db,c.as_ptr(),None,ptr::null_mut(),ptr::null_mut());}sqlite3_close(db);l.push(format!("OBS {cid} {label} {rc}"));}',
           'unsafe fn rd(l:&mut Vec<String>,cid:&str,path:&str,sql:&str){let mut db:*mut Sqlite3=ptr::null_mut();let p=CString::new(path).unwrap();let o=sqlite3_open(p.as_ptr(),&mut db);l.push(format!("OBS {cid} reopen.rc {o}"));let mut cap=Cap{cid,rows:0,lines:l};let c=CString::new(sql).unwrap();let rc=sqlite3_exec(db,c.as_ptr(),Some(cb),&mut cap as *mut Cap as *mut c_void,ptr::null_mut());let rows=cap.rows;l.push(format!("OBS {cid} read.rc {rc}"));l.push(format!("OBS {cid} cb.rows {rows}"));sqlite3_close(db);}','']
    for i, c in enumerate(CASES):
        cid = c["id"]; feat = "-".join(cid.split("-")[:3]); cnum = cid.split("-")[-1]
        fn = cid.replace("-", "_")
        body = [f'#[test]', f'fn {fn}() {{',
                f'  let path=format!("/tmp/eftest/fb_{i}_{{}}.db",std::process::id()); let _=std::fs::remove_file(&path);',
                f'  let mut l=Vec::new(); unsafe {{']
        for op in c["ops"]:
            if op[0] == "w":
                body.append(f'    wr(&mut l,"{cid}","{op[1]}",&path,"{cesc(op[2])}");')
            else:
                body.append(f'    rd(&mut l,"{cid}",&path,"{cesc(op[1])}");')
        body += [f'  }}', f'  assert_eq!(l.join("\\n")+"\\n", golden("{feat}","{cnum}"));',
                 f'  let _=std::fs::remove_file(&path);', f'}}','']
        out += body
    Path('/workspace/modern/tests/files_batch.rs').write_text("\n".join(out))

def emit_meta():
    meta = [{"id": c["id"], "feat": "-".join(c["id"].split("-")[:3]), "cnum": c["id"].split("-")[-1]} for c in CASES]
    Path('/tmp/files_meta.json').write_text(json.dumps(meta))

if __name__ == "__main__":
    emit_c(); emit_rust(); emit_meta()
    print(f"emitted C + Rust for {len(CASES)} file-path cases")
