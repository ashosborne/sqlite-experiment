# engine-trig2-001-C001 — run-11 oneshot characterization

Feature: `engine-trig2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE bt(a INTEGER); CREATE VIEW bv AS SELECT a FROM bt; CREATE TRIGGER ti INSTEAD OF INSERT ON bv BEGIN INSERT INTO bt VALUES(new.a * 10); END; INSERT INTO bv VALUES(4); SELECT a FROM bt;
```

## Observables

- `row0.col0` = `40`
- `exec.rc` = `0`
- `cb.rows` = `1`
