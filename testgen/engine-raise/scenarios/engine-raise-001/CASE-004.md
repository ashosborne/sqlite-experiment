# engine-raise-001-C004 — run-11 oneshot characterization

Feature: `engine-raise-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE bt(a INTEGER, b TEXT); INSERT INTO bt VALUES(1,'x'); CREATE VIEW bv AS SELECT a, b FROM bt; CREATE TABLE lg(o INTEGER, n INTEGER); CREATE TRIGGER tu INSTEAD OF UPDATE ON bv BEGIN INSERT INTO lg VALUES(old.a, new.a); END; UPDATE bv SET a = 9 WHERE a = 1; SELECT o, n FROM lg; SELECT a FROM bt;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `9`
- `row1.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `2`
