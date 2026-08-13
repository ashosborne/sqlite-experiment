# engine-harvest37-006-C002 — run-11 oneshot characterization

Feature: `engine-harvest37-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: db_config ENABLE_VIEW: off -> access to view prohibited, on restores (see harness).

## Observables

- `view_get_rc` = `0`
- `view_default` = `1`
- `view_off_select` = `rc=1 err=access to view "vv" prohibited`
- `view_on_select` = `4`
