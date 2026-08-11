# attach-detach-002-C001 — run-11 oneshot characterization

Feature: `attach-detach-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
ATTACH ':memory:' AS aux2; DETACH aux2; SELECT count(*) FROM pragma_database_list;
```

## Observables

- `row0.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
