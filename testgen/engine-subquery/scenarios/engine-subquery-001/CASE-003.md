# engine-subquery-001-C003 — run-11 oneshot characterization

Feature: `engine-subquery-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT (SELECT b FROM t2 ORDER BY b LIMIT 1), (SELECT b FROM t2 ORDER BY b LIMIT 1 OFFSET 1);
```

## Observables

- `row0.col0` = `10`
- `row0.col1` = `20`
- `exec.rc` = `0`
- `cb.rows` = `1`
