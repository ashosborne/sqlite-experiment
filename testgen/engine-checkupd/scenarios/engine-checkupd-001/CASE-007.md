# engine-checkupd-001-C007 — run-11 oneshot characterization

Feature: `engine-checkupd-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE e(a INTEGER CHECK(a % 2 = 0)); INSERT INTO e VALUES(2); UPDATE e SET a = 4; SELECT a FROM e;
```

## Observables

- `row0.col0` = `4`
- `exec.rc` = `0`
- `cb.rows` = `1`
