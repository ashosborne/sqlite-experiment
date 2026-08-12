# engine-conn-002 — busy handler / timeout over the file write lock

Confidence: observed-in-code (composed slice, run 36).
Second connection's write blocked while a txn holds the file lock ("database is
locked", rc 5); handler invoked with increasing counts (1-call and 3-call
shapes); handler and timeout mutually exclusive; timeout(0) clears; COMMIT
unblocks. Single-process lock model only — C's cross-process locking not claimed.
Frozen scope = run-36 pinned cases (pack v26).
