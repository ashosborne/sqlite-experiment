# engine-trig2-001-C004 — run-11 oneshot characterization

Feature: `engine-trig2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER, b INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tu AFTER UPDATE OF a ON t BEGIN INSERT INTO lg VALUES(1); END; INSERT INTO t VALUES(1,1); UPDATE t SET b=2 WHERE a=1; UPDATE t SET a=3 WHERE b=2; SELECT count(*) FROM lg;
```

## Observables

- `row0.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
