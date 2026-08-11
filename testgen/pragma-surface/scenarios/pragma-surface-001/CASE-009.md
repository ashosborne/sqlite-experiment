# pragma-surface-001-C009 — run-11 oneshot characterization

Feature: `pragma-surface-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA automatic_index; PRAGMA automatic_index=0; PRAGMA automatic_index;
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `2`
