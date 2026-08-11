# engine-indexes-001-C003 — run-11 oneshot characterization

Feature: `engine-indexes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE s(a INTEGER); CREATE INDEX sa ON s(a); INSERT INTO s VALUES(1),(2),(2); || [read] SELECT count(*) FROM sqlite_master WHERE type='index'; SELECT count(*) FROM pragma_index_list('s'); SELECT count(*) FROM
```

## Observables

- `write.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `1`
- `row1.col0` = `1`
- `row2.col0` = `3`
- `row3.col0` = `ok`
- `read.rc` = `0`
- `cb.rows` = `4`
