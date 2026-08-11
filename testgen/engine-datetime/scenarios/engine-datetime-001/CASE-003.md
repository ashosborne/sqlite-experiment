# engine-datetime-001-C003 — run-11 oneshot characterization

Feature: `engine-datetime-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT date('2026-08-11','-40 days'), datetime('2026-08-11 04:05:06','+2 hours','+30 minutes','+15 seconds');
```

## Observables

- `row0.col0` = `2026-07-02`
- `row0.col1` = `2026-08-11 06:35:21`
- `exec.rc` = `0`
- `cb.rows` = `1`
