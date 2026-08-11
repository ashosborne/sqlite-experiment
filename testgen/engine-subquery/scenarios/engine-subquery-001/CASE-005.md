# engine-subquery-001-C005 — run-11 oneshot characterization

Feature: `engine-subquery-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT (SELECT x FROM (SELECT 5 AS x UNION ALL SELECT 3) ORDER BY x LIMIT 1);
```

## Observables

- `row0.col0` = `3`
- `exec.rc` = `0`
- `cb.rows` = `1`
