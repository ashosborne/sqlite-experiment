# engine-triggers-001-C004 — run-11 oneshot characterization

Feature: `engine-triggers-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); CREATE TABLE lg(v TEXT); CREATE TRIGGER tb BEFORE INSERT ON t BEGIN INSERT INTO lg VALUES('B'); END; CREATE TRIGGER ta AFTER INSERT ON t BEGIN INSERT INTO lg VALUES('A'); END; INSERT INTO t VALUES(1); SELECT v FROM lg;
```

## Observables

- `row0.col0` = `B`
- `row1.col0` = `A`
- `exec.rc` = `0`
- `cb.rows` = `2`
