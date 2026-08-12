# engine-txnfile-001 — Durable txn reopen

Confidence: observed-in-code (composed slice, run 25).
file reopen after COMMIT sees rows; after ROLLBACK or uncommitted close it does not.
Frozen scope = the run-25 pinned cases; snapshot-model undo (pack v15) — not a pager journal, not WAL.
