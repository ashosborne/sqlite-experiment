# engine-misc2-001-C005 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT hex(base85(base85(X'0102030405')));
```

Extension init (static, -DSQLITE_CORE, no load_extension): base85

## Observables

- `row0.col0` = `0102030405`
- `exec.rc` = `0`
- `cb.rows` = `1`
