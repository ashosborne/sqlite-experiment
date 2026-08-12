# engine-pragma34-001-C002 — run-11 oneshot characterization

Feature: `engine-pragma34-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: data_version: own write does not bump, sibling commit does (see harness).

## Observables

- `own_write_same` = `1`
- `sibling_bumps` = `1`
