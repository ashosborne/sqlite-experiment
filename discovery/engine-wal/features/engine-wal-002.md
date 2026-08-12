# engine-wal-002 — checkpoint pragmas (single-connection regime)

Confidence: observed-in-code (composed slice, run 32).
PRAGMA wal_checkpoint bare / PASSIVE / FULL / RESTART / TRUNCATE: busy=0,
log==checkpointed, TRUNCATE zeroes the -wal, backfill observable by a wal-blind
(immutable=1) read of the main db. Blocking/busy semantics across connections
are NOT exercised.
Frozen scope = run-32 pinned cases (pack v22).
