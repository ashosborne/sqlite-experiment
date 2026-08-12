# engine-upsert-expr-003-C001 — run-11 oneshot characterization

Feature: `engine-upsert-expr-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: partial unique index + matching ON CONFLICT ... WHERE (see harness).

## Observables

- `match` = `rc=0 err=-`
- `after` = `p,1,9`
