# engine-harvest40-006-C002 — run-11 oneshot characterization

Feature: `engine-harvest40-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: WRITABLE_SCHEMA gates sqlite_master UPDATE; DEFENSIVE makes PRAGMA writable_schema=ON a no-op and keeps the gate closed (see harness).

## Observables

- `ws_off` = `rc=1 err=table sqlite_master may not be modified`
- `ws_on` = `rc=0 err=-`
- `ws_state` = `1`
- `def_pragma_ws` = `rc=0 err=-`
- `def_ws_state` = `0`
- `def_update` = `rc=1 err=table sqlite_master may not be modified`
- `def_state` = `0`
- `off_pragma_ws` = `rc=0 err=-`
- `off_update` = `rc=0 err=-`
