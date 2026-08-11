# engine-misc2-001-C012 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT printf('%w','my"id'), printf('%#x|%#o', 255, 8);
```

## Observables

- `row0.col0` = `my""id`
- `row0.col1` = `0xff|010`
- `exec.rc` = `0`
- `cb.rows` = `1`
