# engine-checkupd-002-C002 — run-11 oneshot characterization

Feature: `engine-checkupd-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: UPDATE OR ABORT in txn: statement fails, txn survives, COMMIT keeps prior work (see harness).

## Observables

- `begin.rc` = `0`
- `upd.rc` = `19`
- `autocommit` = `0`
- `commit.rc` = `0`
- `count` = `2`
- `a5` = `5`
