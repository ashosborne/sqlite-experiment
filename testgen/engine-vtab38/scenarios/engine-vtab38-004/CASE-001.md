# engine-vtab38-004-C001 — run-11 oneshot characterization

Feature: `engine-vtab38-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: xUpdate INSERT: auto rowid (argv1 NULL) and explicit rowid; last_insert_rowid follows the module (see harness).

## Observables

- `ins_auto` = `rc=0 err=-`
- `last_rowid_auto` = `1`
- `ins_explicit` = `rc=0 err=-`
- `last_rowid_explicit` = `7`
- `scan` = `1,10,x|7,20,y`
