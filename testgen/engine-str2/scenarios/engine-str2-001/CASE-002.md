# engine-str2-001-C002 — run-11 oneshot characterization

Feature: `engine-str2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_str_reset empties the builder; finish -> NULL (see harness).

## Observables

- `len.after.reset` = `0`
- `finish.null` = `1`
- `finish` = `NULL`
