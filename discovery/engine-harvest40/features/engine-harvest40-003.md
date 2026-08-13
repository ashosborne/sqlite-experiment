# engine-harvest40-003 — legacy sqlite3_trace / sqlite3_profile

Confidence: observed-in-code (composed slice, run 50).
the deprecated trace/profile hooks share the trace slot, fire per statement with parameter-expanded text, and trace_v2 replaces both.
Frozen scope = run-50 pinned cases (pack v40).
