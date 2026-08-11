# engine-join-002-C007 — run-11 oneshot characterization

Feature: `engine-join-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('b',2),('a',3); SELECT k, sum(v), count(*) FROM g GROUP BY k ORDER BY k;
```

## Observables

- `row0.col0` = `a`
- `row0.col1` = `4`
- `row0.col2` = `2`
- `row1.col0` = `b`
- `row1.col1` = `2`
- `row1.col2` = `1`
- `exec.rc` = `0`
- `cb.rows` = `2`
