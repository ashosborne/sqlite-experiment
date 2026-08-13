# engine-btree45-001-C001 — run-11 oneshot characterization

Feature: `engine-btree45-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_txn_state: idle 0; a deferred BEGIN alone stays 0; a SELECT lifts to read 1; a write lifts to write 2; COMMIT returns to 0 (see harness).

## Observables

- `idle` = `0`
- `after_deferred_begin` = `0`
- `sel` = `0`
- `after_read` = `1`
- `after_write` = `2`
- `after_commit` = `0`
