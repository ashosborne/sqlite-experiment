# engine-vacuum-001-C004 — run-11 oneshot characterization

Feature: `engine-vacuum-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: VACUUM inside BEGIN -> C error; transaction continues (see harness).

## Observables

- `begin` = `rc=0 err=-`
- `vac` = `rc=1 err=cannot VACUUM from within a transaction`
- `in_txn` = `1`
- `ins` = `rc=0 err=-`
- `commit` = `rc=0 err=-`
- `cnt` = `2`
