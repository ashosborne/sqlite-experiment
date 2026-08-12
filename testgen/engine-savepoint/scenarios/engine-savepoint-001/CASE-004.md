# engine-savepoint-001-C004 — run-11 oneshot characterization

Feature: `engine-savepoint-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE q(a INTEGER); SAVEPOINT a; INSERT INTO q VALUES(1); ROLLBACK TO a; INSERT INTO q VALUES(2); RELEASE a; SELECT a FROM q;
```

## Observables

- `row0.col0` = `2`
- `exec.rc` = `0`
- `cb.rows` = `1`
