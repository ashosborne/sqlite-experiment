# misc-shathree-001-C001 — run-11 oneshot characterization

Feature: `misc-shathree-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT lower(hex(sha3('abc',256)));
```

Extension init (static, -DSQLITE_CORE, no load_extension): shathree

## Observables

- `row0.col0` = `3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532`
- `exec.rc` = `0`
- `cb.rows` = `1`
