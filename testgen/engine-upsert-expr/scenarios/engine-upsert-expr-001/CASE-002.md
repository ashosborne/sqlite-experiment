# engine-upsert-expr-001-C002 — run-11 oneshot characterization

Feature: `engine-upsert-expr-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: expression target DO UPDATE SET n=excluded.n (see harness).

## Observables

- `upsert` = `rc=0 err=-`
- `after` = `Alpha,9`
