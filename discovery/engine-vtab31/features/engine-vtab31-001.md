# engine-vtab31-001 — module registration and vtab lifecycle

Confidence: observed-in-code (composed slice, run 41).
create_module(+v2 destructor on replace/close); CREATE VIRTUAL TABLE -> xCreate with C argv convention; unknown-module + constructor-failure errors; DROP -> xDestroy; fresh-connection re-registration.
Frozen scope = run-41 pinned cases (pack v31).
