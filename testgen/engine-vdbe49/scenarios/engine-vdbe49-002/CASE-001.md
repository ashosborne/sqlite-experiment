# engine-vdbe49-002-C001 — run-11 oneshot characterization

Feature: `engine-vdbe49-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: executing the WHERE scan returns C rows in rowid order (both matching rows, one and uno) (see harness).

## Observables

- `run_eq` = `one/uno`
- `run_eq_ab` = `1|one/1|uno`
