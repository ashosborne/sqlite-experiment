# engine-vdbe48-003 — scan over a split interior root; expression boundary stays kitchen

Confidence: observed-in-code (composed slice, run 59).
After a 12-row overflow split (page_count 4, root page 2 now interior 0x05) the scan listing is unchanged and OpenRead on the interior root yields all rows in rowid order. Boundary pin: SELECT length(b) FROM t is an expression — not this pack's scan shape — and the kitchen evaluator still owns it (neither the dispatch nor the cursor-read counter moves).
Frozen scope = run-59 pinned cases (pack v48). No 3-level-tree claim.
