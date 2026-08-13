# engine-harvest42-005 — UNION-distinct recursion, LIMIT, subquery

Confidence: observed-in-code (composed slice, run 52).
recursive UNION dedupes and terminates a cyclic graph; an outer LIMIT stops an unbounded machine; ORDER BY sorts the output; CTEs work as subquery sources.
Frozen scope = run-52 pinned cases (pack v42).
