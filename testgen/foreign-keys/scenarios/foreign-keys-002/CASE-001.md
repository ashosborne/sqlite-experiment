# foreign-keys-002-C001 — run-11 oneshot characterization

Feature: `foreign-keys-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
PRAGMA foreign_keys=ON; CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE CASCADE); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1); DELETE FROM p2; SELECT count(*) FROM c2;
```

## Observables

- `row0.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
