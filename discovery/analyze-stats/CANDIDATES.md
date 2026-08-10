# CANDIDATES — analyze-stats (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | ANALYZE execution → stat tables | `sqlite3Analyze` `src/analyze.c:1457`, `analyzeOneTable` `:977`, stat1/stat4 formats `src/analyze.c:20,23` | Writes planner statistics |
| 002 | Statistics loading at schema init | `sqlite3AnalysisLoad` `src/analyze.c:1942` | Reads stat tables into planner structures |
