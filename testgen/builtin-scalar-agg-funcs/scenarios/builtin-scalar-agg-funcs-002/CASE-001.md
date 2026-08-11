# builtin-scalar-agg-funcs-002-C001 — run-11 oneshot characterization

Feature: `builtin-scalar-agg-funcs-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT sum(x), total(x), count(*), count(x), group_concat(x,'-') FROM (SELECT 1 AS x UNION ALL SELECT NULL UNION ALL SELECT 2);
```

## Observables

- `row0.col0` = `3`
- `row0.col1` = `3.0`
- `row0.col2` = `3`
- `row0.col3` = `2`
- `row0.col4` = `1-2`
- `exec.rc` = `0`
- `cb.rows` = `1`
