# ddl-schema-001-C001 — run-11 oneshot characterization

Feature: `ddl-schema-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t1(a INTEGER PRIMARY KEY, b TEXT); SELECT count(*) FROM sqlite_master WHERE name='t1'; DROP TABLE t1; SELECT count(*) FROM sqlite_master;
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `2`
