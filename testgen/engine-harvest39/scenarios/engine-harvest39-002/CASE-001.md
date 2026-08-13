# engine-harvest39-002-C001 — run-11 oneshot characterization

Feature: `engine-harvest39-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: fresh connection: pragma_module_list lists only its own lazily-created pragma vtab (see harness).

## Observables

- `fresh` = `pragma_module_list`
