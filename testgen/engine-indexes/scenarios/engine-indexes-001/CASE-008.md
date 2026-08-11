# engine-indexes-001-C008 — run-11 oneshot characterization

Feature: `engine-indexes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE mc(a INTEGER, b INTEGER, UNIQUE(a,b)); INSERT INTO mc VALUES(1,1),(1,2); || [dup.rc] INSERT INTO mc VALUES(1,1); || [ok.rc] INSERT INTO mc VALUES(2,1); || [read] SELECT count(*) FROM mc; PRAGMA integrity_check;
```

## Observables

- `write.rc` = `0`
- `dup.rc` = `19`
- `ok.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `3`
- `row1.col0` = `ok`
- `read.rc` = `0`
- `cb.rows` = `2`
