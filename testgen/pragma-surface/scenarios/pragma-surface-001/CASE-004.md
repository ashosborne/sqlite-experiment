# pragma-surface-001-C004 — run-11 oneshot characterization

Feature: `pragma-surface-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA cache_size; PRAGMA cache_size=100; PRAGMA cache_size;
```

## Observables

- `row0.col0` = `-2000`
- `row1.col0` = `100`
- `exec.rc` = `0`
- `cb.rows` = `2`
