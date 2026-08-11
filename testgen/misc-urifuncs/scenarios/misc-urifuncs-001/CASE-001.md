# misc-urifuncs-001-C001 — run-11 oneshot characterization

Feature: `misc-urifuncs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT sqlite3_uri_parameter('main','vfs') IS NULL, sqlite3_uri_boolean('main','ro',0);
```

Extension init (static, -DSQLITE_CORE, no load_extension): urifuncs

## Observables

- `row0.col0` = `1`
- `row0.col1` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
