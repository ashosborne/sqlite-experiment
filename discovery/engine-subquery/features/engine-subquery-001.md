# engine-subquery-001 — scalar subqueries (SELECT list, WHERE, LIMIT inside)

Confidence: observed-in-code. `(SELECT ...)` in expression positions compiles via
`sqlite3CodeSubselect` (src/expr.c) and the SELECT core (src/select.c); LIMIT inside a
subquery is honored by the subquery's own OP_Limit. Five pinned shapes: SELECT-list
scalar, WHERE scalar, LIMIT/LIMIT-OFFSET inside, pragma_database_list twin (reclaims the
v8 defer shape of pragma-surface-002-C001), subquery-in-FROM + ORDER BY + LIMIT.
