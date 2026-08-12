# engine-udf-001-C006 — run-11 oneshot characterization

Feature: `engine-udf-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: result_error -> statement fails rc1 errmsg "boom" (see harness).

## Observables

- `rc` = `1`
- `errmsg` = `boom`
