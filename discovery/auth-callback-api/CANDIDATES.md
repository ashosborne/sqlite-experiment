# CANDIDATES — auth-callback-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Authorizer registration + dispatch | `src/auth.c:70` (`sqlite3_set_authorizer`), check dispatch `src/auth.c:238` (`sqlite3AuthCheck`) | Policy callback with OK/DENY/IGNORE contract |
| 002 | Column-read authorization (IGNORE→NULL) | `src/auth.c:104` (`sqlite3AuthReadCol`), `src/auth.c:136` (`sqlite3AuthRead`) | SQLITE_IGNORE substitutes NULL for column reads |
