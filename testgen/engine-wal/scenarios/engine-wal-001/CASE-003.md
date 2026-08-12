# engine-wal-001-C003 — run-11 oneshot characterization

Feature: `engine-wal-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: clean close deletes -wal/-shm; mode persists; rows durable (see harness).

## Observables

- `wal_after_close` = `0`
- `shm_after_close` = `0`
- `mode` = `wal`
- `rows` = `7`
