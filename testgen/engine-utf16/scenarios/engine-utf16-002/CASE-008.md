# engine-utf16-002-C008 — run-11 oneshot characterization

Feature: `engine-utf16-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: UTF-8-stored non-ASCII read as text16 (byte lengths) (see harness).

## Observables

- `a` = `café`
- `a.b16` = `8`
- `b` = `😀`
- `b.b16` = `4`
