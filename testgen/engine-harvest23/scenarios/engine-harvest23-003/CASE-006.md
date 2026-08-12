# engine-harvest23-003-C006 — run-11 oneshot characterization

Feature: `engine-harvest23-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: str_append after appendchar; str_length tracking (see harness).

## Observables

- `len` = `6`
- `out` = `===abc`
