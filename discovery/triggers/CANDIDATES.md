# CANDIDATES — triggers (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Trigger DDL (create/finish/drop) | `src/trigger.c:104,324,659` | Schema objects with parse-time validation |
| 002 | Row-trigger firing (BEFORE/AFTER/INSTEAD OF) | `sqlite3TriggersExist` `src/trigger.c:870`, `codeRowTrigger` `:1232`, `sqlite3CodeRowTrigger` `:1469`, direct `:1397` | Sub-program compilation + recursion limits |
