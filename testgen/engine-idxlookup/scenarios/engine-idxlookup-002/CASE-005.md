# engine-idxlookup-002-C005 — run-11 oneshot characterization

Feature: `engine-idxlookup-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE dx(nm TEXT); CREATE INDEX de ON dx(lower(nm)); SELECT count(*) FROM sqlite_master WHERE type='index'; DROP INDEX de; SELECT count(*) FROM sqlite_master WHERE type='index';
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `2`
