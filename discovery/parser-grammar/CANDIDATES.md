# CANDIDATES — parser-grammar (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. Grammar: 417 `::=` productions in `src/parse.y`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Statement grammar (cmd productions: txn, savepoint, DDL, DML) | `src/parse.y:181-207` (BEGIN/COMMIT/ROLLBACK/SAVEPOINT/CREATE), token prefix `:31` | Accepted-SQL dialect definition |
| 002 | Fallback/ambiguity policy (keywords usable as identifiers) | `%fallback` directives in `src/parse.y` | Dialect-compatibility behaviour worth pinning |
