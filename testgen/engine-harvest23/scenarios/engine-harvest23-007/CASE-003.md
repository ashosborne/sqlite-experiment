# engine-harvest23-007-C003 — run-11 oneshot characterization

Feature: `engine-harvest23-007` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: PRAGMA defer_foreign_keys defers an immediate FK; resets at COMMIT (see harness).

## Observables

- `orphan` = `rc=0 err=-`
- `parent` = `rc=0 err=-`
- `commit` = `rc=0 err=-`
- `pragma_after` = `0`
- `final` = `1`
