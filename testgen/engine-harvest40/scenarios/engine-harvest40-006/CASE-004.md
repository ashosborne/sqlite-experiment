# engine-harvest40-006-C004 — run-11 oneshot characterization

Feature: `engine-harvest40-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: RESET_DATABASE + VACUUM empties the schema; the connection stays usable (see harness).

## Observables

- `reset_state0` = `0`
- `reset_state1` = `1`
- `reset_vacuum` = `rc=0 err=-`
- `after_reset` = `0`
- `post_reset_use` = `rc=0 err=-`
- `post_reset_rows` = `3`
