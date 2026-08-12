# engine-attach30-001-C008 — run-11 oneshot characterization

Feature: `engine-attach30-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: UPDATE/DELETE on an attached table (see harness).

## Observables

- `upd` = `rc=0 err=-`
- `del` = `rc=0 err=-`
- `after` = `1,x|2,Q`
