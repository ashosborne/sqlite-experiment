# engine-subquery-001-C002 — run-11 oneshot characterization

Feature: `engine-subquery-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t1(a INTEGER); INSERT INTO t1 VALUES(1),(2); CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT a FROM t1 WHERE a = (SELECT count(*) FROM t2);
```

## Observables

- `row0.col0` = `2`
- `exec.rc` = `0`
- `cb.rows` = `1`
