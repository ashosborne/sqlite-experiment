# engine-join-002-C001 — run-11 oneshot characterization

Feature: `engine-join-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE a1(x INTEGER); INSERT INTO a1 VALUES(1),(2); CREATE TABLE b1(x INTEGER, y INTEGER); INSERT INTO b1 VALUES(1,10),(2,20); CREATE TABLE c1(y INTEGER, z TEXT); INSERT INTO c1 VALUES(10,'ten'),(20,'twenty'); SELECT a1.x, c1.z FROM a1 JOIN b1 ON a1.x = b1.x JOIN c1 ON b1.y = c1.y ORDER BY a1.x;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `ten`
- `row1.col0` = `2`
- `row1.col1` = `twenty`
- `exec.rc` = `0`
- `cb.rows` = `2`
