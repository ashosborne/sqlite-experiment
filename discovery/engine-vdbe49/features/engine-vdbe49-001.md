# engine-vdbe49-001 — EXPLAIN of WHERE compares is C's inverted jump-to-Next program

Confidence: observed-in-code (composed slice, run 60).
C inverts the WHERE test into a jump-to-Next compare: = emits Ne, <> emits Eq, > emits Le, < emits Ge, >= emits Lt, <= emits Gt — all with p4=BINARY-8 and p5=84, the literal loaded into r2 by an init-section Integer (or Variable for ?). WHERE rowid=N emits Integer+SeekRowid with no Rewind/Next loop. Column a→r1, result columns from r3.
Frozen scope = run-60 pinned cases (pack v49). Integer literals on one column of one plain rowid table; no AND/OR, no index.
