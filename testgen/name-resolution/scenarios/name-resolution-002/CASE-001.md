# name-resolution-002-C001 — run-11 oneshot characterization

Feature: `name-resolution-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT 3 AS k UNION ALL SELECT 1 ORDER BY k;
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `3`
- `exec.rc` = `0`
- `cb.rows` = `2`
