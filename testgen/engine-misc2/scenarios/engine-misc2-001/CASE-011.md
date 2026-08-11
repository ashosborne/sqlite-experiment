# engine-misc2-001-C011 — run-11 oneshot characterization

Feature: `engine-misc2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT uuid_str('{A0EEBC99-9C0B-4EF8-BB6D-6BB9BD380A11}'), hex(uuid_blob('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'));
```

Extension init (static, -DSQLITE_CORE, no load_extension): uuid

## Observables

- `row0.col0` = `a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11`
- `row0.col1` = `A0EEBC999C0B4EF8BB6D6BB9BD380A11`
- `exec.rc` = `0`
- `cb.rows` = `1`
