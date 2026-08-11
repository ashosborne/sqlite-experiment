# engine-trig2-001-C002 — run-11 oneshot characterization

Feature: `engine-trig2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tg AFTER INSERT ON t BEGIN INSERT INTO lg VALUES(1); END; SELECT count(*) FROM sqlite_master WHERE type='trigger'; DROP TRIGGER tg; SELECT count(*) FROM sqlite_master WHERE type='trigger'; INSERT INTO t VALUES(1); SELECT count(*) FROM lg;
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `0`
- `row2.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `3`
