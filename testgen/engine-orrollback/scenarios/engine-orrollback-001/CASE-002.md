# engine-orrollback-001-C002 — run-11 oneshot characterization

Feature: `engine-orrollback-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: INSERT OR ABORT fails the statement only (txn survives; COMMIT keeps prior work) (see harness).

## Observables

- `begin.rc` = `0`
- `dup.rc` = `19`
- `autocommit` = `0`
- `commit.rc` = `0`
- `count` = `1`
