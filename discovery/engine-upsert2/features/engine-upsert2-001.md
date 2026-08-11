# engine-upsert2-001 — Upsert DO UPDATE WHERE

Confidence: observed-in-code (composed slice, run 21).
ON CONFLICT DO UPDATE SET c=excluded.c WHERE <expr> — conditional apply.
Frozen scope = the run-21 pinned cases; executed for real by store/eval (pack v11).
