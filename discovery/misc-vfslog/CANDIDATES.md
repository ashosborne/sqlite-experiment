# CANDIDATES — misc-vfslog (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vfs-shims` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | vfslog binary operation logger | `ext/misc/vfslog.c:755`, `ext/misc/vfslog.c:40` | Logs VFS operations to a binary file for later analysis; registered via sqlite3_register_vfslog (SQLITE_EXTRA_INIT hook) |
