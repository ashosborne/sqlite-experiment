# CANDIDATES — upsert (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Conflict-target resolution | `sqlite3UpsertAnalyzeTarget` `src/upsert.c:90`, `sqlite3UpsertOfIndex` `src/upsert.c:247` | Maps ON CONFLICT (cols) to a unique index; ambiguity errors |
| 002 | DO UPDATE execution | `sqlite3UpsertDoUpdate` `src/upsert.c:267`, ctor `src/upsert.c:55` | Update-on-conflict path incl. excluded.* references |
