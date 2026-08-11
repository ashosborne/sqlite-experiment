# date-time-funcs-001-C001 — run-11 oneshot characterization

Feature: `date-time-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT date('2026-08-11'), datetime(2460000.5), unixepoch('2001-01-01');
```

## Observables

- `row0.col0` = `2026-08-11`
- `row0.col1` = `2023-02-25 00:00:00`
- `row0.col2` = `978307200`
- `exec.rc` = `0`
- `cb.rows` = `1`
