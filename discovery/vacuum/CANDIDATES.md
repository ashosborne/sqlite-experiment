# CANDIDATES — vacuum (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | VACUUM rebuild | `sqlite3Vacuum` `src/vacuum.c:105`, `sqlite3RunVacuum` `:143` | Full db rebuild via temp copy; page_size/encoding effects |
| 002 | VACUUM INTO | INTO handling `src/vacuum.c:147,245,250` | Writes rebuilt copy to a new file (read-only-safe backup path) |
