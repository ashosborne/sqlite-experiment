# engine-conn-003-C004 — run-11 oneshot characterization

Feature: `engine-conn-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: commit_hook non-zero aborts COMMIT (rc 19 constraint failed) and rolls back (see harness).

## Observables

- `begin` = `rc=0 err=-`
- `ins` = `rc=0 err=-`
- `commit` = `rc=19 err=constraint failed`
- `autocommit` = `1`
- `cnt` = `1`
