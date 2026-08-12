# engine-attach30-001-C002 — run-11 oneshot characterization

Feature: `engine-attach30-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: CREATE/INSERT/SELECT on aux.t via qualified name (see harness).

## Observables

- `create` = `rc=0 err=-`
- `ins` = `rc=0 err=-`
- `qual` = `1,x|2,y`
