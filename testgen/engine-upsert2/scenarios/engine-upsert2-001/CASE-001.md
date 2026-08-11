# engine-upsert2-001-C001 — run-11 oneshot characterization

Feature: `engine-upsert2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE u(k INTEGER PRIMARY KEY, v INTEGER); INSERT INTO u VALUES(1,10); INSERT INTO u VALUES(1,99) ON CONFLICT(k) DO UPDATE SET v = excluded.v WHERE excluded.v > 50; INSERT INTO u VALUES(1,5) ON CONFLICT(k) DO UPDATE SET v = excluded.v WHERE excluded.v > 50; SELECT v FROM u;
```

## Observables

- `row0.col0` = `99`
- `exec.rc` = `0`
- `cb.rows` = `1`
