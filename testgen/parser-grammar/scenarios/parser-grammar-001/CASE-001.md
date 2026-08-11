# parser-grammar-001-C001 — run-11 oneshot characterization

Feature: `parser-grammar-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE p1(key TEXT, abort INT); INSERT INTO p1 VALUES('k',1); SELECT key, abort FROM p1;
```

## Observables

- `row0.col0` = `k`
- `row0.col1` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
