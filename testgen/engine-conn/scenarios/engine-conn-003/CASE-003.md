# engine-conn-003-C003 — run-11 oneshot characterization

Feature: `engine-conn-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: commit_hook fires per committed txn (autocommit + explicit) (see harness).

## Observables

- `calls` = `3`
- `cnt` = `4`
