# engine-idxlookup-001-C006 — run-11 oneshot characterization

Feature: `engine-idxlookup-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE e(id INTEGER, nm TEXT); CREATE INDEX ei ON e(id); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'); SELECT e.nm, d.dept FROM e JOIN d ON e.id = d.eid WHERE e.id = 2;
```

## Observables

- `row0.col0` = `bob`
- `row0.col1` = `ops`
- `exec.rc` = `0`
- `cb.rows` = `1`
