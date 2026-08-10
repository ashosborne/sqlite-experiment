# CANDIDATES — expr-codegen (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Value expression codegen | `sqlite3ExprCodeTarget` `src/expr.c:4972`, wrapper `:5931` | Core expression→opcode translation incl. affinity/collation |
| 002 | Boolean jump codegen (three-valued logic) | `sqlite3ExprIfTrue` `src/expr.c:6147`, `IfFalse` `:6313` | NULL-handling in WHERE conditions |
| 003 | Expression equivalence comparison | `sqlite3ExprCompare` `src/expr.c:6591` | Drives optimizer reuse decisions (idx-on-expr matching) |
