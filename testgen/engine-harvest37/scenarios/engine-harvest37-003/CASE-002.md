# engine-harvest37-003-C002 — run-11 oneshot characterization

Feature: `engine-harvest37-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: source write between steps restarts the copy; dest sees the late write (see harness).

## Observables

- `restart_step_rc` = `0`
- `restart_rem_is_pc_minus_1` = `1`
- `finish_rc` = `0`
- `dst_sees_late_write` = `201`
