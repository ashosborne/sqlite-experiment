# engine-harvest40-002-C003 — run-11 oneshot characterization

Feature: `engine-harvest40-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: autocommit vtab DML wraps in a statement txn (begin/ins/sync/commit); explicit ROLLBACK fires xRollback and the module restores its base (see harness).

## Observables

- `create` = `rc=0 err=- mlog=[sync][commit]`
- `auto_ins` = `rc=0 err=- mlog=[begin][ins:1][sync][commit]`
- `begin` = `rc=0 err=- mlog=`
- `ins` = `rc=0 err=- mlog=[begin][ins:2]`
- `rollback` = `rc=0 err=- mlog=[rollback]`
- `after_rollback` = `1`
