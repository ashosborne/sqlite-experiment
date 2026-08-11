# engine-misc2-001-C004 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT replace(base85(X'DEADBEEF'),char(10),'|'), is_base85(replace(base85(X'DEADBEEF'),char(10),'')), is_base85('~');
```

Extension init (static, -DSQLITE_CORE, no load_extension): base85

## Observables

- `row0.col0` = `mVBSa|`
- `row0.col1` = `1`
- `row0.col2` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
