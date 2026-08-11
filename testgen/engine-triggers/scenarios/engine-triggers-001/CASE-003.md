# engine-triggers-001-C003 — run-11 oneshot characterization

Feature: `engine-triggers-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tw AFTER INSERT ON t WHEN new.a > 10 BEGIN INSERT INTO lg VALUES(new.a); END; INSERT INTO t VALUES(5),(15); SELECT count(*), max(v) FROM lg;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `15`
- `exec.rc` = `0`
- `cb.rows` = `1`
