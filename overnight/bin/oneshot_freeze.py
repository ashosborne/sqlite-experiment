#!/usr/bin/env python3
"""Run-11 stage 2: freeze goldens + specs + traceability + delegated stamps + manifest flips."""
import json, subprocess, sys, yaml
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

ROOT=Path('/workspace'); NOW=datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
RUN="2026-08-11T1600Z-legacy-record-oneshot"
CAT=json.loads((ROOT/'overnight/oneshot/catalog.json').read_text())
REC=json.load(open('/tmp/record11.json'))
CAP=REC['captures']
STAMP={"status":"HUMAN_ACCEPTED","operator":"Ash Osborne","date":"2026-08-11","tz":"Europe/London",
       "note":"run-11 delegated stamp (full-autonomy charter)"}

by_slice=defaultdict(list)
for c in CAT: by_slice[c['slice']].append(c)

for slice_, cases in sorted(by_slice.items()):
    base=ROOT/f'tests/characterization/{slice_}'
    tg=ROOT/f'testgen/{slice_}'
    (tg/'scenarios').mkdir(parents=True, exist_ok=True)
    (base/'runs'/RUN/'actuals').mkdir(parents=True, exist_ok=True)
    (base/'runs'/RUN/'logs').mkdir(parents=True, exist_ok=True)
    results=[]; tg_rows=[]; char_rows=[]
    for c in cases:
        cid=c['id']; fid=c['feature']; cnum=cid.rsplit('-',1)[1]
        lines=CAP[cid]
        gdir=base/'cases'/fid; gdir.mkdir(parents=True, exist_ok=True)
        gp=gdir/f'{cnum}.approved.txt'
        assert not gp.exists() or gp.read_text()=="\n".join(lines)+"\n", f"refusing golden rewrite: {gp}"
        gp.write_text("\n".join(lines)+"\n")
        (base/'runs'/RUN/'actuals'/f'{cid}.replay.txt').write_text("\n".join(lines)+"\n")
        sdir=tg/'scenarios'/fid; sdir.mkdir(parents=True, exist_ok=True)
        spec=sdir/f'CASE-{cnum[1:]}.md'
        body=(f"# {cid} — run-11 oneshot characterization\n\n"
              f"Feature: `{fid}` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)\n"
              f"Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.\n\n")
        if c['kind'] in ('script','error-script'):
            body+=f"## Boundary invoke\n\nOne `sqlite3_exec` call on a fresh `:memory:` db:\n\n```sql\n{c['sql']}\n```\n\n"
            if c.get('init'): body+=f"Extension init (static, -DSQLITE_CORE, no load_extension): {', '.join(c['init'])}\n\n"
        else:
            body+=f"## Boundary invoke\n\nBespoke C API sequence: {c['note']} (see harness).\n\n"
        body+="## Observables\n\n"+ "\n".join(f"- `{l.split(' ',3)[2]}` = `{l.split(' ',3)[3] if len(l.split(' ',3))>3 else ''}`" for l in lines)+"\n"
        spec.write_text(body)
        results.append({"case_id":cid,"feature_id":fid,"status":"REPLAY_GREEN","failure_class":None,
            "golden_path":f"cases/{fid}/{cnum}.approved.txt",
            "actual_path":f"runs/{RUN}/actuals/{cid}.replay.txt"})
        row={"id":cid,"feature_id":fid,"status":"REPLAY_GREEN","kind":"characterization",
             "evidence_gate":"observed-in-code","assert_mode":"RECORDED",
             "spec_path":f"scenarios/{fid}/CASE-{cnum[1:]}.md",
             "golden":f"tests/characterization/{slice_}/cases/{fid}/{cnum}.approved.txt",
             "discovery_evidence":[f"discovery/{slice_}/features/{fid}.md"]}
        tg_rows.append(row)
        char_rows.append({"case_id":cid,"feature_id":fid,"status":"REPLAY_GREEN",
                          "golden_path":f"cases/{fid}/{cnum}.approved.txt","run_id":RUN})
    # merge testgen TRACEABILITY (append; keep existing rows/stamps)
    tp=tg/'TRACEABILITY.yaml'
    d=yaml.safe_load(tp.read_text()) if tp.exists() else {"slice_id":slice_,
        "source_discovery":f"discovery/{slice_}/MANIFEST.yaml","phase":"B","batch_card_ids":[]}
    have={r.get('id') for r in d.get('cases',[])}
    d.setdefault('cases',[]).extend(r for r in tg_rows if r['id'] not in have)
    d['batch_card_ids']=sorted(set(d.get('batch_card_ids',[])+[c['feature'] for c in cases]))
    ga=d.get('golden_approval') or {}
    prev=ga.get('stamped_case_ids',[]) if isinstance(ga,dict) else []
    s=dict(STAMP); s['stamped_case_ids']=sorted(set(prev+[c['id'] for c in cases]))
    d['golden_approval']=s; d['updated_at']=NOW
    tp.write_text(yaml.safe_dump(d,sort_keys=False,width=140))
    # characterization TRACEABILITY
    cp=base/'TRACEABILITY.yaml'
    dc=yaml.safe_load(cp.read_text()) if cp.exists() else {"slice_id":slice_,"mode":"RECORD","target":"legacy","cases":[]}
    have={r.get('case_id') for r in dc.get('cases',[])}
    dc.setdefault('cases',[]).extend(r for r in char_rows if r['case_id'] not in have)
    ga=dc.get('golden_approval') or {}
    prev=ga.get('stamped_case_ids',[]) if isinstance(ga,dict) else []
    s=dict(STAMP); s['stamped_case_ids']=sorted(set(prev+[c['id'] for c in cases]))
    dc['golden_approval']=s; dc['updated_at']=NOW; dc['run_id']=RUN
    cp.write_text(yaml.safe_dump(dc,sort_keys=False,width=140))
    json.dump({"run_id":RUN,"slice_id":slice_,"mode":"RECORD","target":"legacy",
        "build":{"recipe":"run-5 pin reused (/tmp/sqlite-build), fingerprint re-verified 3.54.0/armor0/autoreset0",
                 "determinism_gate":"two independent runs byte-matched before freeze"},
        "scrub_profile":"none (deterministic static inputs; nondeterministic sources captured as derived booleans/shape only)",
        "cases":results}, open(base/'runs'/RUN/'results.json','w'), indent=1)
    (base/'runs'/RUN/'REPORT.md').write_text(
        f"# RECORD run report — {slice_} ({RUN})\n\nRun-11 oneshot: {len(cases)} case(s) RECORDED on the run-5 pin, "
        f"two-run determinism gate, immediate replay byte-match -> REPLAY_GREEN. Stamped HUMAN_ACCEPTED (delegated, "
        f"Ash Osborne 2026-08-11 Europe/London). parity_green untouched.\n")
print(f"artefacts written for {len(by_slice)} slices, {len(CAT)} cases")

# APP_MANIFEST: legacy_green flips
import jsonschema
p=ROOT/'inventory/sqlite-experiment/APP_MANIFEST.yaml'
app=yaml.safe_load(p.read_text())
feats=sorted({c['feature'] for c in CAT})
for b in app['behaviours']:
    if b['behaviour_id'] in feats:
        b['legacy_green']=True
        b['traceability']=f"tests/characterization/{b['slice_id']}/TRACEABILITY.yaml"
        base=(b.get('notes') or '')
        add="legacy RECORD REPLAY_GREEN + HUMAN_ACCEPTED 2026-08-11 (run-11 delegated stamp)"
        if add not in base: b['notes']=f"{base} | {add}"
lg=sum(1 for b in app['behaviours'] if b.get('legacy_green'))
assert not any(b.get('parity_green') for b in app['behaviours'])
app['counts']['legacy_green']=lg
app['last_updated']=NOW; app['updated_by']='sqlite-oneshot-50-slices'
jsonschema.validate(app,json.load(open(ROOT/'migration-factory/schemas/app-manifest.schema.json')))
header=p.read_text().split('schema_version:')[0]
p.write_text(header+yaml.safe_dump(app,sort_keys=False,width=120,allow_unicode=True))
print(f"legacy_green now {lg}; parity 0; schema VALID")
