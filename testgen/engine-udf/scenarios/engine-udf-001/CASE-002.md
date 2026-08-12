# engine-udf-001-C002 — run-11 oneshot characterization

Feature: `engine-udf-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: wrong arity -> prepare error "wrong number of arguments" (see harness).

## Observables

- `prep.rc` = `1`
- `errmsg` = `wrong number of arguments to function twice()`
