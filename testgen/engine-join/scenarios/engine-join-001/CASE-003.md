# engine-join-001-C003 — run-11 oneshot characterization

Feature: `engine-join-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT e.name, d.dept FROM e, d WHERE e.id = d.eid ORDER BY d.dept, e.name;
```

## Observables

- `row0.col0` = `ann`
- `row0.col1` = `eng`
- `row1.col0` = `bob`
- `row1.col1` = `ops`
- `row2.col0` = `ann`
- `row2.col1` = `qa`
- `exec.rc` = `0`
- `cb.rows` = `3`
