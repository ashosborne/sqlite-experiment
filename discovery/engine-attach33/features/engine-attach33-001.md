# engine-attach33-001 — attached-trigger fire with unqualified body

Confidence: observed-in-code (composed slice, run 43).
CREATE TRIGGER aux.trg ON t fires; body writes aux objects only (collision-safe); WHEN gating; two schemas; unqualified-INSERT entry; aux.sqlite_master lists the trigger.
Frozen scope = run-43 pinned cases (pack v33).
