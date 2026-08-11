# engine-overflow-001-C001 — run-11 oneshot characterization

Feature: `engine-overflow-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE ov(a INTEGER, t TEXT); INSERT INTO ov VALUES(1,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghi || [read] SELECT length(t), substr(t,1,10), substr(t,5991,10) FROM ov;
```

## Observables

- `write.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `6000`
- `row0.col1` = `abcdefghij`
- `row0.col2` = `abcdefghij`
- `read.rc` = `0`
- `cb.rows` = `1`
