# engine-setops-001-C002 — run-11 oneshot characterization

Feature: `engine-setops-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE s1(a); INSERT INTO s1 VALUES(1),(2),(3); CREATE TABLE s2(a); INSERT INTO s2 VALUES(2),(3),(4); SELECT a FROM s1 EXCEPT SELECT a FROM s2 ORDER BY a;
```

## Observables

- `row0.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
