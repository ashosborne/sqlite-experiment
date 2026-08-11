# CANDIDATES — misc-vfsstat (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vfs-shims` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | vfsstat I/O statistics shim + vtab | `ext/misc/vfsstat.c:806` | Counts read/write/sync per file class; exposes vfsstat vtab for SQL queries |
