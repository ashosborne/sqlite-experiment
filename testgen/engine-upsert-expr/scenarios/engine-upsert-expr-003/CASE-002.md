# engine-upsert-expr-003-C002 — run-11 oneshot characterization

Feature: `engine-upsert-expr-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: absent / wrong target WHERE -> mismatch error (see harness).

## Observables

- `no_where` = `rc=1 err=ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint`
- `wrong_where` = `rc=1 err=ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint`
