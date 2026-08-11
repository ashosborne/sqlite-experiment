# engine-pragma-001-C002 — run-11 oneshot characterization

Feature: `engine-pragma-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA encoding; PRAGMA page_size; PRAGMA journal_mode; PRAGMA synchronous; PRAGMA locking_mode; PRAGMA read_uncommitted;
```

## Observables

- `row0.col0` = `UTF-8`
- `row1.col0` = `4096`
- `row2.col0` = `memory`
- `row3.col0` = `2`
- `row4.col0` = `normal`
- `row5.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `6`
