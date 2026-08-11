# engine-misc2-001-C008 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT '10' < '9', '10' < '9' COLLATE decimal;
```

Extension init (static, -DSQLITE_CORE, no load_extension): decimal

## Observables

- `row0.col0` = `1`
- `row0.col1` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
