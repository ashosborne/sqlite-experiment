# CANDIDATES — vfs-kv (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Refines run-1 vfs-os-abstraction-003 (row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | kvvfs VFS (db pages as key/value pairs) | `sqlite3KvvfsInit` `src/os_kv.c:1094`, vfs object `:114`, `kvvfsOpen` `:893` | Stores pages in a K/V store (browser localStorage/sessionStorage target) |
| 002 | Pluggable K/V method table | `sqlite3_kvvfs_methods` struct `src/os_kv.c:330`, instance `:361` | Replaceable storage callbacks — the actual persistence seam |
