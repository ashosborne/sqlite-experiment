# CANDIDATES — builtin-scalar-agg-funcs (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. Registry: `sqlite3RegisterBuiltinFunctions` `src/func.c:3311`, table `aBuiltinFunc[]` `src/func.c:3322` (~111 registration rows). Clustered by family to avoid card-per-function noise. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Core scalar functions (string/numeric: length, substr, upper/lower, abs, round, hex, quote, coalesce, typeof...) | table rows e.g. `src/func.c:3404-3407` (substr), `src/func.c:348` (`substrFunc`) | Largest SQL-visible function family |
| 002 | Aggregates (count, sum/total/avg, min/max, group_concat) | `src/func.c:3408` (`WAGGREGATE(sum...)`), `sumStep` `src/func.c:1929`, minmax rows `src/func.c:3357,3360` | Aggregate step/finalize semantics incl. NULL rules |
| 003 | LIKE/GLOB machinery + case_sensitive_like | `likeFunc` `src/func.c:921`, `sqlite3RegisterLikeFunctions` `src/func.c:2348`, `sqlite3IsLikeFunction` `src/func.c:2388` | Pattern-match operators backed by functions; optimizer hook |
