# engine-upsert-expr-003 — partial UNIQUE index targets

Confidence: observed-in-code (composed slice, run 31).
ON CONFLICT(c) WHERE flag=1 matches a partial UNIQUE index with a structurally
equal predicate; absent or non-matching WHERE reproduces the mismatch error.
Frozen scope = run-31 pinned cases (pack v21).
