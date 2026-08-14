# engine-vdbe48-002-C001 — run-11 oneshot characterization

Feature: `engine-vdbe48-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: executing the scan returns rows in rowid order, both columns decoded from cells (see harness).

## Observables

- `run_a` = `10/20/30`
- `run_ab` = `10|ten/20|twenty/30|thirty`
- `run_b` = `ten/twenty/thirty`
