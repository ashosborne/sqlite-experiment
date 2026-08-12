# engine-harvest28-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest28-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: status64 invalid op -> MISUSE(21) (see harness).

## Observables

- `badop_rc` = `21`
