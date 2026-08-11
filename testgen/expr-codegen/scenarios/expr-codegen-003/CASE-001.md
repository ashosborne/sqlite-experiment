# expr-codegen-003-C001 — run-11 oneshot characterization

Feature: `expr-codegen-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT CASE WHEN 1 THEN 'y' ELSE 'n' END, iif(0,'a','b');
```

## Observables

- `row0.col0` = `y`
- `row0.col1` = `b`
- `exec.rc` = `0`
- `cb.rows` = `1`
