# engine-datetime-001-C005 — run-11 oneshot characterization

Feature: `engine-datetime-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT datetime(978307200,'unixepoch'), unixepoch('2026-08-11 00:00:00');
```

## Observables

- `row0.col0` = `2001-01-01 00:00:00`
- `row0.col1` = `1786406400`
- `exec.rc` = `0`
- `cb.rows` = `1`
