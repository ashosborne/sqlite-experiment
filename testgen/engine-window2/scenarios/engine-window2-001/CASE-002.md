# engine-window2-001-C002 — run-11 oneshot characterization

Feature: `engine-window2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE v(x INTEGER); INSERT INTO v VALUES(1),(2),(3); SELECT x, lag(x) OVER (ORDER BY x), lead(x) OVER (ORDER BY x) FROM v ORDER BY x;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `NULL`
- `row0.col2` = `2`
- `row1.col0` = `2`
- `row1.col1` = `1`
- `row1.col2` = `3`
- `row2.col0` = `3`
- `row2.col1` = `2`
- `row2.col2` = `NULL`
- `exec.rc` = `0`
- `cb.rows` = `3`
