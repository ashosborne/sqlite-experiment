# CANDIDATES — connection-lifecycle-api (Phase A, unbound)

All items status `candidate`; confidence `observed-in-code` unless noted. STOP: human bind required before Phase B.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Open database (3 variants + URI filename parsing) | `src/main.c:3742` (`sqlite3_open`), `src/main.c:3749` (`_v2`), `src/main.c:3762` (`_open16`), shared `openDatabase` `src/main.c:3380`, URI parser `sqlite3ParseUri` `src/main.c:3125`; decl `src/sqlite.h.in:4019,4027` | Constructor of the central `sqlite3` handle |
| 002 | Close database (deferred vs immediate) | `src/main.c:1381` (`sqlite3_close`), `src/main.c:1382` (`sqlite3_close_v2` zombie/deferred close) | Destructor semantics incl. unfinalized-stmt behaviour |
| 003 | Busy handler / timeout | `src/main.c:1816` (`sqlite3_busy_handler`), `src/main.c:1873` (`sqlite3_busy_timeout`) | Lock-contention callback contract on the handle |
| 004 | Connection hooks & tracing | `src/main.c:2369` (`commit_hook`), `src/main.c:2394` (`update_hook`), `src/main.c:2309` (`trace_v2`) | Observable per-connection callbacks |

Adjacent-not-included (recommend separate seeds): global `sqlite3_initialize`/`sqlite3_config` (`src/sqlite.h.in:1689,1729`); `sqlite3_errmsg` (`src/main.c:2743`) → error-status-api.
