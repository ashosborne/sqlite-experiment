# engine-harvest23-003 — snprintf + sqlite3_str_append

Confidence: observed-in-code (composed slice, run 33).
%d/%s/%q/%Q, truncation with NUL at n-1, n<=0 no-op returning buf; raw n-limited str_append; length tracking.
Frozen scope = run-33 pinned cases (pack v23).
