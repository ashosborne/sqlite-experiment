# engine-idxlookup-002-C003 — run-11 oneshot characterization

Feature: `engine-idxlookup-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE pt(a INTEGER); CREATE INDEX ip ON pt(a) WHERE a > 10; INSERT INTO pt VALUES(5),(20); SELECT count(*) FROM pragma_index_list('pt'); SELECT a FROM pt WHERE a = 20; SELECT a FROM pt WHERE a = 5;
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `20`
- `row2.col0` = `5`
- `exec.rc` = `0`
- `cb.rows` = `3`
