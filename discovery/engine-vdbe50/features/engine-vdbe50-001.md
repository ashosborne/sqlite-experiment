# engine-vdbe50-001 — EXPLAIN of single-row INSERT is C's OpenWrite program

Confidence: observed-in-code (composed slice, run 61).
EXPLAIN INSERT INTO t[(a,b)] VALUES(...) emits C's exact listing: Init, OpenWrite (p2 = real root page, p4 = column count), value loads (Integer/String8, or Variable for ?), NewRowid→r1, MakeRecord (p4 = the affinity string, DB for INTEGER+TEXT), Insert (p4 = table name, p5=57), Halt, Transaction (p2=1 — a WRITE transaction, p3 = schema cookie), Goto. Omitting the column list emits the identical program.
Frozen scope = run-61 pinned cases (pack v50). One plain rowid table, no constraints, no IPK, non-WAL, autocommit.
