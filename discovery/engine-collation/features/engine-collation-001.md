# engine-collation-001 — create_collation[_v2] + registry-driven COMPARE/ORDER BY

Confidence: observed-in-code (composed slice, run 30).
Register named collating sequences with real xCompare callbacks; use from SQL in
equality / range / ORDER BY / WHERE+ORDER BY composition. Overwrite, delete-by-NULL,
v2 xDestroy (replace/close), pArg user data, eTextRep accept/reject matrix,
builtins unchanged, xCompare invocation proven by counter.
Frozen scope = run-30 pinned cases (pack v20).
