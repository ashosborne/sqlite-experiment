# engine-datetime-001-C004 — run-11 oneshot characterization

Feature: `engine-datetime-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT strftime('%H:%M:%S','2026-08-11 04:05:06'), strftime('%s','2001-01-01'), strftime('%w','2026-08-11');
```

## Observables

- `row0.col0` = `04:05:06`
- `row0.col1` = `978307200`
- `row0.col2` = `2`
- `exec.rc` = `0`
- `cb.rows` = `1`
