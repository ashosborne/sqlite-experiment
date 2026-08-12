# engine-prepare-001-C008 — run-11 oneshot characterization

Feature: `engine-prepare-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: stmt_readonly + stmt_busy against real statements (see harness).

## Observables

- `ro.select` = `1`
- `busy.before` = `0`
- `busy.row` = `1`
- `busy.done` = `0`
- `ro.insert` = `0`
