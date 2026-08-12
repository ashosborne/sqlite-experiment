# engine-txn-001 — BEGIN/COMMIT/ROLLBACK atomicity

Confidence: observed-in-code (composed slice, run 25).
snapshot-model txns: happy paths, nested-begin + no-txn errors, DDL rollback, DEFERRED/IMMEDIATE accepted.
Frozen scope = the run-25 pinned cases; snapshot-model undo (pack v15) — not a pager journal, not WAL.
