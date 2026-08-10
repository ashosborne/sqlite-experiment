# CANDIDATES — recover (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Recovery to new db (init/step/run) | `sqlite3_recover_init` `ext/recover/sqlite3recover.c:2783`, `step` `:2877` | Salvage pipeline over corrupt files |
| 002 | Recovery as SQL script (init_sql callback) | `sqlite3_recover_init_sql` `ext/recover/sqlite3recover.c:2795` | Alternative output mode: SQL text stream |
