# engine-subquery-002 — EXISTS / correlated subqueries

Confidence: observed-in-code. EXISTS(...) compiles to an existence probe; correlated
references (t1.x inside the subquery) resolve against the outer cursor
(src/expr.c sqlite3ExprCodeIN/CodeSubselect, src/resolve.c outer-context lookup).
Five pinned shapes incl. correlated scalar + LIMIT, correlated NOT EXISTS, and a fresh
literal (900017) that exists in no other golden.
