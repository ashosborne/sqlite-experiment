# engine-harvest40-004 — WITHOUT ROWID hook suppression + truncate fast-path

Confidence: observed-in-code (composed slice, run 50).
update_hook is suppressed on WITHOUT ROWID tables; DELETE-without-WHERE fires no per-row hooks but changes() counts the rows.
Frozen scope = run-50 pinned cases (pack v40).
