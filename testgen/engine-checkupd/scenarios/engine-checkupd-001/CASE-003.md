# engine-checkupd-001-C003 — run-11 oneshot characterization

Feature: `engine-checkupd-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE m(a INTEGER, b INTEGER, CHECK(a < b)); INSERT INTO m VALUES(1,5); UPDATE m SET a = 3; SELECT a, b FROM m;
```

## Observables

- `row0.col0` = `3`
- `row0.col1` = `5`
- `exec.rc` = `0`
- `cb.rows` = `1`
