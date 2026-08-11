# engine-funcs-001-C006 — run-11 oneshot characterization

Feature: `engine-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT min(3,1,2), max(3,1,2), sign(-5), sign(0), char(65,66,67), unhex('4142') = X'4142';
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `3`
- `row0.col2` = `-1`
- `row0.col3` = `0`
- `row0.col4` = `ABC`
- `row0.col5` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
