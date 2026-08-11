# misc-sha1-001-C001 — run-11 oneshot characterization

Feature: `misc-sha1-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT sha1('abc');
```

Extension init (static, -DSQLITE_CORE, no load_extension): sha1

## Observables

- `row0.col0` = `a9993e364706816aba3e25717850c26c9cd0d89d`
- `exec.rc` = `0`
- `cb.rows` = `1`
