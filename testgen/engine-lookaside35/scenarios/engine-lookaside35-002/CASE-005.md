# engine-lookaside35-002-C005 — run-11 oneshot characterization

Feature: `engine-lookaside35-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: resetFlag: USED highwater pulls to current while live; HIT reset clears (see harness).

## Observables

- `used_reset_hi_eq_cur` = `1`
- `used_live_pos` = `1`
- `hit_reset_zero` = `1`
