# engine-agg-having-001-C002 — run-11 oneshot characterization

Feature: `engine-agg-having-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT sum(v), sum(DISTINCT v) FROM g;
```

## Observables

- `row0.col0` = `9`
- `row0.col1` = `8`
- `exec.rc` = `0`
- `cb.rows` = `1`
