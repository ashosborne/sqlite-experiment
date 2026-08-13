# engine-btree45-001 — btree handle txn_state on the pager

Confidence: observed-in-code (composed slice, run 56).
sqlite3_txn_state: idle 0, deferred BEGIN 0, read 1 after a SELECT, write 2 after a write / BEGIN IMMEDIATE|EXCLUSIVE, 0 on COMMIT/ROLLBACK.
Frozen scope = run-56 pinned cases (pack v45). File-backed, DELETE journal mode, single-leaf rowid table.
