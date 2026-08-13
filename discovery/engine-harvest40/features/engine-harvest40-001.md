# engine-harvest40-001 — vtab ALTER RENAME (xRename)

Confidence: observed-in-code (composed slice, run 50).
ALTER TABLE RENAME of a vtab fires xRename and rewrites the quoted schema sql; a module without xRename still renames.
Frozen scope = run-50 pinned cases (pack v40).
