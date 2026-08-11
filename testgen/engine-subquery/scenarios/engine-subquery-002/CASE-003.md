# engine-subquery-002-C003 — run-11 oneshot characterization

Feature: `engine-subquery-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t1(x INTEGER); INSERT INTO t1 VALUES(1),(2),(3); CREATE TABLE t2(k INTEGER, w INTEGER); INSERT INTO t2 VALUES(1,1),(2,5),(3,3); SELECT x FROM t1 WHERE x = (SELECT w FROM t2 WHERE t2.k = t1.x LIMIT 1);
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `3`
- `exec.rc` = `0`
- `cb.rows` = `2`
