# engine-funcs-001-C001 — run-11 oneshot characterization

Feature: `engine-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT printf('%5d|%-4d|%05.1f|%x|%X', 42, 7, 3.14159, 255, 255), printf('%.3s|%c|%o', 'hello', 65, 8);
```

## Observables

- `row0.col0` = `   42|7   |003.1|ff|FF`
- `row0.col1` = `hel|6|10`
- `exec.rc` = `0`
- `cb.rows` = `1`
