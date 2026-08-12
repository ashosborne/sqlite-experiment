# engine-upsert-expr-001-C007 — run-11 oneshot characterization

Feature: `engine-upsert-expr-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: durable: expression unique index + upsert after reopen (see harness).

## Observables

- `upsert` = `rc=0 err=-`
- `after` = `Gamma,7`
