# CANDIDATES — rbu (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | RBU update lifecycle (open/step/close, resumable state) | `sqlite3rbu_open` `ext/rbu/sqlite3rbu.c:4210`, `step` `:3713`, `close` `:4272` | Incremental bulk-apply seam with persistent resume state |
| 002 | RBU vacuum mode | `sqlite3rbu_vacuum` `ext/rbu/sqlite3rbu.c:4222` | Incremental VACUUM alternative with same step model |
