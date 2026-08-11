# window-functions-002-C001 — run-11 oneshot characterization

Feature: `window-functions-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT x, sum(x) OVER (ORDER BY x ROWS BETWEEN 1 PRECEDING AND CURRENT ROW) FROM (SELECT 10 AS x UNION ALL SELECT 20 UNION ALL SELECT 30) ORDER BY x;
```

## Observables

- `row0.col0` = `10`
- `row0.col1` = `10`
- `row1.col0` = `20`
- `row1.col1` = `30`
- `row2.col0` = `30`
- `row2.col1` = `50`
- `exec.rc` = `0`
- `cb.rows` = `3`
