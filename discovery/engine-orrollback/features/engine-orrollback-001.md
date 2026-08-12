# engine-orrollback-001 — OR ROLLBACK conflict semantics

Confidence: observed-in-code (composed slice, run 25).
INSERT OR ROLLBACK kills the txn; OR ABORT keeps it; DELETE OR ROLLBACK is a C syntax error (pinned); get_autocommit.
Frozen scope = the run-25 pinned cases; snapshot-model undo (pack v15) — not a pager journal, not WAL.
