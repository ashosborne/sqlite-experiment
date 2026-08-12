# engine-raise-001-C001 — run-11 oneshot characterization

Feature: `engine-raise-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER ti BEFORE INSERT ON t WHEN new.a < 0 BEGIN SELECT RAISE(IGNORE); END; INSERT INTO t VALUES(1),(-5),(2); SELECT count(*), sum(a) FROM t;
```

## Observables

- `row0.col0` = `2`
- `row0.col1` = `3`
- `exec.rc` = `0`
- `cb.rows` = `1`
