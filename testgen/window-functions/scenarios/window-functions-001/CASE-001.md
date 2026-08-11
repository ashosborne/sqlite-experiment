# window-functions-001-C001 — run-11 oneshot characterization

Feature: `window-functions-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT x, row_number() OVER (ORDER BY x) FROM (SELECT 30 AS x UNION ALL SELECT 10 UNION ALL SELECT 20) ORDER BY x;
```

## Observables

- `row0.col0` = `10`
- `row0.col1` = `1`
- `row1.col0` = `20`
- `row1.col1` = `2`
- `row2.col0` = `30`
- `row2.col1` = `3`
- `exec.rc` = `0`
- `cb.rows` = `3`
