# engine-savepoint-001-C001 — run-11 oneshot characterization

Feature: `engine-savepoint-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE q(a INTEGER); SAVEPOINT s1; INSERT INTO q VALUES(1); ROLLBACK TO s1; RELEASE s1; SELECT count(*) FROM q;
```

## Observables

- `row0.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
