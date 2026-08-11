# engine-subquery-002-C002 — run-11 oneshot characterization

Feature: `engine-subquery-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t1(a INTEGER); INSERT INTO t1 VALUES(1),(2),(3); CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT a FROM t1 WHERE NOT EXISTS (SELECT 1 FROM t2 WHERE b = a*10);
```

## Observables

- `row0.col0` = `3`
- `exec.rc` = `0`
- `cb.rows` = `1`
