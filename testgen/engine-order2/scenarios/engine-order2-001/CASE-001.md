# engine-order2-001-C001 — run-11 oneshot characterization

Feature: `engine-order2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE o(t TEXT); INSERT INTO o VALUES('b'),('A'),('c'),(NULL); SELECT t FROM o ORDER BY t COLLATE nocase NULLS LAST;
```

## Observables

- `row0.col0` = `A`
- `row1.col0` = `b`
- `row2.col0` = `c`
- `row3.col0` = `NULL`
- `exec.rc` = `0`
- `cb.rows` = `4`
