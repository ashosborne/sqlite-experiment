# engine-agg-having-001 — DISTINCT/FILTER aggregates + HAVING

Confidence: observed-in-code (composed slice, run 21).
count/sum/group_concat DISTINCT, agg FILTER (WHERE ...), HAVING over GROUP BY incl. aggregate conditions.
Frozen scope = the run-21 pinned cases; executed for real by store/eval (pack v11).
