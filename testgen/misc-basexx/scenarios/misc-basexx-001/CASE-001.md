# misc-basexx-001-C001 — run-11 oneshot characterization

Feature: `misc-basexx-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT base64(X'01FF'), hex(base64('Af8='));
```

Extension init (static, -DSQLITE_CORE, no load_extension): base64

## Observables

- `row0.col0` = `Af8=`
- `row0.col1` = `01FF`
- `exec.rc` = `0`
- `cb.rows` = `1`
