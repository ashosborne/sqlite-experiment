# engine-pager44-001-C001 — run-11 oneshot characterization

Feature: `engine-pager44-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DELETE-mode rollback journal: <db>-journal present during an open write txn, absent after COMMIT and after ROLLBACK; committed row visible, rolled-back row gone (see harness).

## Observables

- `journal_during_txn` = `1`
- `journal_after_commit` = `0`
- `after_commit_rows` = `one|two|three|four`
- `journal_during_txn2` = `1`
- `journal_after_rollback` = `0`
- `after_rollback_rows` = `one|two|three|four`
