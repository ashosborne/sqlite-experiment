# engine-vdbe49-002-C004 — run-11 oneshot characterization

Feature: `engine-vdbe49-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: stretch execution: rowid seek returns the single row; bound a=? with 3 returns three (see harness).

## Observables

- `run_rowid` = `two`
- `bound_eq_3` = `three`
