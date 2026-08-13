# engine-vtab38-002 — sqlite3_drop_modules

Confidence: observed-in-code (composed slice, run 48).
keep-list contract; dropped names stop resolving; live instances still scan but cannot be dropped without the module.
Frozen scope = run-48 pinned cases (pack v38).
