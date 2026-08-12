# engine-orrollback-001-C006 — run-11 oneshot characterization

Feature: `engine-orrollback-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_get_autocommit across BEGIN/COMMIT (see harness).

## Observables

- `ac0` = `1`
- `ac1` = `0`
- `ac2` = `1`
