# engine-overflow-001-C004 — run-11 oneshot characterization

Feature: `engine-overflow-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE mix(a INTEGER, t TEXT); INSERT INTO mix VALUES(1,'v1'); INSERT INTO mix VALUES(2,'v2'); INSERT INTO mix VAL || [read] SELECT count(*), max(length(t)) FROM mix;
```

## Observables

- `write.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `51`
- `row0.col1` = `6000`
- `read.rc` = `0`
- `cb.rows` = `1`
