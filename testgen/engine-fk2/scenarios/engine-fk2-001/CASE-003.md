# engine-fk2-001-C003 — run-11 oneshot characterization

Feature: `engine-fk2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid INTEGER DEFAULT 7 REFERENCES p(id) ON DELETE SET DEFAULT); INSERT INTO p VALUES(1),(7); INSERT INTO c VALUES(1); DELETE FROM p WHERE id=1; SELECT pid FROM c;
```

## Observables

- `row0.col0` = `7`
- `exec.rc` = `0`
- `cb.rows` = `1`
