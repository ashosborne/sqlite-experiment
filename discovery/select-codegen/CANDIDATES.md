# CANDIDATES — select-codegen (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | SELECT orchestration | `sqlite3Select` `src/select.c:7642`, expander `sqlite3ExpandSubquery` `:5929` | Top-level query compilation |
| 002 | Compound selects (UNION/EXCEPT/INTERSECT) | `multiSelect` `src/select.c:2978`, values fast-path `:2905`, merge `:3443` | Set-operation semantics incl. ORDER BY/LIMIT placement |
| 003 | Subquery flattening | `flattenSubquery` `src/select.c:4334` | Rewrite with ~20 restriction rules — silent plan-shape behaviour |
