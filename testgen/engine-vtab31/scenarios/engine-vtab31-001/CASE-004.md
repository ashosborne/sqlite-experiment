# engine-vtab31-001-C004 — run-11 oneshot characterization

Feature: `engine-vtab31-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DROP TABLE on vtab invokes xDestroy; table gone after (see harness).

## Observables

- `drop` = `rc=0 err=-`
- `destroyed` = `n=1`
- `gone` = `prep.rc=1 err=no such table: nums`
