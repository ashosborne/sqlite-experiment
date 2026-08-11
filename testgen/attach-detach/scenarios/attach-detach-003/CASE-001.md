# attach-detach-003-C001 — run-11 oneshot characterization

Feature: `attach-detach-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
ATTACH ':memory:' AS aux3; CREATE TABLE main.mm(v); CREATE TABLE aux3.t(a); CREATE TRIGGER aux3.trg AFTER INSERT ON aux3.t BEGIN INSERT INTO main.mm VALUES(1); END;
```

## Observables

- `exec.rc` = `1`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
