# engine-compile32-001-C001 — run-11 oneshot characterization

Feature: `engine-compile32-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: used(): plain / SQLITE_-prefixed / =value forms; wrong value -> 0 (see harness).

## Observables

- `plain` = `used=1`
- `prefixed` = `used=1`
- `valued` = `used=1`
- `wrong_value` = `used=0`
