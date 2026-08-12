# engine-harvest36-002-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: quoted reserved word as table name: CREATE/INSERT/SELECT on "select" (see harness).

## Observables

- `create` = `rc=0 err=-`
- `insert` = `rc=0 err=-`
- `select` = `1,a|2,b`
