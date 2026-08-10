# CANDIDATES — exec-convenience-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | sqlite3_exec callback loop (multi-statement, abort-on-nonzero) | `src/legacy.c:30`; decl `src/sqlite.h.in:430` | Convenience boundary wrapping prepare/step/finalize |
| 002 | get_table / free_table result marshalling | `src/table.c:116` (`sqlite3_get_table`), `src/table.c:185` (`sqlite3_free_table`) | Legacy tabular result API with distinct memory contract |
