# CANDIDATES — printf-format (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | SQL printf()/format() | registry `src/func.c:3369,3370`, impl `printfFunc` `src/func.c:315`, SQLFUNC arg mode `src/printf.c:244` | SQL-visible formatter incl. %q/%Q/%w SQL-quoting directives |
| 002 | C API mprintf/snprintf | `src/printf.c:1519,1559` | Public formatting API with SQLite-specific directives |
| 003 | sqlite3_str dynamic string builder | `sqlite3_str_new` `src/printf.c:1447` | Public accumulator API with precision/width limits (`src/printf.c:192,303`) |
