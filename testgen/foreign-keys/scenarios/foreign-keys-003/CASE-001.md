# foreign-keys-003-C001 — run-11 oneshot characterization

Feature: `foreign-keys-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA foreign_keys=ON; CREATE TABLE p3(id INTEGER PRIMARY KEY); CREATE TABLE c3(pid REFERENCES p3(id)); INSERT INTO p3 VALUES(1); INSERT INTO c3 VALUES(1); DROP TABLE p3;
```

## Observables

- `exec.rc` = `19`
- `cb.rows` = `0`
- `errmsg.nonempty` = `1`
