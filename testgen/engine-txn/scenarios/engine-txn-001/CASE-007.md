# engine-txn-001-C007 — run-11 oneshot characterization

Feature: `engine-txn-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); BEGIN; CREATE TABLE d(x INTEGER); INSERT INTO d VALUES(5); ROLLBACK; SELECT count(*) FROM sqlite_master WHERE name='d'; SELECT count(*) FROM t;
```

## Observables

- `row0.col0` = `0`
- `row1.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `2`
