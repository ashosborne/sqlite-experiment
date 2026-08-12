# engine-harvest36-003-C002 — run-11 oneshot characterization

Feature: `engine-harvest36-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DETACH with only a main-table statement active succeeds (see harness).

## Observables

- `detach_main_stmt_ok` = `rc=0 err=-`
