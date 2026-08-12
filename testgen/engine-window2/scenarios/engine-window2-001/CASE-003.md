# engine-window2-001-C003 — run-11 oneshot characterization

Feature: `engine-window2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE v(x INTEGER); INSERT INTO v VALUES(10),(20),(30); SELECT x, min(x) OVER (ORDER BY x), max(x) OVER (ORDER BY x), avg(x) OVER (ORDER BY x) FROM v ORDER BY x;
```

## Observables

- `row0.col0` = `10`
- `row0.col1` = `10`
- `row0.col2` = `10`
- `row0.col3` = `10.0`
- `row1.col0` = `20`
- `row1.col1` = `10`
- `row1.col2` = `20`
- `row1.col3` = `15.0`
- `row2.col0` = `30`
- `row2.col1` = `10`
- `row2.col2` = `30`
- `row2.col3` = `20.0`
- `exec.rc` = `0`
- `cb.rows` = `3`
