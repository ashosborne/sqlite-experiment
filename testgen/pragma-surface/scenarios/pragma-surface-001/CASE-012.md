# pragma-surface-001-C012 — run-11 oneshot characterization

Feature: `pragma-surface-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE ic(a CHECK(a>0)); PRAGMA integrity_check; PRAGMA quick_check;
```

## Observables

- `row0.col0` = `ok`
- `row1.col0` = `ok`
- `exec.rc` = `0`
- `cb.rows` = `2`
