# misc-ieee754-001-C001 — run-11 oneshot characterization

Feature: `misc-ieee754-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT ieee754(2.5), ieee754_mantissa(2.5), ieee754_exponent(2.5);
```

Extension init (static, -DSQLITE_CORE, no load_extension): ieee754

## Observables

- `row0.col0` = `ieee754(5,-1)`
- `row0.col1` = `5`
- `row0.col2` = `-1`
- `exec.rc` = `0`
- `cb.rows` = `1`
