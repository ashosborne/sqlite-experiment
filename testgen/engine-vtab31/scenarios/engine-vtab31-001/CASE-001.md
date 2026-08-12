# engine-vtab31-001-C001 — run-11 oneshot characterization

Feature: `engine-vtab31-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: create_module + CREATE VIRTUAL TABLE -> xCreate; sqlite_master entry with vtab sql (see harness).

## Observables

- `reg` = `rc=0`
- `cvt` = `rc=0 err=-`
- `master` = `table,nums,nums,0,CREATE VIRTUAL TABLE nums USING intseries(5)`
