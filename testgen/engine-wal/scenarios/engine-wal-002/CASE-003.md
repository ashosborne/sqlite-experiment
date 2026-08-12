# engine-wal-002-C003 — run-11 oneshot characterization

Feature: `engine-wal-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: bare PRAGMA wal_checkpoint (see harness).

## Observables

- `bare` = `busy=0 backfilled=1`
- `rows` = `9`
