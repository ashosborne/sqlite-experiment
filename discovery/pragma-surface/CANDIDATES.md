# CANDIDATES — pragma-surface (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | PRAGMA dispatcher + name lookup | `sqlite3Pragma` `src/pragma.c:425`, binary-search `pragmaLocate` `src/pragma.c:311` | Single choke point for the whole PRAGMA language surface |
| 002 | pragma_* eponymous virtual tables | `pragmaVtabConnect` `src/pragma.c:2814`, module `src/pragma.c:3059`, register `src/pragma.c:3092` | PRAGMAs with results are queryable as table-valued functions |
