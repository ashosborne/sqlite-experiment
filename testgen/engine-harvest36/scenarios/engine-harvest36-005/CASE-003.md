# engine-harvest36-005-C003 — run-11 oneshot characterization

Feature: `engine-harvest36-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: deferred FK: DROP parent OK in txn, COMMIT blocked 19, ROLLBACK restores (see harness).

## Observables

- `deferred_drop_ok` = `rc=0 err=-`
- `commit_blocked` = `rc=19 err=FOREIGN KEY constraint failed`
- `rollback_ok` = `rc=0 err=-`
- `parent_restored` = `1`
