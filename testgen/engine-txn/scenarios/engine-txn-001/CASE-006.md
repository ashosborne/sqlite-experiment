# engine-txn-001-C006 — run-11 oneshot characterization

Feature: `engine-txn-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); BEGIN; INSERT INTO t VALUES(1); INSERT INTO t VALUES(2); INSERT INTO t VALUES(3); ROLLBACK; SELECT count(*) FROM t;
```

## Observables

- `row0.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
