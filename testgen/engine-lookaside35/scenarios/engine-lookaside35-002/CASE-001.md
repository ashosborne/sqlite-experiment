# engine-lookaside35-002-C001 — run-11 oneshot characterization

Feature: `engine-lookaside35-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: prepare/finalize traffic grows HIT (cur always 0); USED drains to 0 (see harness).

## Observables

- `hit_grew` = `1`
- `hit_cur_always_zero` = `1`
- `used_back_to_zero` = `1`
