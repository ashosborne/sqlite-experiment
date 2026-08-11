#!/usr/bin/env python3
"""Run-11 oneshot pipeline: emit script harness C from catalog, RECORD (2 runs,
determinism gate), freeze goldens, write per-slice artefacts + delegated stamps."""
import json, subprocess, sys
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path('/workspace')
BUILD = Path('/tmp/sqlite-build')
CAT = json.loads((ROOT/'overnight/oneshot/catalog.json').read_text())
INITS = {"uuid":"sqlite3_uuid_init","regexp":"sqlite3_regexp_init","series":"sqlite3_series_init",
 "csv":"sqlite3_csv_init","decimal":"sqlite3_decimal_init","base64":"sqlite3_base64_init",
 "rot13":"sqlite3_rot_init","totype":"sqlite3_totype_init","uint":"sqlite3_uint_init",
 "ieee754":"sqlite3_ieee_init","completion":"sqlite3_completion_init","prefixes":"sqlite3_prefixes_init",
 "wholenumber":"sqlite3_wholenumber_init"}

def cesc(s):
    return s.replace('\\','\\\\').replace('"','\\"').replace('\n','\\n')

def emit_c():
    scripts=[c for c in CAT if c['kind'] in ('script','error-script')]
    exts=sorted({e for c in scripts for e in c.get('init',[])})
    decls="\n".join(f"extern int {INITS[e]}(sqlite3*, char**, const void*);" for e in exts)
    rows=[]
    for c in scripts:
        mask=",".join(f'"{e}"' for e in c.get('init',[])) or "0"
        rows.append(f'  {{"{c["id"]}", "{cesc(c["sql"])}", {1 if c["kind"]=="error-script" else 0}, {{{mask}, 0}}}},')
    table="\n".join(rows)
    src=f'''/* GENERATED run-11 script harness (capture-only, full case ids). */
#include <stdio.h>
#include <string.h>
#include "sqlite3.h"
{decls}
typedef struct {{ const char *id; const char *sql; int isErr; const char *init[4]; }} Case;
static Case aCase[] = {{
{table}
}};
typedef struct {{ const Case *c; int rows; }} CbCtx;
static int cb(void *pArg, int argc, char **argv, char **azCol){{
  CbCtx *p=(CbCtx*)pArg; int i; (void)azCol;
  for(i=0;i<argc;i++) printf("OBS %s row%d.col%d %s\\n", p->c->id, p->rows, i, argv[i]?argv[i]:"NULL");
  p->rows++;
  return 0;
}}
static void run_init(sqlite3 *db, const char *name){{
'''
    for e in exts:
        src+=f'  if(strcmp(name,"{e}")==0) {INITS[e]}(db, 0, 0);\n'
    src+='''}
int main(void){
  unsigned i, j;
  for(i=0;i<sizeof(aCase)/sizeof(aCase[0]);i++){
    const Case *c=&aCase[i];
    sqlite3 *db=0; char *zErr=0; CbCtx ctx={c,0}; int rc;
    sqlite3_open(":memory:", &db);
    for(j=0;c->init[j];j++) run_init(db, c->init[j]);
    rc = sqlite3_exec(db, c->sql, cb, &ctx, &zErr);
    printf("OBS %s exec.rc %d\\n", c->id, rc);
    printf("OBS %s cb.rows %d\\n", c->id, ctx.rows);
    if(c->isErr) printf("OBS %s errmsg.nonempty %d\\n", c->id, zErr && zErr[0]);
    sqlite3_free(zErr);
    sqlite3_close(db);
  }
  return 0;
}
'''
    (Path('/tmp/script_harness.c')).write_text(src)
    ext_srcs=" ".join(f"/workspace/ext/misc/{e}.c" for e in exts)
    cmd=(f"gcc -DSQLITE_CORE -I{BUILD} -I/workspace/src /tmp/script_harness.c {ext_srcs} "
         f"{BUILD}/sqlite3.c -lpthread -ldl -lm -o {BUILD}/harness/script11 -w")
    r=subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if r.returncode: sys.exit(f"compile failed:\n{r.stderr[-3000:]}")
    print(f"script harness compiled ({len(scripts)} cases, exts: {exts})")

def record():
    out1=subprocess.run([f"{BUILD}/harness/script11"],capture_output=True,text=True).stdout
    out2=subprocess.run([f"{BUILD}/harness/script11"],capture_output=True,text=True).stdout
    b1=subprocess.run([f"{BUILD}/harness/bespoke11"],capture_output=True,text=True).stdout
    b2=subprocess.run([f"{BUILD}/harness/bespoke11"],capture_output=True,text=True).stdout
    def split(txt):
        m=defaultdict(list)
        for l in txt.splitlines():
            if l.startswith("OBS "): m[l.split(' ',2)[1]].append(l)
        return m
    r1,r2={**split(out1),**split(b1)},{**split(out2),**split(b2)}
    green,flaky=[],[]
    for c in CAT:
        cid=c['id']
        if cid not in r1: sys.exit(f"no capture for {cid}")
        if r1[cid]==r2[cid]: green.append(cid)
        else: flaky.append(cid)
    json.dump({"green":green,"flaky":flaky,
               "captures":{cid:r1[cid] for cid in green}},
              open('/tmp/record11.json','w'))
    print(f"RECORD: {len(green)} deterministic (2-run byte-match), {len(flaky)} flaky -> BLOCKED: {flaky}")

if __name__=="__main__":
    emit_c(); record()
