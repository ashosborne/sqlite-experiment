#!/usr/bin/env python3
"""Emit modern/tests/oneshot_compare.rs: per-case compare_script tests for catalog
cases that the store+eval executor now runs for real. EXCLUDE = re-homed (tested in
kitchen_compare) + deferred pins (no store coverage claimed)."""
import json, os
from pathlib import Path
ROOT=Path('/workspace')
CATS=os.environ.get('CATALOGS','overnight/oneshot/catalog.json,overnight/oneshot/catalog2.json').split(',')
CAT=[x for p in CATS for x in json.loads((ROOT/p).read_text())]
EXCLUDE=set(os.environ.get('EXCLUDE_IDS','').split(',')) - {''}
def resc(s): return s.replace('\\','\\\\').replace('"','\\"')
out=['//! GENERATED (pack v8): store+eval executor replays — no script_table.',
     'mod util;','use util::compare_script;','']
n=0
for c in CAT:
    cid=c['id']
    if cid in EXCLUDE or c['kind']=='bespoke': continue
    feat='-'.join(cid.split('-')[:-1]); cnum=cid.split('-')[-1]
    fn=cid.replace('-','_')
    out += [f'#[test]', f'fn {fn}() {{',
            f'    compare_script("{c["slice"]}", "{feat}", "{cnum}", "{resc(c["sql"])}");',
            f'}}','']
    n+=1
Path(os.environ.get('OUT','modern/tests/oneshot_compare.rs')).write_text("\n".join(out))
print(f"emitted {n} store/eval replay tests (excluded {len(EXCLUDE)})")
