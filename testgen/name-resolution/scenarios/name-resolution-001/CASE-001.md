# name-resolution-001-C001 — run-11 oneshot characterization

Feature: `name-resolution-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE n1(a); INSERT INTO n1 VALUES(5); SELECT n1.a, a, rowid FROM n1;
```

## Observables

- `row0.col0` = `5`
- `row0.col1` = `5`
- `row0.col2` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
