# engine-join-002-C004 — run-11 oneshot characterization

Feature: `engine-join-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT e.name, count(*) FROM e JOIN d ON e.id = d.eid GROUP BY e.name ORDER BY e.name;
```

## Observables

- `row0.col0` = `ann`
- `row0.col1` = `2`
- `row1.col0` = `bob`
- `row1.col1` = `1`
- `exec.rc` = `0`
- `cb.rows` = `2`
