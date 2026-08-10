# CANDIDATES — error-status-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Error introspection family | `src/main.c:2851` (`errcode`), `:2866` (`extended_errcode`), `:2743` (`errmsg`), `:2896` (`errstr`), `:2792` (`error_offset`) | Post-failure contract every caller relies on |
| 002 | Runtime limits | `src/main.c:3049` (`sqlite3_limit`) + `src/sqliteLimit.h` defaults | Per-connection limit get/set with clamping |
| 003 | Status counters (global + per-db) | `src/status.c:134,159` (`status64/status`), `:426` (`db_status`) | Memory/lookaside/cache counters with reset semantics |
