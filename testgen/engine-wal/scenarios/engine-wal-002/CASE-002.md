# engine-wal-002-C002 — run-11 oneshot characterization

Feature: `engine-wal-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: checkpoint then more writes then reopen (see harness).

## Observables

- `cp1` = `busy=0 backfilled=1`
- `rows` = `2,3`
