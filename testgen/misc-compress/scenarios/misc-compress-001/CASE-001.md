# misc-compress-001-C001 — run-11 oneshot characterization

Feature: `misc-compress-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT hex(uncompress(compress('hello')))=hex('hello'), length(compress(zeroblob(1000)))<1000;
```

Extension init (static, -DSQLITE_CORE, no load_extension): compress

## Observables

- `row0.col0` = `1`
- `row0.col1` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
