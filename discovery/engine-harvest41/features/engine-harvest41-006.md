# engine-harvest41-006 — view / trigger s4 contexts

Confidence: observed-in-code (composed slice, run 51).
view SELECT: base-table READs carry s4 = view (whole body), then view-column READs, then a nested SELECT consult; trigger bodies fire with s4 = trigger name.
Frozen scope = run-51 pinned cases (pack v41). No sqlite_master tails frozen.
