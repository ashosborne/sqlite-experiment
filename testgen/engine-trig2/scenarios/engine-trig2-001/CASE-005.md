# engine-trig2-001-C005 — run-11 oneshot characterization

Feature: `engine-trig2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA recursive_triggers=ON; CREATE TABLE t(a INTEGER); CREATE TRIGGER tr AFTER INSERT ON t WHEN new.a < 3 BEGIN INSERT INTO t VALUES(new.a + 1); END; INSERT INTO t VALUES(1); SELECT count(*), max(a) FROM t;
```

## Observables

- `row0.col0` = `3`
- `row0.col1` = `3`
- `exec.rc` = `0`
- `cb.rows` = `1`
