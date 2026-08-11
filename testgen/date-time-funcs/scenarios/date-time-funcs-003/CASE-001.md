# date-time-funcs-003-C001 — run-11 oneshot characterization

Feature: `date-time-funcs-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT datetime('2026-01-31','+1 month'), date('2026-08-11','weekday 0');
```

## Observables

- `row0.col0` = `2026-03-03 00:00:00`
- `row0.col1` = `2026-08-16`
- `exec.rc` = `0`
- `cb.rows` = `1`
