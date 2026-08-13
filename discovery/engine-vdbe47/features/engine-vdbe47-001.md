# engine-vdbe47-001 — EXPLAIN of constant SELECT is C's program

Confidence: observed-in-code (composed slice, run 58).
EXPLAIN of SELECT n / n WHERE 1|0 / n+m / 'text' / n,m emits C's exact listing: Init/Integer/String8/Add/ResultRow/Halt/Goto with C's p1-p5 and layout (WHERE-1 folds, WHERE-0 jumps Goto->Halt, 1+2 stays unfolded with init-section register loads after Halt; p4 NULL except String8, p5 0, comment NULL).
Frozen scope = run-58 pinned cases (pack v47). Constant SELECT only; anything with a FROM is out.
