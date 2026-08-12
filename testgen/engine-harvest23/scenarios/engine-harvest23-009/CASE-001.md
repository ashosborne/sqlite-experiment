# engine-harvest23-009-C001 — run-11 oneshot characterization

Feature: `engine-harvest23-009` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_complete: CASE..END nesting inside trigger body (see harness).

## Observables

- `open_case` = `0`
- `closed` = `1`
- `plain_body` = `1`
