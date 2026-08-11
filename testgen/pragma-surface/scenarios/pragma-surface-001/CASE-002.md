# pragma-surface-001-C002 — run-11 oneshot characterization

Feature: `pragma-surface-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA application_id; PRAGMA application_id=42; PRAGMA application_id;
```

## Observables

- `row0.col0` = `0`
- `row1.col0` = `42`
- `exec.rc` = `0`
- `cb.rows` = `2`
