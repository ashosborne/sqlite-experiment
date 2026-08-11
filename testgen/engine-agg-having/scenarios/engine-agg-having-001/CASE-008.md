# engine-agg-having-001-C008 — run-11 oneshot characterization

Feature: `engine-agg-having-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT k, total(v) FILTER (WHERE v > 1), count(DISTINCT v) FROM g GROUP BY k HAVING count(*) > 1 ORDER BY k;
```

## Observables

- `row0.col0` = `a`
- `row0.col1` = `2.0`
- `row0.col2` = `2`
- `exec.rc` = `0`
- `cb.rows` = `1`
