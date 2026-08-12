# engine-str2-001-C001 — run-11 oneshot characterization

Feature: `engine-str2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_str_appendchar + length + value + finish (see harness).

## Observables

- `len` = `5`
- `val` = `xxx-7`
- `finish` = `xxx-7`
