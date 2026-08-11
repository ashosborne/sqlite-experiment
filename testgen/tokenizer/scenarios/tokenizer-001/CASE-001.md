# tokenizer-001-C001 — run-11 oneshot characterization

Feature: `tokenizer-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT 0x10, 1e2, X'41', [a] FROM (SELECT 1 AS a);
```

## Observables

- `row0.col0` = `16`
- `row0.col1` = `100.0`
- `row0.col2` = `A`
- `row0.col3` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
