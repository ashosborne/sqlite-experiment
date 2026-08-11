# engine-constraints-001-C005 — run-11 oneshot characterization

Feature: `engine-constraints-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE n1(a); CREATE TABLE n2(a); SELECT a FROM n1, n2;
```

## Observables

- `exec.rc` = `1`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
