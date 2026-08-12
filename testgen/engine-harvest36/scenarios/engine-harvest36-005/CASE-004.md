# engine-harvest36-005-C004 — run-11 oneshot characterization

Feature: `engine-harvest36-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: dropping child then parent inside one txn commits clean (see harness).

## Observables

- `both_in_txn` = `rc=0 err=-`
- `all_gone` = `0`
