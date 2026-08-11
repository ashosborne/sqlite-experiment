# CANDIDATES — global-init-config (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Library init/shutdown lifecycle | `sqlite3_initialize` `src/main.c:360`, `sqlite3_shutdown` `src/main.c:389`; decl `src/sqlite.h.in:1689` | Process-wide once-init with re-entrancy rules |
| 002 | Global configuration (sqlite3_config op matrix) | `sqlite3_config` `src/main.c:443` (post-init legality mask `:453`); defaults struct `sqlite3Config` `src/global.c:238` | ~30 config ops, most illegal after initialize |
| 003 | Per-connection configuration (sqlite3_db_config) | `sqlite3_db_config` `src/main.c:967`; decl `src/sqlite.h.in:1748` | Per-db toggles (FK, trigger, defensive, lookaside...) |
