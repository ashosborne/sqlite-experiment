# engine-subquery-002-C005 — run-11 oneshot characterization

Feature: `engine-subquery-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE f(v INTEGER); INSERT INTO f VALUES(900017); SELECT (SELECT v FROM f), (SELECT v+1 FROM f), (SELECT v FROM f WHERE v=0);
```

## Observables

- `row0.col0` = `900017`
- `row0.col1` = `900018`
- `row0.col2` = `NULL`
- `exec.rc` = `0`
- `cb.rows` = `1`
