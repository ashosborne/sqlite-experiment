# upsert-001-C001 — run-11 oneshot characterization

Feature: `upsert-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE up(a INTEGER PRIMARY KEY, b); INSERT INTO up VALUES(1,'x'); INSERT INTO up VALUES(1,'y') ON CONFLICT(a) DO NOTHING; SELECT b, count(*) FROM up;
```

## Observables

- `row0.col0` = `x`
- `row0.col1` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
