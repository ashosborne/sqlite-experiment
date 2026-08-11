# engine-triggers-001-C002 — run-11 oneshot characterization

Feature: `engine-triggers-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); CREATE TABLE lg(o INTEGER, n INTEGER); CREATE TRIGGER tu AFTER UPDATE ON t BEGIN INSERT INTO lg VALUES(old.a, new.a); END; INSERT INTO t VALUES(5); UPDATE t SET a=9 WHERE a=5; SELECT o, n FROM lg;
```

## Observables

- `row0.col0` = `5`
- `row0.col1` = `9`
- `exec.rc` = `0`
- `cb.rows` = `1`
