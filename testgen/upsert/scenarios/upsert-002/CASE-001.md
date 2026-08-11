# upsert-002-C001 — run-11 oneshot characterization

Feature: `upsert-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE up2(a INTEGER PRIMARY KEY, b); INSERT INTO up2 VALUES(1,'x'); INSERT INTO up2 VALUES(1,'y') ON CONFLICT(a) DO UPDATE SET b=excluded.b; SELECT b FROM up2;
```

## Observables

- `row0.col0` = `y`
- `exec.rc` = `0`
- `cb.rows` = `1`
