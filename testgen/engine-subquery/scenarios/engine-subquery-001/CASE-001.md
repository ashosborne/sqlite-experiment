# engine-subquery-001-C001 — run-11 oneshot characterization

Feature: `engine-subquery-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t1(a INTEGER); INSERT INTO t1 VALUES(1),(2); CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT a, (SELECT max(b) FROM t2) FROM t1 ORDER BY a;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `20`
- `row1.col0` = `2`
- `row1.col1` = `20`
- `exec.rc` = `0`
- `cb.rows` = `2`
