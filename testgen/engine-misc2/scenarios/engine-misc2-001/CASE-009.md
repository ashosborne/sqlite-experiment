# engine-misc2-001-C009 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE u(t TEXT); INSERT INTO u VALUES('x10'),('x9'),('x2'); SELECT t FROM u ORDER BY t COLLATE uint;
```

Extension init (static, -DSQLITE_CORE, no load_extension): uint

## Observables

- `row0.col0` = `x2`
- `row1.col0` = `x9`
- `row2.col0` = `x10`
- `exec.rc` = `0`
- `cb.rows` = `3`
