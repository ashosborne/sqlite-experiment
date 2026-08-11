# engine-funcs-001-C004 — run-11 oneshot characterization

Feature: `engine-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT 'Uryyb' = 'Hello' COLLATE rot13, 'abc' < 'abd' COLLATE rot13, rot13('Gung');
```

Extension init (static, -DSQLITE_CORE, no load_extension): rot13

## Observables

- `row0.col0` = `0`
- `row0.col1` = `1`
- `row0.col2` = `That`
- `exec.rc` = `0`
- `cb.rows` = `1`
