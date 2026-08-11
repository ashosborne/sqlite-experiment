# printf-format-001-C001 — run-11 oneshot characterization

Feature: `printf-format-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT printf('%d|%s|%q', 7, 'a', 'a''b');
```

## Observables

- `row0.col0` = `7|a|a''b`
- `exec.rc` = `0`
- `cb.rows` = `1`
