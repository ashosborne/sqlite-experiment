# misc-fossildelta-001-C001 — run-11 oneshot characterization

Feature: `misc-fossildelta-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT delta_apply('abc', delta_create('abc','abcd'))='abcd', delta_output_size(delta_create('abc','abcd'));
```

Extension init (static, -DSQLITE_CORE, no load_extension): fossildelta

## Observables

- `row0.col0` = `0`
- `row0.col1` = `4`
- `exec.rc` = `0`
- `cb.rows` = `1`
