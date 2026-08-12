# engine-harvest28-002-C003 — run-11 oneshot characterization

Feature: `engine-harvest28-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: status64 MEMORY_USED tracks alloc/free; hi sticky (see harness).

## Observables

- `cur_grew` = `1`
- `hi_ge_cur` = `1`
- `cur_back` = `1`
- `hi_sticky` = `1`
