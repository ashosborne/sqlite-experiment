# engine-triggers-001-C001 — run-11 oneshot characterization

Feature: `engine-triggers-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER td AFTER DELETE ON t BEGIN INSERT INTO lg VALUES(old.a); END; INSERT INTO t VALUES(7),(8); DELETE FROM t WHERE a=7; SELECT v FROM lg;
```

## Observables

- `row0.col0` = `7`
- `exec.rc` = `0`
- `cb.rows` = `1`
