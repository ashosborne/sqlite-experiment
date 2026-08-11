# ddl-schema-003-C001 — run-11 oneshot characterization

Feature: `ddl-schema-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t3(a); ALTER TABLE t3 RENAME TO t3x; ALTER TABLE t3x ADD COLUMN b DEFAULT 5; INSERT INTO t3x(a) VALUES(9); SELECT a,b FROM t3x;
```

## Observables

- `row0.col0` = `9`
- `row0.col1` = `5`
- `exec.rc` = `0`
- `cb.rows` = `1`
