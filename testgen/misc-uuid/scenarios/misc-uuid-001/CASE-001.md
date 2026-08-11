# misc-uuid-001-C001 — run-11 oneshot characterization

Feature: `misc-uuid-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT length(uuid()), substr(uuid(),15,1), uuid_str(uuid_blob(uuid())) IS NOT NULL;
```

Extension init (static, -DSQLITE_CORE, no load_extension): uuid

## Observables

- `row0.col0` = `36`
- `row0.col1` = `4`
- `row0.col2` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
