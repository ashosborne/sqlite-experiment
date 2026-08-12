# engine-prepare2-001-C003 — run-11 oneshot characterization

Feature: `engine-prepare2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: prepare_v3 SQLITE_PREPARE_NO_VTAB on a plain table (see harness).

## Observables

- `prepare.rc` = `0`
- `step.rc` = `100`
- `a` = `9`
