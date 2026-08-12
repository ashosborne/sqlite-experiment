# engine-checkupd-001-C002 — run-11 oneshot characterization

Feature: `engine-checkupd-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5); UPDATE c SET a = 7; SELECT a FROM c;
```

## Observables

- `row0.col0` = `7`
- `exec.rc` = `0`
- `cb.rows` = `1`
