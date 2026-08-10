# CANDIDATES — where-optimizer (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | WHERE-loop generation boundary | `sqlite3WhereBegin` `src/where.c:6826`, `sqlite3WhereEnd` `src/where.c:7529` | The planner's callable seam for all query shapes |
| 002 | Access-path enumeration + cost solver | `whereLoopAddBtree` `src/where.c:4003`, index variant `:3219`, `wherePathSolver` `:5834` | Plan selection — parity-visible via EXPLAIN QUERY PLAN |
