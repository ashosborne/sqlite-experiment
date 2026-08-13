# engine-vtab38-003-C001 — run-11 oneshot characterization

Feature: `engine-vtab38-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: _v2 destructor DEFERS while a live instance holds the module: replace 0, DROP 1, close 2 (see harness).

## Observables

- `mdest_after_replace_with_live` = `0`
- `mdest_after_drop` = `1`
- `mdest_after_close` = `2`
