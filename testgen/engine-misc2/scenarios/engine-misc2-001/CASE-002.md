# engine-misc2-001-C002 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE hq(a INTEGER, b TEXT); INSERT INTO hq VALUES(1,'x'),(2,NULL); SELECT sha1_query('SELECT a, b FROM hq');
```

Extension init (static, -DSQLITE_CORE, no load_extension): sha1

## Observables

- `row0.col0` = `0fd31a477035bcab1fa793409ff8ed1d25386c33`
- `exec.rc` = `0`
- `cb.rows` = `1`
