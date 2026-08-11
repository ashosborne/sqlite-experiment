# misc-func-packs-001-C001 — run-11 oneshot characterization

Feature: `misc-func-packs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT decimal_mul('1.5','2'), 'pack' REGEXP 'p.ck';
```

Extension init (static, -DSQLITE_CORE, no load_extension): decimal, regexp

## Observables

- `row0.col0` = `3`
- `row0.col1` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
