# misc-uint-001-C001 — run-11 oneshot characterization

Feature: `misc-uint-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT 'x9' < 'x10' COLLATE uint, 'x9' < 'x10';
```

Extension init (static, -DSQLITE_CORE, no load_extension): uint

## Observables

- `row0.col0` = `1`
- `row0.col1` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
