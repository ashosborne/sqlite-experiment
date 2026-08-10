# CANDIDATES — foreign-keys (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Immediate/deferred FK checking | `sqlite3FkCheck` `src/fkey.c:889`, `sqlite3FkRequired` `:1145` | Constraint-counter semantics per statement vs transaction |
| 002 | Cascading actions (CASCADE/SET NULL/SET DEFAULT/RESTRICT) | `fkActionTrigger` `src/fkey.c:1217`, `sqlite3FkActions` `:1419` | Synthesized action triggers |
| 003 | FK schema interactions on DROP TABLE | `sqlite3FkDropTable` `src/fkey.c:736` | DDL-time FK bookkeeping |
