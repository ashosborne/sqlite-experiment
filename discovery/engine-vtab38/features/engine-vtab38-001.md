# engine-vtab38-001 — xConnect on file schema reload

Confidence: observed-in-code (composed slice, run 48).
durable CREATE VIRTUAL TABLE row (rootpage 0 + sql); unregistered reopen errors exactly; re-register -> SELECT runs xConnect (never xCreate); close disconnects, DROP destroys.
Frozen scope = run-48 pinned cases (pack v38).
