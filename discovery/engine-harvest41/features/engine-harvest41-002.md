# engine-harvest41-002 — SQLITE_SAVEPOINT authorizer code

Confidence: observed-in-code (composed slice, run 51).
code 32 with s1 = BEGIN/RELEASE/ROLLBACK and s2 = the savepoint name; plain BEGIN/ROLLBACK stay code 22; DENY rc 23 fires before the exists check.
Frozen scope = run-51 pinned cases (pack v41). No sqlite_master tails frozen.
