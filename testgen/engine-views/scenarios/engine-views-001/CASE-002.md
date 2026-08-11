# engine-views-001-C002 — run-11 oneshot characterization

Feature: `engine-views-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE vt(a INTEGER); INSERT INTO vt VALUES(3); CREATE VIEW v AS SELECT a*2 AS d FROM vt; SELECT d FROM v; SELECT count(*) FROM sqlite_master WHERE type='view'; DROP VIEW v; SELECT count(*) FROM sqlite_master WHERE type='view';
```

## Observables

- `row0.col0` = `6`
- `row1.col0` = `1`
- `row2.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `3`
