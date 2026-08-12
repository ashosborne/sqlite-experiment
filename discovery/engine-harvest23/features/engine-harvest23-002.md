# engine-harvest23-002 — malloc accounting: memory_used / memory_highwater

Confidence: observed-in-code (composed slice, run 33).
usage grows/returns on alloc/free; highwater sticky; reset returns prior and re-arms.
Frozen scope = run-33 pinned cases (pack v23).
