# engine-harvest28-002-C001 — run-11 oneshot characterization

Feature: `engine-harvest28-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: status64 MEMORY_USED cur<=hi, non-negative (see harness).

## Observables

- `rc` = `0`
- `cur_le_hi` = `1`
- `cur_nonneg` = `1`
