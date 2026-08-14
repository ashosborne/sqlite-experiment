# engine-vdbe48-002 — stepping the scan walks btree cells through the dispatch loop

Confidence: observed-in-code (composed slice, run 59).
sqlite3_step of the same SQL runs the program: OpenRead opens the cursor on cells parsed from the file image, Rewind positions on the first cell (or jumps to Halt when empty — first step DONE), Column decodes the current cell's payload into a register, Next advances and loops. Rows come back in rowid order; reset replays the scan; a later INSERT is visible on the next step.
Frozen scope = run-59 pinned cases (pack v48).
