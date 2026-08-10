# CANDIDATES — fts3 (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | FTS3/4 vtab module (legacy generation) | `sqlite3Fts3Init` `ext/fts3/fts3.c:4119`; loadable init `ext/fts3/fts3.c:6211` | Legacy FTS surface still shipped; candidate for retire-vs-migrate decision |
