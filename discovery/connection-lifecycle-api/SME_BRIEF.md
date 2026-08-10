# SME brief — connection-lifecycle-api (Phase A)

**Found:** 4 candidate behaviours on the `sqlite3` handle lifecycle seam (open/URI-parse, deferred close, busy handling, hooks/trace). All observed-in-code with citations in CANDIDATES.md.

**Ambiguous boundaries:**
- Global library init (`sqlite3_initialize`/`sqlite3_config`) is a distinct process-wide seam — recommend its own seed rather than folding in here.
- `sqlite3_errmsg`/error codes live in main.c too but belong to error-status-api.

**Recommended binds:** accept 001–003 (thin, characterizable via observable return codes/callbacks); 004 accept if tracing behaviour matters to the migration target.
**Recommended defers:** legacy `sqlite3_trace`/`sqlite3_profile` (deprecated pair) — prose recommendation only, nothing written as deferred.
**Open questions:** compile-time flag matrix (`SQLITE_OMIT_*`) changes the visible API surface; which build config is the migration baseline?

STOPPED for human bind. No Phase B performed.
