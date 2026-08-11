# misc-prefixes-001-C001 — run-11 oneshot characterization

Feature: `misc-prefixes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT count(*) FROM prefixes('abc');
```

Extension init (static, -DSQLITE_CORE, no load_extension): prefixes

## Observables

- `row0.col0` = `4`
- `exec.rc` = `0`
- `cb.rows` = `1`
