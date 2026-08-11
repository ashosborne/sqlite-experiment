# CANDIDATES — misc-vfstrace (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vfs-shims` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | vfstrace call-tracing shim | `ext/misc/vfstrace.c:1138`, `ext/misc/vfstrace.c:21` | Wraps a VFS and prints every method call with args — debugging aid; registered via C API |
