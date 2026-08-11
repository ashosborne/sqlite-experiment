# engine-constraints-001-C001 — run-11 oneshot characterization

Feature: `engine-constraints-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE c(a CHECK(a>0)); INSERT INTO c VALUES(-1);
```

## Observables

- `exec.rc` = `19`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
