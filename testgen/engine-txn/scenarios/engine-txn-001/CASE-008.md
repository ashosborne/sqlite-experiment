# engine-txn-001-C008 — run-11 oneshot characterization

Feature: `engine-txn-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); BEGIN DEFERRED; INSERT INTO t VALUES(4); COMMIT; BEGIN IMMEDIATE; INSERT INTO t VALUES(5); COMMIT; SELECT count(*) FROM t;
```

## Observables

- `row0.col0` = `2`
- `exec.rc` = `0`
- `cb.rows` = `1`
