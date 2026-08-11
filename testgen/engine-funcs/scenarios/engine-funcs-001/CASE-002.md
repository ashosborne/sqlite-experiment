# engine-funcs-001-C002 — run-11 oneshot characterization

Feature: `engine-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT printf('%e|%g|%+d', 12345.678, 0.0001, 5), format('%08.3f', 2.5);
```

## Observables

- `row0.col0` = `1.234568e+04|0.0001|+5`
- `row0.col1` = `0002.500`
- `exec.rc` = `0`
- `cb.rows` = `1`
