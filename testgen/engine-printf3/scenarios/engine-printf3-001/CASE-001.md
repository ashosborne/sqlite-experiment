# engine-printf3-001-C001 — run-11 oneshot characterization

Feature: `engine-printf3-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT printf('%,d', 1234567), printf('%,d', -1234);
```

## Observables

- `row0.col0` = `1,234,567`
- `row0.col1` = `-1,234`
- `exec.rc` = `0`
- `cb.rows` = `1`
