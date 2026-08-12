# engine-harvest36-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest36-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: UPDATE/DELETE on "select" (see harness).

## Observables

- `update` = `rc=0 err=-`
- `after_upd` = `z`
- `delete` = `rc=0 err=-`
- `count` = `1`
