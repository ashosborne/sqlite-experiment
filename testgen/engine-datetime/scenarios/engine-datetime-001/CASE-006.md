# engine-datetime-001-C006 — run-11 oneshot characterization

Feature: `engine-datetime-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT date('2026-08-16','weekday 0'), date('2026-08-11','weekday 2');
```

## Observables

- `row0.col0` = `2026-08-16`
- `row0.col1` = `2026-08-11`
- `exec.rc` = `0`
- `cb.rows` = `1`
