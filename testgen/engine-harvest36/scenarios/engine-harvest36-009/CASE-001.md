# engine-harvest36-009-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-009` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_vtab_config: OK inside xCreate, MISUSE for bad op and outside a constructor (see harness).

## Observables

- `cvt` = `rc=0 err=-`
- `vtab_config_rc` = `0`
- `vtab_config_badop_rc` = `21`
- `vtab_config_outside_rc` = `21`
