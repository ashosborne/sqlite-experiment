# engine-idxfile-002 — Durable/multi-leaf indexes + C interop

Confidence: observed-in-code (composed slice, run 27).
multi-leaf index b-trees, expression/partial/multi-col indexes survive reopen and are C-readable (integrity_check ok).
Frozen scope = the run-27 pinned cases (pack v17).
