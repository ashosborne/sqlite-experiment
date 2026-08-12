# engine-checkupd-002-C003 — run-11 oneshot characterization

Feature: `engine-checkupd-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: UPDATE OR ROLLBACK: violation kills the whole txn (autocommit 1, COMMIT errors) (see harness).

## Observables

- `begin.rc` = `0`
- `upd.rc` = `19`
- `autocommit` = `1`
- `commit.rc` = `1`
- `count` = `1`
