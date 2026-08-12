# engine-harvest23-009-C002 — run-11 oneshot characterization

Feature: `engine-harvest23-009` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_complete: multi-statement trigger bodies (see harness).

## Observables

- `two_stmts_open` = `0`
- `two_stmts_done` = `1`
- `no_trigger` = `1`
