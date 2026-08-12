# engine-idxlookup-002-C006 — run-11 oneshot characterization

Feature: `engine-idxlookup-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE pu(a INTEGER); CREATE UNIQUE INDEX pux ON pu(a) WHERE a > 10; INSERT INTO pu VALUES(5),(5),(20); INSERT INTO pu VALUES(20);
```

## Observables

- `exec.rc` = `19`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
