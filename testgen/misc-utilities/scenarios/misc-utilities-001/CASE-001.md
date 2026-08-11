# misc-utilities-001-C001 — run-11 oneshot characterization

Feature: `misc-utilities-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT eval('SELECT 3'), eval('SELECT 1; SELECT 2');
```

Extension init (static, -DSQLITE_CORE, no load_extension): eval

## Observables

- `row0.col0` = `3`
- `row0.col1` = `1 2`
- `exec.rc` = `0`
- `cb.rows` = `1`
