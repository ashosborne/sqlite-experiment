# engine-overflow-001-C007 — run-11 oneshot characterization

Feature: `engine-overflow-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE m3(t TEXT); INSERT INTO m3 VALUES('KLMNOPQRSTKLMNOPQRSTKLMNOPQRSTKLMNOPQRSTKLMNOPQRSTKLMNOPQRSTKLMNOPQRSTKL || [read] SELECT length(t), substr(t,14991,10) FROM m3; PRAGMA integrity_check;
```

## Observables

- `write.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `15000`
- `row0.col1` = `KLMNOPQRST`
- `row1.col0` = `ok`
- `read.rc` = `0`
- `cb.rows` = `2`
