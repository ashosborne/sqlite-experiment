# CANDIDATES — tokenizer (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Tokenizer + parser driver loop | `sqlite3RunParser` `src/tokenize.c:600`, `getToken` `:197` | Token classification incl. keywords (keywordhash.h generated from tool/mkkeywordhash.c — cite src only) |
| 002 | sqlite3_complete statement detection | `src/complete.c:340` | Public API: is this SQL statement terminated? |
