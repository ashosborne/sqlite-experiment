# engine-wal-001-C005 — run-11 oneshot characterization

Feature: `engine-wal-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: :memory: journal_mode=WAL -> memory (WAL refused) (see harness).

## Observables

- `set` = `memory`
- `get` = `memory`
