# engine-vdbe50-002 — OP_Insert puts the MakeRecord cell through the pager cursor

Confidence: observed-in-code (composed slice, run 61).
Stepping the INSERT dispatches the program: NewRowid computes max(rowid)+1, MakeRecord encodes the registers into a record blob, and Insert puts THAT payload verbatim through the pager's cell packing (split-capable), committed via the journal mini-txn — never the kitchen store, never the whole-image writer. DONE + last_insert_rowid match C; reset+step inserts again with C's next rowids; changes/total_changes match; bound ?,? flows through Variable; VDBE-inserted rows read back through the v48 scan, the v49 WHERE compare and the rowid seek.
Frozen scope = run-61 pinned cases (pack v50).
