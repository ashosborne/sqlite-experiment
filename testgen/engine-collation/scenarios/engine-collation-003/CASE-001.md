# engine-collation-003-C001 — run-11 oneshot characterization

Feature: `engine-collation-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: collation_needed factory registers lazily (see harness).

## Observables

- `eq` = `1`
- `factory_called` = `1`
- `factory_name` = `lazy1`
