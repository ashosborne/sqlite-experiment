# engine-vdbe50-002-C005 — run-11 oneshot characterization

Feature: `engine-vdbe50-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: VDBE-inserted rows are read back by the v48 scan, the v49 WHERE compare and the rowid seek (see harness).

## Observables

- `scan` = `1|one/2|two`
- `where_eq` = `two`
- `seek` = `one`
