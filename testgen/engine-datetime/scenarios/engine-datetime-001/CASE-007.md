# engine-datetime-001-C007 — run-11 oneshot characterization

Feature: `engine-datetime-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT datetime('2024-02-29','+1 year'), date('2026-03-31','-1 month');
```

## Observables

- `row0.col0` = `2025-03-01 00:00:00`
- `row0.col1` = `2026-03-03`
- `exec.rc` = `0`
- `cb.rows` = `1`
