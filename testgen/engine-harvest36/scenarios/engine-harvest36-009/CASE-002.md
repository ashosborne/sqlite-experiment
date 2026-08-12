# engine-harvest36-009-C002 — run-11 oneshot characterization

Feature: `engine-harvest36-009` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: no WHERE -> zero constraints offered, zero-arg xFilter, default scan (see harness).

## Observables

- `default_scan` = `1|2|3`
- `no_constraint_offered` = `0`
- `filter_argc` = `0`
