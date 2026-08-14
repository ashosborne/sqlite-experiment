# engine-vdbe50-002-C001 — run-11 oneshot characterization

Feature: `engine-vdbe50-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: stepping the INSERT returns DONE, last_insert_rowid=1, and the row is visible to the cell scan (see harness).

## Observables

- `ins_step_rc` = `101 last 1`
- `after_one` = `1|one`
