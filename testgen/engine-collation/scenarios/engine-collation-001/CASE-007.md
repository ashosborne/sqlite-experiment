# engine-collation-001-C007 — run-11 oneshot characterization

Feature: `engine-collation-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: eTextRep matrix: UTF8/UTF16 ok, 0 and 99 -> MISUSE (see harness).

## Observables

- `utf8.rc` = `0`
- `zero.rc` = `21`
- `bad99.rc` = `21`
- `utf16.rc` = `0`
