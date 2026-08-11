# misc-percentile-001-C001 — run-11 oneshot characterization

Feature: `misc-percentile-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT median(x), percentile(x,25) FROM (SELECT 1 AS x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4);
```

## Observables

- `exec.rc` = `1`
- `cb.rows` = `0`
