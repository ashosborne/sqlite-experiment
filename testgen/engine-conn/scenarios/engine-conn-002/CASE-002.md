# engine-conn-002-C002 — run-11 oneshot characterization

Feature: `engine-conn-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: busy handler returning 0: called once with count 0; BUSY propagates (see harness).

## Observables

- `blocked` = `rc=5 err=database is locked`
- `calls` = `1`
- `first_count` = `0`
