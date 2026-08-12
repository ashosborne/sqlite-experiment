# engine-upsert3-001-C002 — run-11 oneshot characterization

Feature: `engine-upsert3-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE u(k INTEGER PRIMARY KEY, v INTEGER, w TEXT); INSERT INTO u VALUES(1,10,'a'); INSERT INTO u VALUES(1,5,'q') ON CONFLICT(k) DO UPDATE SET v = v + excluded.v; SELECT v, w FROM u;
```

## Observables

- `row0.col0` = `15`
- `row0.col1` = `a`
- `exec.rc` = `0`
- `cb.rows` = `1`
