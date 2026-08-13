# engine-harvest40-002 — vtab savepoint family

Confidence: observed-in-code (composed slice, run 50).
xBegin/xSync/xCommit/xRollback and xSavepoint/xRelease/xRollbackTo with C's txn-savepoint-excluded numbering; ROLLBACK TO changes vtab DML visibility.
Frozen scope = run-50 pinned cases (pack v40).
