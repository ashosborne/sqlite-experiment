# engine-misc2-001-C003 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT lower(hex(sha3_query('SELECT 1',256)));
```

Extension init (static, -DSQLITE_CORE, no load_extension): shathree

## Observables

- `row0.col0` = `87d704d93c82458e27061eefd0458cf1809ecd90a77805eba72b9d2045522951`
- `exec.rc` = `0`
- `cb.rows` = `1`
