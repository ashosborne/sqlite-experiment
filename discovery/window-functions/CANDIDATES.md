# CANDIDATES — window-functions (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Built-in window functions (row_number, rank, dense_rank, ntile, lead/lag, first/last/nth_value, percent_rank, cume_dist) | registry `sqlite3WindowFunctions` `src/window.c:610`, e.g. `row_numberStepFunc` `src/window.c:147` | SQL-visible function family |
| 002 | Frame-spec execution (ROWS/RANGE/GROUPS, exclude) | `sqlite3WindowCodeStep` `src/window.c:2784`, rewrite hook `sqlite3WindowUpdate` `src/window.c:659` | Frame semantics are the behaviour-dense core |
