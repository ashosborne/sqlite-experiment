# engine-pager44-001 — DELETE rollback-journal lifecycle

Confidence: observed-in-code (composed slice, run 55).
<db>-journal present during an open write txn, gone after COMMIT and ROLLBACK; committed rows visible on reopen, rolled-back rows gone.
Frozen scope = run-55 pinned cases (pack v44). File-backed, DELETE journal mode.
