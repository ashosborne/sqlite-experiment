# CANDIDATES — session (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Session lifecycle + change recording | `sqlite3session_create` `ext/session/sqlite3session.c:2362`, `attach` `:2473`, `changeset` `:3213` | Record-changes-on-connection seam |
| 002 | Changeset apply + conflict handling | `sqlite3changeset_apply` `ext/session/sqlite3session.c:5940` | Conflict-callback contract (OMIT/REPLACE/ABORT) |
| 003 | Changeset algebra (iterate/invert/concat) | `start` `:3407`, `invert` `:4308`, `concat` `:6814` | Changeset-as-data operations incl. patchset variant |
