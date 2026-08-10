# CANDIDATES — rtree (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | rtree vtab module (1-5 dims, int/float variants) | `sqlite3RtreeInit` `ext/rtree/rtree.c:4325`, module `:3379`, loadable init `:4485` | Spatial range-query surface |
| 002 | Custom geometry/query callbacks (MATCH operators) | `sqlite3_rtree_geometry_callback` `ext/rtree/rtree.c:4433`, `sqlite3_rtree_query_callback` `:4457`, MATCH plumbing `:361,397` | Public C API for custom spatial predicates |
