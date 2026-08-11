# engine-setops-001 — Compound SELECT set-operation scripts

Confidence: observed-in-code (composed slice, run 20).
INTERSECT / EXCEPT / mixed compounds with ORDER BY over real store tables and subqueries; left-associative fold with distinct semantics.
Frozen scope = the run-20 pinned cases; executed for real by store/eval/datetime (pack v10).
