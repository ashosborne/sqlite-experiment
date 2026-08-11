# engine-join-001-C006 — run-11 oneshot characterization

Feature: `engine-join-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE n(a INTEGER); INSERT INTO n VALUES(1),(2),(3); SELECT p.a, q.a FROM n p JOIN n q ON q.a > p.a ORDER BY p.a, q.a;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `2`
- `row1.col0` = `1`
- `row1.col1` = `3`
- `row2.col0` = `2`
- `row2.col1` = `3`
- `exec.rc` = `0`
- `cb.rows` = `3`
