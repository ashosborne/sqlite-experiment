# engine-vdbe47-002 — stepping a constant SELECT dispatches that program

Confidence: observed-in-code (composed slice, run 58).
sqlite3_step of the same SQL executes the compiled program through a dispatch loop (registers, pc jumps) and returns the constant rows (zero rows for WHERE 0); step/step/reset/step cycles like C (ROW+value, DONE, ROW+value). The dispatch counter moves; kitchen-fallback SQL does not move it.
Frozen scope = run-58 pinned cases (pack v47).
