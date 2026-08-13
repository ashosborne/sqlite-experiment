# engine-harvest40-008-C001 — run-11 oneshot characterization

Feature: `engine-harvest40-008` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: pragma_index_xinfo adds desc/coll/key columns and the trailing rowid entry (cid -1, key 0); DESC column pinned (see harness).

## Observables

- `xinfo_cols` = `seqno,cid,name,desc,coll,key`
- `xinfo_rows` = `0,0,a,0,BINARY,1|1,1,b,1,BINARY,1|2,-1,~,0,BINARY,0`
- `xinfo_missing` = `0`
