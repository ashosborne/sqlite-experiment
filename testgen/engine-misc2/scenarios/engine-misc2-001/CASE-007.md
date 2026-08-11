# engine-misc2-001-C007 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT decimal('7.10'), decimal('-0012.3400'), decimal_pow2(10), decimal_pow2(-3);
```

Extension init (static, -DSQLITE_CORE, no load_extension): decimal

## Observables

- `row0.col0` = `7.10`
- `row0.col1` = `-12.3400`
- `row0.col2` = `+1.024e+03`
- `row0.col3` = `+1.25e-01`
- `exec.rc` = `0`
- `cb.rows` = `1`
