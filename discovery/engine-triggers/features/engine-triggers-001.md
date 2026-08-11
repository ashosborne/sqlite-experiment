# engine-triggers-001 — Trigger matrix scripts

Confidence: observed-in-code (composed slice, run 20).
BEFORE/AFTER x INSERT/UPDATE/DELETE with old.*/new.* bindings and WHEN clauses; per-row firing order pinned (B then A).
Frozen scope = the run-20 pinned cases; executed for real by store/eval/datetime (pack v10).
