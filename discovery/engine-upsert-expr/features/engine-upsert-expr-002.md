# engine-upsert-expr-002 — prior conflict-target surface regression pins

Confidence: observed-in-code (composed slice, run 31).
Column UNIQUE, multi-column UNIQUE index, INTEGER PRIMARY KEY targets and the
DO UPDATE ... WHERE guard re-pinned after target resolution landed — the pre-v21
surface stays green under the new resolver.
Frozen scope = run-31 pinned cases (pack v21).
