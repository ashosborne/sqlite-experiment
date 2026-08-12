# engine-window2-001-C001 — run-11 oneshot characterization

Feature: `engine-window2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE w(x INTEGER); INSERT INTO w VALUES(10),(20),(20),(30); SELECT x, rank() OVER (ORDER BY x), dense_rank() OVER (ORDER BY x) FROM w ORDER BY x;
```

## Observables

- `row0.col0` = `10`
- `row0.col1` = `1`
- `row0.col2` = `1`
- `row1.col0` = `20`
- `row1.col1` = `2`
- `row1.col2` = `2`
- `row2.col0` = `20`
- `row2.col1` = `2`
- `row2.col2` = `2`
- `row3.col0` = `30`
- `row3.col1` = `4`
- `row3.col2` = `3`
- `exec.rc` = `0`
- `cb.rows` = `4`
