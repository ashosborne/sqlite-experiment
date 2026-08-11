# engine-kitchen-001-C002 — run-11 oneshot characterization

Feature: `engine-kitchen-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE k(a INTEGER, b TEXT); INSERT INTO k VALUES(1,'x'); INSERT INTO k VALUES(2,'y'); SELECT a,b FROM k ORDER BY a;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `x`
- `row1.col0` = `2`
- `row1.col1` = `y`
- `exec.rc` = `0`
- `cb.rows` = `2`
