# misc-decimal-001-C001 — run-11 oneshot characterization

Feature: `misc-decimal-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT decimal_add('1.10','2.25'), decimal_cmp('2','10');
```

Extension init (static, -DSQLITE_CORE, no load_extension): decimal

## Observables

- `row0.col0` = `3.35`
- `row0.col1` = `-1`
- `exec.rc` = `0`
- `cb.rows` = `1`
