# engine-harvest37-006-C001 — run-11 oneshot characterization

Feature: `engine-harvest37-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: db_config ENABLE_TRIGGER: default 1, off suppresses firing, on restores (see harness).

## Observables

- `trigger_get_rc` = `0`
- `trigger_default` = `1`
- `trigger_after_off` = `0`
- `trig_off_log` = `0`
- `trig_on_log` = `1`
