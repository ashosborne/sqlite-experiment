# engine-window2-001-C004 — run-11 oneshot characterization

Feature: `engine-window2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE p(g TEXT, x INTEGER); INSERT INTO p VALUES('a',1),('a',2),('b',5),('b',7); SELECT g, x, sum(x) OVER (PARTITION BY g ORDER BY x) FROM p ORDER BY g, x;
```

## Observables

- `row0.col0` = `a`
- `row0.col1` = `1`
- `row0.col2` = `1`
- `row1.col0` = `a`
- `row1.col1` = `2`
- `row1.col2` = `3`
- `row2.col0` = `b`
- `row2.col1` = `5`
- `row2.col2` = `5`
- `row3.col0` = `b`
- `row3.col1` = `7`
- `row3.col2` = `12`
- `exec.rc` = `0`
- `cb.rows` = `4`
