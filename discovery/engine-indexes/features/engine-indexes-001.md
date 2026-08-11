# engine-indexes-001 — On-disk UNIQUE / secondary index b-trees

Confidence: observed-in-code (composed slice, run 22).
Column autoindexes, UNIQUE(a,b) sets and explicit indexes persist as 0x0a index b-trees; reopen enforces duplicates as C does.
Frozen scope = the run-22 pinned cases; executed for real by store/dbfile (pack v12).
