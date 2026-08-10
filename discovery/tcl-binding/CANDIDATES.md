# CANDIDATES — tcl-binding (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | TCL package init + sqlite3 command object | `Sqlite3_Init` `src/tclsqlite.c:4469` (+ alias inits `:4483-4503`), per-db command dispatch (`DbObjCmd`, same file) | Language-binding adapter with eval/transaction/function subcommands |
