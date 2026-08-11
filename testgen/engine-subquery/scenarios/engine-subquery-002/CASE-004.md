# engine-subquery-002-C004 — run-11 oneshot characterization

Feature: `engine-subquery-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t1(x INTEGER); INSERT INTO t1 VALUES(1),(2),(3); CREATE TABLE t2(k INTEGER, w INTEGER); INSERT INTO t2 VALUES(1,1),(2,5),(3,3); SELECT x, (SELECT w FROM t2 WHERE t2.k = t1.x) FROM t1 ORDER BY x;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `1`
- `row1.col0` = `2`
- `row1.col1` = `5`
- `row2.col0` = `3`
- `row2.col1` = `3`
- `exec.rc` = `0`
- `cb.rows` = `3`
