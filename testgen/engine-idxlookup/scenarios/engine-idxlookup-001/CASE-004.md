# engine-idxlookup-001-C004 — run-11 oneshot characterization

Feature: `engine-idxlookup-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE u(k INTEGER, v TEXT); CREATE UNIQUE INDEX uk ON u(k); INSERT INTO u VALUES(1,'a'),(2,'b'),(3,'c'); SELECT v FROM u WHERE k = 2;
```

## Observables

- `row0.col0` = `b`
- `exec.rc` = `0`
- `cb.rows` = `1`
