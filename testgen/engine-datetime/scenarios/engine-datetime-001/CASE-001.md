# engine-datetime-001-C001 — run-11 oneshot characterization

Feature: `engine-datetime-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT time('2026-08-11 04:05:06'), julianday('2000-01-01 12:00:00');
```

## Observables

- `row0.col0` = `04:05:06`
- `row0.col1` = `2451545.0`
- `exec.rc` = `0`
- `cb.rows` = `1`
