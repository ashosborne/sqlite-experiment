# engine-checkupd-001-C005 — run-11 oneshot characterization

Feature: `engine-checkupd-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5); INSERT INTO c VALUES(20); UPDATE OR IGNORE c SET a = -1 WHERE a = 5; UPDATE OR IGNORE c SET a = 15 WHERE a = 20; SELECT a FROM c ORDER BY a;
```

## Observables

- `row0.col0` = `5`
- `row1.col0` = `15`
- `exec.rc` = `0`
- `cb.rows` = `2`
