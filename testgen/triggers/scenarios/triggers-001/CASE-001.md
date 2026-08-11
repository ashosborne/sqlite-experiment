# triggers-001-C001 — run-11 oneshot characterization

Feature: `triggers-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE tr(a); CREATE TABLE tlog(v); CREATE TRIGGER trg AFTER INSERT ON tr BEGIN INSERT INTO tlog VALUES(new.a); END; SELECT count(*) FROM sqlite_master WHERE type='trigger';
```

## Observables

- `row0.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
