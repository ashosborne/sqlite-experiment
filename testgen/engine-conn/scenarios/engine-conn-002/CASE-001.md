# engine-conn-002-C001 — run-11 oneshot characterization

Feature: `engine-conn-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: second connection write blocked while a txn holds the file lock; commit unblocks (see harness).

## Observables

- `blocked` = `rc=5 err=database is locked`
- `after_commit` = `rc=0 err=-`
- `cnt` = `2`
