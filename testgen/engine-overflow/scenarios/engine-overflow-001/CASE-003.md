# engine-overflow-001-C003 — run-11 oneshot characterization

Feature: `engine-overflow-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE ovb(b BLOB); INSERT INTO ovb VALUES(X'DEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADB || [read] SELECT length(b), substr(hex(b),1,8), substr(hex(b),11993,8) FROM ovb;
```

## Observables

- `write.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `6000`
- `row0.col1` = `DEADBEEF`
- `row0.col2` = `DEADBEEF`
- `read.rc` = `0`
- `cb.rows` = `1`
