# engine-upsert-expr-002-C003 — run-11 oneshot characterization

Feature: `engine-upsert-expr-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: regression: INTEGER PRIMARY KEY target (see harness).

## Observables

- `upsert` = `rc=0 err=-`
- `after` = `1,20`
