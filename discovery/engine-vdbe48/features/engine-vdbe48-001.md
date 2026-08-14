# engine-vdbe48-001 — EXPLAIN of a table scan is C's program (real root page)

Confidence: observed-in-code (composed slice, run 59).
EXPLAIN SELECT col(s) FROM t on a file-backed rowid table emits C's exact listing: Init, OpenRead (p2 = the root page C/the file assigned, p4 = column-count hint), Rewind, Column per selected column, ResultRow, Next (p5=1), Halt, Transaction (p3 = the schema cookie from the file header, p4=0, p5=1), Goto. Same listing on an empty table and after reopen.
Frozen scope = run-59 pinned cases (pack v48). One plain rowid table, no IPK, non-WAL, autocommit.
