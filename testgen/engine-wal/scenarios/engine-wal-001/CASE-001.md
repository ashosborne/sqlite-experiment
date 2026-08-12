# engine-wal-001-C001 — run-11 oneshot characterization

Feature: `engine-wal-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: journal_mode=WAL returns wal; no -wal/-shm until first write (see harness).

## Observables

- `set` = `wal`
- `wal_after_pragma` = `0`
- `shm_after_pragma` = `0`
