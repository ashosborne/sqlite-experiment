# engine-prepare2-001-C005 — run-11 oneshot characterization

Feature: `engine-prepare2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DROP TABLE under a live stmt: step -> SQLITE_ERROR no such table (see harness).

## Observables

- `step.rc` = `1`
- `errmsg` = `no such table: t`
