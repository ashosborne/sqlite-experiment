# engine-raise-001-C005 — run-11 oneshot characterization

Feature: `engine-raise-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE bt(a INTEGER); INSERT INTO bt VALUES(1),(2); CREATE VIEW bv AS SELECT a FROM bt; CREATE TABLE lg(v INTEGER); CREATE TRIGGER td INSTEAD OF DELETE ON bv BEGIN INSERT INTO lg VALUES(old.a); END; DELETE FROM bv WHERE a = 1; SELECT v FROM lg; SELECT count(*) FROM bt;
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `2`
- `exec.rc` = `0`
- `cb.rows` = `2`
