# engine-prepare2-001-C002 — run-11 oneshot characterization

Feature: `engine-prepare2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: prepare_v3 SQLITE_PREPARE_PERSISTENT accepted, same results (see harness).

## Observables

- `prepare.rc` = `0`
- `step.rc` = `100`
- `a` = `8`
