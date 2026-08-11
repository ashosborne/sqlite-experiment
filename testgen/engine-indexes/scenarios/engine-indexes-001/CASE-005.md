# engine-indexes-001-C005 — run-11 oneshot characterization

Feature: `engine-indexes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE k(id INTEGER PRIMARY KEY, v INTEGER); INSERT INTO k VALUES(1,10); || [upsert.rc] INSERT INTO k VALUES(1,99) ON CONFLICT(id) DO UPDATE SET v=excluded.v; || [read] SELECT v FROM k;
```

## Observables

- `write.rc` = `0`
- `upsert.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `99`
- `read.rc` = `0`
- `cb.rows` = `1`
