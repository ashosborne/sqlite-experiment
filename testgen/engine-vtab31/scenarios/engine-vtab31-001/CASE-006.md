# engine-vtab31-001-C006 — run-11 oneshot characterization

Feature: `engine-vtab31-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: create_module_v2 destructor on replace and on close (see harness).

## Observables

- `after_reg` = `n=0`
- `after_replace` = `n=1`
- `after_close` = `n=2`
