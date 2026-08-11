# engine-indexes-001-C004 — run-11 oneshot characterization

Feature: `engine-indexes-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE q(a INTEGER UNIQUE); INSERT INTO q VALUES(1); || [ins.rc] INSERT OR IGNORE INTO q VALUES(1),(5); || [read] SELECT a FROM q ORDER BY a;
```

## Observables

- `write.rc` = `0`
- `ins.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `1`
- `row1.col0` = `5`
- `read.rc` = `0`
- `cb.rows` = `2`
