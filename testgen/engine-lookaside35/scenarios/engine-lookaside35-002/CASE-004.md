# engine-lookaside35-002-C004 — run-11 oneshot characterization

Feature: `engine-lookaside35-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: freed slot HITs again on the next prepare (pool reuse) (see harness).

## Observables

- `first_cycle_hit` = `1`
- `slot_reused_hit_again` = `1`
- `used_zero_after_cycles` = `1`
