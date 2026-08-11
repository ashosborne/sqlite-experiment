# engine-join-002-C002 — run-11 oneshot characterization

Feature: `engine-join-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE n(a INTEGER); INSERT INTO n VALUES(1),(2),(3); SELECT p.a, q.a FROM n p, n q WHERE q.a = p.a + 1 ORDER BY p.a;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `2`
- `row1.col0` = `2`
- `row1.col1` = `3`
- `exec.rc` = `0`
- `cb.rows` = `2`
