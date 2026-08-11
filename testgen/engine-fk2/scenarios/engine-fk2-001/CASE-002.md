# engine-fk2-001-C002 — run-11 oneshot characterization

Feature: `engine-fk2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id) ON UPDATE SET NULL); INSERT INTO p VALUES(1); INSERT INTO c VALUES(1); UPDATE p SET id=9 WHERE id=1; SELECT count(*), count(pid) FROM c;
```

## Observables

- `row0.col0` = `1`
- `row0.col1` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
