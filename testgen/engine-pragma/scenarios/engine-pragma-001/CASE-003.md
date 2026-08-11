# engine-pragma-001-C003 — run-11 oneshot characterization

Feature: `engine-pragma-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA trusted_schema; PRAGMA trusted_schema=0; PRAGMA trusted_schema; PRAGMA threads; PRAGMA analysis_limit; PRAGMA reverse_unordered_selects=1; PRAGMA reverse_unordered_selects;
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `0`
- `row2.col0` = `0`
- `row3.col0` = `0`
- `row4.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `5`
