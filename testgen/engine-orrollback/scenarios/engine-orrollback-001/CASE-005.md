# engine-orrollback-001-C005 — run-11 oneshot characterization

Feature: `engine-orrollback-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DELETE OR ROLLBACK is a SYNTAX ERROR in C (DELETE has no conflict clause) — txn untouched (see harness).

## Observables

- `begin.rc` = `0`
- `del.rc` = `1`
- `autocommit` = `0`
- `commit.rc` = `0`
- `p` = `2`
