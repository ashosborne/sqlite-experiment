# engine-views-001-C003 — run-11 oneshot characterization

Feature: `engine-views-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE vt(a INTEGER); CREATE VIEW v AS SELECT a FROM vt; INSERT INTO v VALUES(1);
```

## Observables

- `exec.rc` = `1`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
