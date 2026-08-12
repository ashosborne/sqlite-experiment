# engine-wal-001-C004 — run-11 oneshot characterization

Feature: `engine-wal-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: switch WAL->delete removes -wal; mode persists as delete (see harness).

## Observables

- `set` = `delete`
- `wal_after_switch` = `0`
- `rows` = `7|8`
- `mode2` = `delete`
