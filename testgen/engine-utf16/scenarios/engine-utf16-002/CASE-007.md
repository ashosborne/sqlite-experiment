# engine-utf16-002-C007 — run-11 oneshot characterization

Feature: `engine-utf16-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: empty text16 vs NULL distinct (see harness).

## Observables

- `empty` = ``
- `empty.b16` = `0`
- `null` = `NULL`
- `null.b16` = `0`
