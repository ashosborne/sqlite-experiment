# engine-harvest41-005 — DROP_TABLE / vtab authorizer codes

Confidence: observed-in-code (composed slice, run 51).
code 11 outer for DROP TABLE; CREATE VIRTUAL TABLE fires 29 and a vtab drop fires 30 (s1 table, s2 module, s3 main); DENY rc 23 leaves the object.
Frozen scope = run-51 pinned cases (pack v41). No sqlite_master tails frozen.
