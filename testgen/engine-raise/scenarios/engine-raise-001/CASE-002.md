# engine-raise-001-C002 — run-11 oneshot characterization

Feature: `engine-raise-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tf BEFORE INSERT ON t WHEN new.a < 0 BEGIN SELECT RAISE(FAIL,'neg'); END; INSERT INTO t VALUES(-1);
```

## Observables

- `exec.rc` = `19`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
