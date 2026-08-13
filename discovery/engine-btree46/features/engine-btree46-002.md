# engine-btree46-002 — pinned C reads the modern split file

Confidence: observed-in-code (composed slice, run 57).
the pinned C amalgamation, compiled and exec'd, opens the MODERN-written split file: all rowids, the leaf probe, PRAGMA integrity_check ok, root type 5.
Frozen scope = run-57 pinned cases (pack v46). File-backed, DELETE journal mode.
