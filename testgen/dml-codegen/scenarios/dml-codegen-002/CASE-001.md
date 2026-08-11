# dml-codegen-002-C001 — run-11 oneshot characterization

Feature: `dml-codegen-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE u(a UNIQUE); INSERT INTO u VALUES(1); INSERT OR REPLACE INTO u VALUES(1); INSERT OR IGNORE INTO u VALUES(1); SELECT count(*) FROM u;
```

## Observables

- `row0.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
