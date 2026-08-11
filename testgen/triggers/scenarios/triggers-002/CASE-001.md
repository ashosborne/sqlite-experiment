# triggers-002-C001 — run-11 oneshot characterization

Feature: `triggers-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE tr2(a); CREATE TABLE tlog2(v); CREATE TRIGGER trg2 AFTER INSERT ON tr2 BEGIN INSERT INTO tlog2 VALUES(new.a*2); END; INSERT INTO tr2 VALUES(7); SELECT v FROM tlog2;
```

## Observables

- `row0.col0` = `14`
- `exec.rc` = `0`
- `cb.rows` = `1`
