# builtin-scalar-agg-funcs-001-C001 — run-11 oneshot characterization

Feature: `builtin-scalar-agg-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT upper('abc'), length('hello'), substr('abcdef',-3,2), coalesce(NULL,7), typeof(2.0);
```

## Observables

- `row0.col0` = `ABC`
- `row0.col1` = `5`
- `row0.col2` = `de`
- `row0.col3` = `7`
- `row0.col4` = `real`
- `exec.rc` = `0`
- `cb.rows` = `1`
