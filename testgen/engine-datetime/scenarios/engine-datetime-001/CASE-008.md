# engine-datetime-001-C008 — run-11 oneshot characterization

Feature: `engine-datetime-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT timediff('2026-08-11 10:30:00','2026-08-11 09:15:30'), timediff('2025-08-10','2026-08-11');
```

## Observables

- `row0.col0` = `+0000-00-00 01:14:30.000`
- `row0.col1` = `-0001-00-01 00:00:00.000`
- `exec.rc` = `0`
- `cb.rows` = `1`
