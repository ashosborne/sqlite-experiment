# engine-collation-003 — collation_needed lazy factory

Confidence: observed-in-code (composed slice, run 30).
sqlite3_collation_needed installs a factory invoked on unknown-collation lookup;
registering inside the callback lets the statement succeed; declining leaves the
C error standing (factory still called, name argument pinned).
Frozen scope = run-30 pinned cases (pack v20). collation_needed16 not claimed.
