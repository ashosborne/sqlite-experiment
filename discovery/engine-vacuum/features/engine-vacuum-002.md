# engine-vacuum-002 — durable VACUUM + interop

Confidence: observed-in-code (composed slice, run 34).
File rows survive VACUUM + reopen with integrity ok; journal_mode preserved
(delete and wal); unique index still enforced after VACUUM + reopen. The pinned
C CLI reads Rust files after in-place VACUUM (mandatory interop test).
Frozen scope = run-34 pinned cases (pack v24).
