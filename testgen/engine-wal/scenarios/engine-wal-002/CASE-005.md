# engine-wal-002-C005 — run-11 oneshot characterization

Feature: `engine-wal-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: wal_checkpoint(TRUNCATE) zeroes -wal (see harness).

## Observables

- `wal_before` = `1`
- `trunc` = `busy=0 backfilled=1`
- `wal_size_zero` = `1`
- `rows` = `12`
- `reopen` = `12`
