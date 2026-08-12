# engine-harvest36-005-C005 — run-11 oneshot characterization

Feature: `engine-harvest36-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: immediate FK blocks DROP parent even inside a txn (see harness).

## Observables

- `immediate_in_txn_blocked` = `rc=19 err=FOREIGN KEY constraint failed`
- `rb` = `rc=0 err=-`
