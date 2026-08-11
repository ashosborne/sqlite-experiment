# misc-wholenumber-001-C001 — run-11 oneshot characterization

Feature: `misc-wholenumber-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE VIRTUAL TABLE temp.w USING wholenumber; SELECT count(*) FROM w WHERE value BETWEEN 1 AND 5;
```

Extension init (static, -DSQLITE_CORE, no load_extension): wholenumber

## Observables

- `row0.col0` = `5`
- `exec.rc` = `0`
- `cb.rows` = `1`
