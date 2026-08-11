# pragma-surface-002-C002 — run-11 oneshot characterization

Feature: `pragma-surface-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE pt(a INTEGER PRIMARY KEY, b TEXT REFERENCES pt(a)); CREATE INDEX pi ON pt(b); SELECT count(*) FROM pragma_table_info('pt'); SELECT count(*) FROM pragma_foreign_key_list('pt'); SELECT count(*) FROM pragma_index_list('pt');
```

## Observables

- `row0.col0` = `2`
- `row1.col0` = `1`
- `row2.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `3`
