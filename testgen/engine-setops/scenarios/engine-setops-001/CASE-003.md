# engine-setops-001-C003 — run-11 oneshot characterization

Feature: `engine-setops-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT x FROM (SELECT 1 AS x UNION ALL SELECT 1 UNION ALL SELECT 2) INTERSECT SELECT 1;
```

## Observables

- `row0.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
