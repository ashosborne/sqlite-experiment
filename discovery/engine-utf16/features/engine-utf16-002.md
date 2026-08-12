# engine-utf16-002 — UTF-16 column / metadata accessors

Confidence: observed-in-code (composed slice, run 29).
sqlite3_column_text16 / column_bytes16 / column_name16 / column_decltype16 (+ UTF-8
column_decltype landing with declared-type tracking): coercion matrix, bytes16 = 2x code
units, empty-vs-NULL, before-step/after-done/out-of-range tolerance, UTF-8-stored
non-ASCII text read as UTF-16.
Frozen scope = run-29 pinned cases (pack v19).
