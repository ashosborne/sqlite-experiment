# engine-vdbe49-002 — the compare dispatches as an opcode over btree cells

Confidence: observed-in-code (composed slice, run 60).
Stepping the WHERE SQL runs the compare as an opcode in the dispatch loop: the cursor still positions on every cell (rejected rows too — cursor-read counter ≥ table row count), matching rows come back in rowid order, a no-match WHERE is DONE on the first step, the whole family (<>, >, <, >=, <=) executes, the rowid seek touches a single cell, a bound ? flows through Variable, and a later INSERT becomes visible.
Frozen scope = run-60 pinned cases (pack v49).
