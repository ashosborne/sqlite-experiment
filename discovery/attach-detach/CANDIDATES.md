# CANDIDATES — attach-detach (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | ATTACH DATABASE (internal function-call implementation) | codegen `sqlite3Attach` `src/attach.c:449`, runtime `attachFunc` `src/attach.c:74` | Adds db to connection; max-attached limit; name collision errors |
| 002 | DETACH DATABASE | `sqlite3Detach` `src/attach.c:429`, `detachFunc` `src/attach.c:293` | Removes db; busy/locked error paths |
| 003 | Cross-database name fixation | `sqlite3FixInit` `src/attach.c:533` | DDL in attached dbs must not reference other dbs |
