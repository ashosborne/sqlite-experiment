# CANDIDATES — intck (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Incremental integrity-check lifecycle | `sqlite3_intck_open` `ext/intck/sqlite3intck.c:801`, `_step` `:849`, `_message` `:907` | Piecewise integrity_check that doesn't lock the whole db |
