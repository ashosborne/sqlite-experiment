# expr-codegen-001-C001 — run-11 oneshot characterization

Feature: `expr-codegen-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT 1+2*3, 'a'||'b', CAST('12x' AS INTEGER), CAST(2.9 AS INTEGER);
```

## Observables

- `row0.col0` = `7`
- `row0.col1` = `ab`
- `row0.col2` = `12`
- `row0.col3` = `2`
- `exec.rc` = `0`
- `cb.rows` = `1`
