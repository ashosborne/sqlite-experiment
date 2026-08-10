# CANDIDATES — pager (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Transaction lifecycle + two-phase commit | `sqlite3PagerBegin` `src/pager.c:5976`, `CommitPhaseOne` `:6519`, `Rollback` `:6822`, open `:4789`, get `:5780` | ACID boundary of the storage stack |
| 002 | Journal-mode state machine | `sqlite3PagerSetJournalMode` `src/pager.c:7415` | delete/truncate/persist/memory/wal/off transitions with legality rules |
