# pragma-surface-002-C003 — run-11 oneshot characterization

Feature: `pragma-surface-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT count(*) FROM pragma_compile_options;
```

## Observables

- `row0.col0` = `38`
- `exec.rc` = `0`
- `cb.rows` = `1`
