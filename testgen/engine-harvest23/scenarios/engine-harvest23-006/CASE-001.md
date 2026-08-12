# engine-harvest23-006-C001 — run-11 oneshot characterization

Feature: `engine-harvest23-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_errstr table incl. extended-code fallthrough (see harness).

## Observables

- `e0` = `not an error`
- `e1` = `SQL logic error`
- `e5` = `database is locked`
- `e14` = `unable to open database file`
- `e19` = `constraint failed`
- `e21` = `bad parameter or other API misuse`
- `e23` = `authorization denied`
- `e25` = `column index out of range`
- `e100` = `another row available`
- `e101` = `no more rows available`
- `e787` = `constraint failed`
- `e2067` = `constraint failed`
