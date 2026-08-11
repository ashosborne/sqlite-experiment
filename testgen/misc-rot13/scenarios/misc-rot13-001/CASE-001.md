# misc-rot13-001-C001 — run-11 oneshot characterization

Feature: `misc-rot13-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT rot13('Hello'), rot13(rot13('Hello'));
```

Extension init (static, -DSQLITE_CORE, no load_extension): rot13

## Observables

- `row0.col0` = `Uryyb`
- `row0.col1` = `Hello`
- `exec.rc` = `0`
- `cb.rows` = `1`
