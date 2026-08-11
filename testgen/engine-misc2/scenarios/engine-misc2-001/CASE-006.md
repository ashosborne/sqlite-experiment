# engine-misc2-001-C006 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT ieee754_from_blob(X'3FF0000000000000'), hex(ieee754_to_blob(1.5));
```

Extension init (static, -DSQLITE_CORE, no load_extension): ieee754

## Observables

- `row0.col0` = `1.0`
- `row0.col1` = `3FF8000000000000`
- `exec.rc` = `0`
- `cb.rows` = `1`
