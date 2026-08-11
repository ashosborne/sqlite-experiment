# misc-csv-001-C001 — run-11 oneshot characterization

Feature: `misc-csv-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE VIRTUAL TABLE temp.c1 USING csv(data='1,2
3,4'); SELECT c0,c1 FROM c1;
```

Extension init (static, -DSQLITE_CORE, no load_extension): csv

## Observables

- `row0.col0` = `1`
- `row0.col1` = `2`
- `row1.col0` = `3`
- `row1.col1` = `4`
- `exec.rc` = `0`
- `cb.rows` = `2`
