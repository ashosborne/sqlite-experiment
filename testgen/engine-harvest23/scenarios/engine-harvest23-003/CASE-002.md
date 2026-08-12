# engine-harvest23-003-C002 — run-11 oneshot characterization

Feature: `engine-harvest23-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: snprintf truncation at n (NUL at n-1) (see harness).

## Observables

- `out` = `1234`
- `nul_at_4` = `1`
