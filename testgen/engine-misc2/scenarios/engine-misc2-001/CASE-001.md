# engine-misc2-001-C001 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT sha1_query('SELECT 1');
```

Extension init (static, -DSQLITE_CORE, no load_extension): sha1

## Observables

- `row0.col0` = `ef27bcb1ef9e0359c8d4e4d96f2dbc2ddf61dc31`
- `exec.rc` = `0`
- `cb.rows` = `1`
