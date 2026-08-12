# engine-harvest28-002-C004 — run-11 oneshot characterization

Feature: `engine-harvest28-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: status64 reset flag re-arms highwater (see harness).

## Observables

- `h1_ge_alloc` = `1`
- `h2_le_h1` = `1`
