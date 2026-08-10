# CANDIDATES — qrf (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Result formatting API (spec-driven) | `sqlite3_format_query_result` decl `ext/qrf/qrf.h:68`, spec struct `:27`, impl `ext/qrf/qrf.c:3068` | Reusable formatter (modes/widths/escaping) consumed by the shell |
