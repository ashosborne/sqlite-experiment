# engine-vdbe47-002-C001 — run-11 oneshot characterization

Feature: `engine-vdbe47-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: executing those programs returns the constant rows (incl. zero rows for WHERE 0) (see harness).

## Observables

- `run_1` = `1`
- `run_42` = `42`
- `run_add` = `3`
- `run_text` = `hi`
- `run_two` = `1|2`
- `run_where_0` = ``
