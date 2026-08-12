# engine-checkupd-003 — CHECK constraints on UPDATE (post-update image + OR-modes)

Confidence: observed-in-code (composed slice, run 26).
column + table CHECKs on the post-update row image; OR IGNORE/ABORT/FAIL/ROLLBACK semantics against v15 txns; durable file twins.
Frozen scope = the run-26 pinned cases (pack v16).
