# engine-utf16-002-C005 — run-11 oneshot characterization

Feature: `engine-utf16-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: text then text16 then text again (see harness).

## Observables

- `t8` = `abc`
- `t16` = `abc`
- `t8_after` = `abc`
