# engine-vtab38-003 — deferred module destructor

Confidence: observed-in-code (composed slice, run 48).
_v2 destructor defers while a live instance holds the registration: replace 0, drop 1, close 2.
Frozen scope = run-48 pinned cases (pack v38).
