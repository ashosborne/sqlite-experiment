# engine-udf-001-C009 — run-11 oneshot characterization

Feature: `engine-udf-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: create_function_v2 xDestroy on replace(1) then close(2) (see harness).

## Observables

- `after_replace` = `1`
- `after_close` = `2`
