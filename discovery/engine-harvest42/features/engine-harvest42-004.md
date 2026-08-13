# engine-harvest42-004 — SQLITE_RECURSIVE scan-gating

Confidence: observed-in-code (composed slice, run 52).
code 33 fires only for a used recursive member as [33|~|~|~|cte] among the s4-context consults; unused WITH RECURSIVE is silent; DENY rc 23.
Frozen scope = run-52 pinned cases (pack v42).
