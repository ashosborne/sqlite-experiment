# select-codegen-002-C001 — run-11 oneshot characterization

Feature: `select-codegen-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT 2 UNION SELECT 1 ORDER BY 1;
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `2`
- `exec.rc` = `0`
- `cb.rows` = `2`
