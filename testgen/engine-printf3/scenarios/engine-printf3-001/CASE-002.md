# engine-printf3-001-C002 — run-11 oneshot characterization

Feature: `engine-printf3-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT printf('%p', 123), length(printf('%p', 123)) > 0;
```

## Observables

- `row0.col0` = `7B`
- `row0.col1` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
