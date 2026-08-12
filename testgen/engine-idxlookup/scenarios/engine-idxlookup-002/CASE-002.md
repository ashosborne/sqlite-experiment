# engine-idxlookup-002-C002 — run-11 oneshot characterization

Feature: `engine-idxlookup-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE nx(nm TEXT); CREATE INDEX ie ON nx(lower(nm)); INSERT INTO nx VALUES('Ann'),('BOB'); SELECT count(*) FROM sqlite_master WHERE type='index'; SELECT nm FROM nx WHERE lower(nm) = 'bob';
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `BOB`
- `exec.rc` = `0`
- `cb.rows` = `2`
