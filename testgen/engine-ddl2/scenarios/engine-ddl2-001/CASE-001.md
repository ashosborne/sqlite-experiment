# engine-ddl2-001-C001 — run-11 oneshot characterization

Feature: `engine-ddl2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE r(a INTEGER, b TEXT); INSERT INTO r VALUES(1,'x'); ALTER TABLE r RENAME COLUMN b TO c; SELECT c FROM r;
```

## Observables

- `row0.col0` = `x`
- `exec.rc` = `0`
- `cb.rows` = `1`
