# engine-harvest40-008-C002 — run-11 oneshot characterization

Feature: `engine-harvest40-008` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: index_xinfo carries the column collation (NOCASE) and shows one more row than index_info (the rowid tail) (see harness).

## Observables

- `xinfo_coll` = `0,1,y,0,NOCASE,1|1,-1,~,0,BINARY,0`
- `info_vs_xinfo` = `1,2`
