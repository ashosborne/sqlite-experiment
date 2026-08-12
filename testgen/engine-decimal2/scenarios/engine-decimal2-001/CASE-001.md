# engine-decimal2-001-C001 — run-11 oneshot characterization

Feature: `engine-decimal2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT decimal_exp('123.5'), decimal_exp('-0.05'), decimal_exp('2');
```

Extension init (static, -DSQLITE_CORE, no load_extension): decimal

## Observables

- `row0.col0` = `+1.235e+02`
- `row0.col1` = `-5.0e-02`
- `row0.col2` = `+2.0e+00`
- `exec.rc` = `0`
- `cb.rows` = `1`
