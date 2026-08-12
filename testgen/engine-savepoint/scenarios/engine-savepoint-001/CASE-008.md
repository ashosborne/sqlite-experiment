# engine-savepoint-001-C008 — run-11 oneshot characterization

Feature: `engine-savepoint-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE q(a INTEGER); SAVEPOINT a; RELEASE a; ROLLBACK TO a;
```

## Observables

- `exec.rc` = `1`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
