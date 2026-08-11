# engine-funcs-001-C003 — run-11 oneshot characterization

Feature: `engine-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT decimal_sub('5.25','1.1'), decimal_mul('1.25','4'), decimal_add('0.1','0.2');
```

Extension init (static, -DSQLITE_CORE, no load_extension): decimal

## Observables

- `row0.col0` = `4.15`
- `row0.col1` = `5`
- `row0.col2` = `0.3`
- `exec.rc` = `0`
- `cb.rows` = `1`
