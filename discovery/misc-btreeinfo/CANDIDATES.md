# CANDIDATES — misc-btreeinfo (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vtab-packs` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | sqlite_btreeinfo introspection vtab | `ext/misc/btreeinfo.c:439` | Per-btree stats (depth, page counts, estimated entries) via sqlite_dbpage |
