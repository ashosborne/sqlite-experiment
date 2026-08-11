# builtin-scalar-agg-funcs-003-C001 — run-11 oneshot characterization

Feature: `builtin-scalar-agg-funcs-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT 'abc' LIKE 'A%', 'abc' GLOB 'A*', 'a%c' LIKE 'a\%c' ESCAPE '\';
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `0`
- `row0.col2` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
