# engine-constraints-001-C008 — run-11 oneshot characterization

Feature: `engine-constraints-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA foreign_keys=ON; CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE RESTRICT); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1); DELETE FROM p2 WHERE id=1;
```

## Observables

- `exec.rc` = `19`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
