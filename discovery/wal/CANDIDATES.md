# CANDIDATES — wal (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | WAL write path + reader snapshots | `sqlite3WalOpen` `src/wal.c:1647`, `WalFrames` `:4275`, `BeginReadTransaction` `:3493` | Concurrent-reader snapshot isolation |
| 002 | Checkpoint modes (PASSIVE/FULL/RESTART/TRUNCATE) | `sqlite3WalCheckpoint` `src/wal.c:4301`, `walCheckpoint` `:2199` | Backfill semantics + blocking behaviour per mode |
