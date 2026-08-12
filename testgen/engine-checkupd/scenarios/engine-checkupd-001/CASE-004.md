# engine-checkupd-001-C004 — run-11 oneshot characterization

Feature: `engine-checkupd-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE m(a INTEGER, b INTEGER, CHECK(a < b)); INSERT INTO m VALUES(1,5); UPDATE m SET a = 9;
```

## Observables

- `exec.rc` = `19`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
