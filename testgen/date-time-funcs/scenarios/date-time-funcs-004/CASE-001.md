# date-time-funcs-004-C001 — run-11 oneshot characterization

Feature: `date-time-funcs-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT timediff('2026-08-11','2025-08-10');
```

## Observables

- `row0.col0` = `+0001-00-01 00:00:00.000`
- `exec.rc` = `0`
- `cb.rows` = `1`
