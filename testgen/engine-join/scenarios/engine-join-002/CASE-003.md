# engine-join-002-C003 — run-11 oneshot characterization

Feature: `engine-join-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT count(*), min(e.id), max(d.dept) FROM e JOIN d ON e.id = d.eid;
```

## Observables

- `row0.col0` = `3`
- `row0.col1` = `1`
- `row0.col2` = `qa`
- `exec.rc` = `0`
- `cb.rows` = `1`
