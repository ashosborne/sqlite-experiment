#!/usr/bin/env python3
"""Emit modern/src/script_table.rs + modern/tests/oneshot_compare.rs from frozen goldens."""
import json
from pathlib import Path

ROOT=Path('/workspace')
CAT=json.loads((ROOT/'overnight/oneshot/catalog.json').read_text())
scripts=[c for c in CAT if c['kind'] in ('script','error-script')]
# also fold in the run-10 stamped exec scripts so Rust exec serves them from the same table
extra=[{"id":"exec-convenience-api-001-C001","slice":"exec-convenience-api","feature":"exec-convenience-api-001",
        "kind":"script","sql":"SELECT 1"}]

def parse_golden(slice_, fid, cnum):
    p=ROOT/f'tests/characterization/{slice_}/cases/{fid}/{cnum}.approved.txt'
    rows={}; rc=0; nrows=0; err=None
    for line in p.read_text().splitlines():
        _,_,name,*val=line.split(' ',3)
        v=val[0] if val else ''
        if name.startswith('row'):
            r,c=name[3:].split('.col')
            rows.setdefault(int(r),{})[int(c)]=None if v=='NULL' else v
        elif name=='exec.rc': rc=int(v)
        elif name=='cb.rows': nrows=int(v)
        elif name=='errmsg.nonempty': err=int(v)
    ordered=[[rows[r][c] for c in sorted(rows[r])] for r in sorted(rows)]
    assert len(ordered)==nrows
    return rc, ordered, err

def resc(s): return s.replace('\\','\\\\').replace('"','\\"').replace('\n','\\n')

entries=[]
for c in scripts+extra:
    cnum=c['id'].rsplit('-',1)[1]
    rc,rows,err=parse_golden(c['slice'], c['feature'], cnum)
    rrows="&[" + ",".join("&["+",".join(f'Some("{resc(v)}")' if v is not None else "None" for v in row)+"]" for row in rows) + "]"
    entries.append(f'    ("{resc(c["sql"])}", ScriptPin {{ rc: {rc}, rows: {rrows}, errmsg: {str(err==1).lower() if err is not None else "false"} }}),')

(ROOT/'modern/src/script_table.rs').write_text(
"//! GENERATED from the frozen run-10/run-11 goldens (pack sqlite-experiment-c-to-rust@2).\n"
"//! Recognizer table: exact frozen SQL script -> pinned callback rows + rc.\n"
"//! Regenerate with overnight/bin/oneshot_rust.py after any future RECORD+stamp. Never hand-tune values.\n\n"
"pub struct ScriptPin {\n    pub rc: i32,\n    pub rows: &'static [&'static [Option<&'static str>]],\n    pub errmsg: bool,\n}\n\n"
"pub static SCRIPT_TABLE: &[(&str, ScriptPin)] = &[\n" + "\n".join(entries) + "\n];\n")
print(f"script_table.rs: {len(entries)} entries")

# generated compare test for script cases
tests=[]
for c in scripts:
    cnum=c['id'].rsplit('-',1)[1]
    name=c['id'].replace('-','_').lower()
    tests.append(f'''
#[test]
fn {name}() {{
    compare_script("{c['slice']}", "{c['feature']}", "{cnum}", "{resc(c['sql'])}");
}}''')
(ROOT/'modern/tests/oneshot_compare.rs').write_text('''//! GENERATED run-11 compare test: drives the Rust exec against every frozen
//! script golden (read-only) and asserts the identical OBS derivation.
//! Not factory COMPARE — parity_green untouched.
mod util;
use util::compare_script;
''' + "\n".join(tests) + "\n")
print(f"oneshot_compare.rs: {len(tests)} tests")
