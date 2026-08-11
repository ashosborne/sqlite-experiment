# CANDIDATES — compile-options-omit-enable (Phase A, unbound; resume run 2)

All `candidate`. STOP for human bind. Note: full option list is emitted into the amalgamation by
tool scripts (out-of-scope path); this slice cites in-tree guards + diagnostics API only.

| # | Candidate | Confidence | Evidence | Why in slice |
| --- | --- | --- | --- | --- |
| 001 | Compile-option diagnostics API (C + SQL) | observed-in-code | `sqlite3_compileoption_used` `src/main.c:5220`, `_get` `src/main.c:5253`; SQL wrappers `sqlite_compileoption_used/get` `src/func.c:1051,1075` | Runtime-queryable record of the build matrix — the honest oracle for baseline pinning |
| 002 | OMIT-gate census (surface removal) | observed-in-code | 77 distinct `SQLITE_OMIT_*` references in `src/sqliteInt.h` alone (e.g. guards over auth, vtab, WAL, windowfunc) | Each OMIT gate deletes a surface run 1 carded as present-by-default |
| 003 | ENABLE-gate census (surface addition) | observed-in-code | 51 distinct `SQLITE_ENABLE_*` references in `src/sqliteInt.h` (STAT4, FTS5, RTREE, DESERIALIZE, DBSTAT_VTAB, LOCKING_STYLE...) | Enabled-only surfaces must be pinned to the migration baseline build |
