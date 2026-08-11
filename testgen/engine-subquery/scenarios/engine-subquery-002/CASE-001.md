# engine-subquery-002-C001 — run-11 oneshot characterization

Feature: `engine-subquery-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT EXISTS(SELECT 1 FROM t2 WHERE b=10), EXISTS(SELECT 1 FROM t2 WHERE b=99);
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
