# pragma-surface-001-C010 — run-11 oneshot characterization

Feature: `pragma-surface-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA ignore_check_constraints; PRAGMA ignore_check_constraints=1; PRAGMA ignore_check_constraints;
```

## Observables

- `row0.col0` = `0`
- `row1.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `2`
