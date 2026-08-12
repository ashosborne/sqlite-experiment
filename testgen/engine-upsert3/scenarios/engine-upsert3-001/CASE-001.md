# engine-upsert3-001-C001 — run-11 oneshot characterization

Feature: `engine-upsert3-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE u(k INTEGER PRIMARY KEY, v INTEGER, w TEXT); INSERT INTO u VALUES(1,10,'a'); INSERT INTO u VALUES(1,99,'z') ON CONFLICT(k) DO UPDATE SET v=excluded.v, w=excluded.w; SELECT v, w FROM u;
```

## Observables

- `row0.col0` = `99`
- `row0.col1` = `z`
- `exec.rc` = `0`
- `cb.rows` = `1`
