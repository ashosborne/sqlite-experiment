# CANDIDATES — name-resolution (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Column/table name lookup rules | `lookupName` `src/resolve.c:278`, entry `sqlite3ResolveExprNames` `:2196`, select `:2295` | Scoping/ambiguity error behaviour |
| 002 | ORDER BY / GROUP BY resolution (aliases, ordinals) | `sqlite3ResolveOrderGroupBy` `src/resolve.c:1748` | Alias-vs-column precedence rules |
