# engine-views-001-C001 — run-11 oneshot characterization

Feature: `engine-views-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE vt(a INTEGER, b TEXT); INSERT INTO vt VALUES(1,'x'),(2,'y'); CREATE VIEW v AS SELECT a, b FROM vt WHERE a > 1; SELECT a, b FROM v;
```

## Observables

- `row0.col0` = `2`
- `row0.col1` = `y`
- `exec.rc` = `0`
- `cb.rows` = `1`
