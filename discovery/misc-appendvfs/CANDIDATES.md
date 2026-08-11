# CANDIDATES — misc-appendvfs (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vfs-shims` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | appendvfs (db appended to host file) | `ext/misc/appendvfs.c:651` | Opens a db image appended to another file (e.g. self-contained executable) with marker + offset handling |
