# engine-overflow-001-C006 — run-11 oneshot characterization

Feature: `engine-overflow-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE sh(t TEXT); INSERT INTO sh VALUES('abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijab || [shrink.rc] UPDATE sh SET t='tiny'; || [read] SELECT length(t), t FROM sh; PRAGMA integrity_check;
```

## Observables

- `write.rc` = `0`
- `shrink.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `4`
- `row0.col1` = `tiny`
- `row1.col0` = `ok`
- `read.rc` = `0`
- `cb.rows` = `2`
