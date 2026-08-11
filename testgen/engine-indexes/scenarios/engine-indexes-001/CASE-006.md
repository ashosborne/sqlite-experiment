# engine-indexes-001-C006 — run-11 oneshot characterization

Feature: `engine-indexes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE m(id INTEGER PRIMARY KEY, u TEXT UNIQUE); INSERT INTO m VALUES(1,'x'),(2,'y'); || [dup.rc] INSERT INTO m VALUES(3,'x'); || [read] SELECT count(*) FROM m; PRAGMA integrity_check;
```

## Observables

- `write.rc` = `0`
- `dup.rc` = `19`
- `reopen.rc` = `0`
- `row0.col0` = `2`
- `row1.col0` = `ok`
- `read.rc` = `0`
- `cb.rows` = `2`
