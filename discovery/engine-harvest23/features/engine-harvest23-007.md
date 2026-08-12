# engine-harvest23-007 — deferred foreign keys

Confidence: observed-in-code (composed slice, run 33).
DEFERRABLE INITIALLY DEFERRED ok-until-COMMIT; failed COMMIT keeps txn open; defer_foreign_keys pragma resets at txn end; immediate outside txn.
Frozen scope = run-33 pinned cases (pack v23).
