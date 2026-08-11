# CANDIDATES — misc-csv (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vtab-packs` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | CSV file virtual table | `ext/misc/csv.c:964` | Reads external CSV files as tables (filename= or data= args); filesystem access from SQL — security-relevant |
