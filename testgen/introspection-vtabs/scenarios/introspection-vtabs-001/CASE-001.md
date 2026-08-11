# introspection-vtabs-001-C001 — run-11 oneshot characterization

Feature: `introspection-vtabs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE iv(a); INSERT INTO iv VALUES(1); SELECT count(*)>0 FROM dbstat;
```

## Observables

- `exec.rc` = `1`
- `cb.rows` = `0`
