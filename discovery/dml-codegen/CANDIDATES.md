# CANDIDATES — dml-codegen (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | DML statement compilation | `sqlite3Insert` `src/insert.c:900`, `sqlite3Update` `src/update.c:285`, `sqlite3DeleteFrom` `src/delete.c:288` | Three write paths incl. xfer optimization `src/insert.c:3079` |
| 002 | Constraint-check generation (NOT NULL/CHECK/UNIQUE + ON CONFLICT modes) | `sqlite3GenerateConstraintChecks` `src/insert.c:1901` | Conflict-resolution matrix (ROLLBACK/ABORT/FAIL/IGNORE/REPLACE) |
