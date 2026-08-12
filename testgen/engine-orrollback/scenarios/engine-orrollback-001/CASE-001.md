# engine-orrollback-001-C001 — run-11 oneshot characterization

Feature: `engine-orrollback-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: INSERT OR ROLLBACK on duplicate kills the whole txn (autocommit back to 1; COMMIT errors; rows gone) (see harness).

## Observables

- `begin.rc` = `0`
- `dup.rc` = `19`
- `autocommit` = `1`
- `commit.rc` = `1`
- `count` = `0`
