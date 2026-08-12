# engine-harvest36-002-C003 — run-11 oneshot characterization

Feature: `engine-harvest36-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: "order" table, qualified "select".x, sqlite_master names, DROP (see harness).

## Observables

- `from_order` = `7`
- `qualified` = `1`
- `master` = `order|select`
- `drop` = `rc=0 err=-`
- `master2` = `1`
