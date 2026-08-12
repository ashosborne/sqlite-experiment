# engine-upsert-expr-001-C005 — run-11 oneshot characterization

Feature: `engine-upsert-expr-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: mismatched targets -> ON CONFLICT clause does not match error (see harness).

## Observables

- `wrong_expr` = `rc=1 err=ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint`
- `plain_col` = `rc=1 err=ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint`
