# engine-constraints-001-C004 — run-11 oneshot characterization

Feature: `engine-constraints-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE cf(a CHECK(a<10)); INSERT INTO cf VALUES(1); INSERT OR FAIL INTO cf VALUES(2),(20),(3); SELECT count(*) FROM cf;
```

## Observables

- `exec.rc` = `19`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
