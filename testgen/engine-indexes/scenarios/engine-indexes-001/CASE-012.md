# engine-indexes-001-C012 — run-11 oneshot characterization

Feature: `engine-indexes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE q(a INTEGER UNIQUE); INSERT INTO q VALUES(1); || [rep.rc] INSERT OR REPLACE INTO q VALUES(1); || [read] SELECT count(*) FROM q;
```

## Observables

- `write.rc` = `0`
- `rep.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `1`
- `read.rc` = `0`
- `cb.rows` = `1`
