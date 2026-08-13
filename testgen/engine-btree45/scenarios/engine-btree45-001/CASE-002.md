# engine-btree45-001-C002 — run-11 oneshot characterization

Feature: `engine-btree45-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: BEGIN IMMEDIATE and BEGIN EXCLUSIVE start a write txn (2) immediately; ROLLBACK/COMMIT return to 0 (see harness).

## Observables

- `after_begin_immediate` = `2`
- `after_rollback` = `0`
- `after_begin_exclusive` = `2`
- `after_commit2` = `0`
