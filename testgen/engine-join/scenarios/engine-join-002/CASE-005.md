# engine-join-002-C005 — run-11 oneshot characterization

Feature: `engine-join-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE l(a INTEGER); INSERT INTO l VALUES(1),(2),(3); CREATE TABLE r(a INTEGER, t TEXT); INSERT INTO r VALUES(1,'one'),(3,'three'); SELECT l.a, r.t FROM l LEFT JOIN r ON l.a = r.a ORDER BY l.a;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `one`
- `row1.col0` = `2`
- `row1.col1` = `NULL`
- `row2.col0` = `3`
- `row2.col1` = `three`
- `exec.rc` = `0`
- `cb.rows` = `3`
