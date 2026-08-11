# engine-indexes-001-C010 — run-11 oneshot characterization

Feature: `engine-indexes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE q(a INTEGER UNIQUE); INSERT INTO q VALUES(1); || [more.rc] INSERT INTO q VALUES(5); || [dup.rc] INSERT INTO q VALUES(5); || [read] SELECT count(*) FROM q;
```

## Observables

- `write.rc` = `0`
- `more.rc` = `0`
- `dup.rc` = `19`
- `reopen.rc` = `0`
- `row0.col0` = `2`
- `read.rc` = `0`
- `cb.rows` = `1`
