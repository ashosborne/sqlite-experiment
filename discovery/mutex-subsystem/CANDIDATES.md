# CANDIDATES — mutex-subsystem (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Pluggable mutex methods + static/dynamic mutex allocation | `sqlite3_mutex_alloc` `src/mutex.c:290`, init `:228`, method tables `src/mutex.c:193`, `src/mutex_unix.c:393` | Threading-mode foundation (single/multi/serialized) |
