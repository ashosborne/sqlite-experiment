# engine-pragma-001-C001 — run-11 oneshot characterization

Feature: `engine-pragma-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA foreign_keys; PRAGMA foreign_keys=ON; PRAGMA foreign_keys; PRAGMA busy_timeout; PRAGMA busy_timeout=250; PRAGMA busy_timeout;
```

## Observables

- `row0.col0` = `0`
- `row1.col0` = `1`
- `row2.col0` = `0`
- `row3.col0` = `250`
- `row4.col0` = `250`
- `exec.rc` = `0`
- `cb.rows` = `5`
