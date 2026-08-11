# misc-nextchar-001-C001 — run-11 oneshot characterization

Feature: `misc-nextchar-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE w(x TEXT); CREATE INDEX wx ON w(x); INSERT INTO w VALUES('cat'),('car'),('cow'); SELECT next_char('ca','w','x');
```

Extension init (static, -DSQLITE_CORE, no load_extension): nextchar

## Observables

- `row0.col0` = `rt`
- `exec.rc` = `0`
- `cb.rows` = `1`
