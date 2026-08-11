# pragma-surface-002-C001 — run-11 oneshot characterization

Feature: `pragma-surface-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT count(*), (SELECT name FROM pragma_database_list LIMIT 1) FROM pragma_database_list;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `main`
- `exec.rc` = `0`
- `cb.rows` = `1`
