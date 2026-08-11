# engine-funcs-001-C005 — run-11 oneshot characterization

Feature: `engine-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT round(2.5), round(2.567,2), trim('  x  '), replace('aXbXc','X','-'), instr('hello','ll');
```

## Observables

- `row0.col0` = `3.0`
- `row0.col1` = `2.57`
- `row0.col2` = `x`
- `row0.col3` = `a-b-c`
- `row0.col4` = `3`
- `exec.rc` = `0`
- `cb.rows` = `1`
