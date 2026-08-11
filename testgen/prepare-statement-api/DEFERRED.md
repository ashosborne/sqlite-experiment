# Deferred — testgen/prepare-statement-api (run 4)

Operator-directed DEFERs (run-4 paste):
- Auto-reprepare / SQLITE_SCHEMA retry matrix (card behaviour, needs schema-change fixtures).
- BUSY-on-COMMIT recoverable-step path (card edge case marked "not independently verified" by the operator).
- UTF-16 prepare/step variants.
- All other prepare-statement-api feature IDs (001, 003-006) — explicitly out of this batch.

## Run-6 additions (operator DEFERs for the new IDs)
- 001: UTF-16 twins; v1 vs v2 SCHEMA-at-step; v3 prepFlags; schema-lock busy; zombie/closed-db MISUSE; nByte exact-length matrix.
- 003: bind-while-busy (MISUSE); TOOBIG; destructor STATIC/TRANSIENT; clear_bindings vs reset; zeroblob / bind_value.
- 005: auto-reprepare / SQLITE_SCHEMA retry; finalize during an open transaction; sqlite3_stmt_status counters.
- Still out of scope: 002-C003 replacement (sqlite3_step(NULL) would be a NEW case id C004 — not this run); 002/004/006; error-status-002/003.
