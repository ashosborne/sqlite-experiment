# CANDIDATES — misc-utilities (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. Cluster (one candidate; per-file evidence). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Utility extension pack (~14) | `fileio.c:1266` (readfile/writefile/fsdir), `dbdump.c:695` (programmatic .dump), `eval.c:119`, `explain.c:312`, `memstat.c:422`, `diskused.c:848`, `noop.c:74`, `randomjson.c:217`, `remember.c:62`, `stmtrand.c:82`, `strdup.c:98`, `showauth.c:93`, `anycollseq.c:49`, `normalize.c` (sqlite3_normalize; test main skipped) (all `ext/misc/`) | Ops/debug utilities incl. filesystem access (fileio — security-relevant) |
