# engine-lookaside35-001-C002 — run-11 oneshot characterization

Feature: `engine-lookaside35-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: live prepared stmt: USED positive; reconfig BUSY 5; finalize then rc 0 (see harness).

## Observables

- `used_live_pos` = `1`
- `cfg_busy_rc` = `5`
- `cfg_after_fin_rc` = `0`
