# engine-idxlookup-002-C001 — run-11 oneshot characterization

Feature: `engine-idxlookup-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE m(a INTEGER, b INTEGER, v TEXT); CREATE INDEX mab ON m(a, b); INSERT INTO m VALUES(1,1,'x'),(1,2,'y'),(2,1,'z'); SELECT v FROM m WHERE a = 1 AND b = 2; SELECT count(*) FROM pragma_index_list('m');
```

## Observables

- `row0.col0` = `y`
- `row1.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `2`
