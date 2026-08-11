# misc-series-001-C001 — run-11 oneshot characterization

Feature: `misc-series-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT count(*), sum(value) FROM generate_series(1,5);
```

Extension init (static, -DSQLITE_CORE, no load_extension): series

## Observables

- `row0.col0` = `5`
- `row0.col1` = `15`
- `exec.rc` = `0`
- `cb.rows` = `1`
