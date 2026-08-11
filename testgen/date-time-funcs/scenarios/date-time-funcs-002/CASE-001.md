# date-time-funcs-002-C001 — run-11 oneshot characterization

Feature: `date-time-funcs-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT strftime('%Y|%m|%d|%j','2026-08-11');
```

## Observables

- `row0.col0` = `2026|08|11|223`
- `exec.rc` = `0`
- `cb.rows` = `1`
