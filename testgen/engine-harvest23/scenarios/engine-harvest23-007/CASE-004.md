# engine-harvest23-007-C004 — run-11 oneshot characterization

Feature: `engine-harvest23-007` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: deferred FK still immediate outside a transaction (see harness).

## Observables

- `orphan` = `rc=19 err=FOREIGN KEY constraint failed`
- `cnt` = `0`
