# engine-harvest23-007-C001 — run-11 oneshot characterization

Feature: `engine-harvest23-007` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DEFERRABLE INITIALLY DEFERRED: COMMIT fails, txn stays open, resolvable (see harness).

## Observables

- `begin` = `rc=0 err=-`
- `orphan` = `rc=0 err=-`
- `commit1` = `rc=19 err=FOREIGN KEY constraint failed`
- `still_in_txn` = `1`
- `parent` = `rc=0 err=-`
- `commit2` = `rc=0 err=-`
- `final` = `1,1`
