# expr-codegen-002-C001 — run-11 oneshot characterization

Feature: `expr-codegen-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT 1 IN (2,NULL), 3 IN (2,NULL,3), NULL IS NULL, NULL = NULL;
```

## Observables

- `row0.col0` = `NULL`
- `row0.col1` = `1`
- `row0.col2` = `1`
- `row0.col3` = `NULL`
- `exec.rc` = `0`
- `cb.rows` = `1`
