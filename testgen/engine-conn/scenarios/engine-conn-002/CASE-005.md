# engine-conn-002-C005 — run-11 oneshot characterization

Feature: `engine-conn-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: busy_timeout(0) clears: immediate BUSY (see harness).

## Observables

- `clear_rc` = `0`
- `blocked` = `rc=5 err=database is locked`
