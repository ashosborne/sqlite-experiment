# engine-lookaside35-002-C002 — run-11 oneshot characterization

Feature: `engine-lookaside35-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: 64-byte slots force MISS_SIZE growth (cur 0) (see harness).

## Observables

- `miss_size_grew` = `1`
- `miss_size_cur_zero` = `1`
