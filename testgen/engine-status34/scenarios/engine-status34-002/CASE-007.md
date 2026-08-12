# engine-status34-002-C007 — run-11 oneshot characterization

Feature: `engine-status34-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DEFERRED_FKS exact: 0 before, 1 inside deferred-violation txn, 0 after ROLLBACK (see harness).

## Observables

- `deferred_before` = `0`
- `def_ins` = `rc=0 err=-`
- `deferred_in_txn` = `1`
- `deferred_hi_zero` = `1`
- `deferred_after_rb` = `0`
