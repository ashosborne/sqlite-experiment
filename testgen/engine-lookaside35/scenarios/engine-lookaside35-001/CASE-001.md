# engine-lookaside35-001-C001 — run-11 oneshot characterization

Feature: `engine-lookaside35-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: fresh connection: USED/HIT zero; reconfig on a quiet connection rc 0 (see harness).

## Observables

- `used_fresh_zero` = `1`
- `hit_fresh_zero` = `1`
- `cfg_fresh_rc` = `0`
