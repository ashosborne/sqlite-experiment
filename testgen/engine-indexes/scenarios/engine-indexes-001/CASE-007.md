# engine-indexes-001-C007 — run-11 oneshot characterization

Feature: `engine-indexes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE d(a INTEGER); CREATE INDEX da ON d(a); DROP INDEX da; INSERT INTO d VALUES(7); || [read] SELECT count(*) FROM sqlite_master WHERE type='index'; SELECT a FROM d;
```

## Observables

- `write.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `0`
- `row1.col0` = `7`
- `read.rc` = `0`
- `cb.rows` = `2`
