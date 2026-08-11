# engine-misc2-001-C010 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT tointeger(X'31') IS NULL, tointeger('9223372036854775808') IS NULL, tointeger(12.0), tointeger(12.5) IS NULL, toreal('abc') IS NULL;
```

Extension init (static, -DSQLITE_CORE, no load_extension): totype

## Observables

- `row0.col0` = `1`
- `row0.col1` = `1`
- `row0.col2` = `12`
- `row0.col3` = `1`
- `row0.col4` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
