# engine-idxlookup-002-C004 — run-11 oneshot characterization

Feature: `engine-idxlookup-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE mu(a INTEGER, b INTEGER); CREATE UNIQUE INDEX mu2 ON mu(a, b); INSERT INTO mu VALUES(1,1),(1,2); INSERT INTO mu VALUES(1,1);
```

## Observables

- `exec.rc` = `19`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
