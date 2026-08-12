# engine-prepare-001-C004 — run-11 oneshot characterization

Feature: `engine-prepare-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: prepare-time error: no such table (see harness).

## Observables

- `prepare.rc` = `1`
- `errcode` = `1`
- `errmsg` = `no such table: nosuch`
