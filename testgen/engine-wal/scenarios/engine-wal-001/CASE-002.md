# engine-wal-001-C002 — run-11 oneshot characterization

Feature: `engine-wal-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: commit creates -wal (nonempty) and -shm while connection open (see harness).

## Observables

- `write` = `rc=0 err=-`
- `wal_exists` = `1`
- `wal_nonempty` = `1`
- `shm_exists` = `1`
- `read` = `7`
