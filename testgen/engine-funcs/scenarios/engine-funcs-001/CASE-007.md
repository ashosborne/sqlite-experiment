# engine-funcs-001-C007 — run-11 oneshot characterization

Feature: `engine-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT concat('a','b','c'), concat_ws('-','x','y'), octet_length('abc'), unicode('A'), ltrim('xxay','x'), rtrim('yaxx','x');
```

## Observables

- `row0.col0` = `abc`
- `row0.col1` = `x-y`
- `row0.col2` = `3`
- `row0.col3` = `65`
- `row0.col4` = `ay`
- `row0.col5` = `ya`
- `exec.rc` = `0`
- `cb.rows` = `1`
