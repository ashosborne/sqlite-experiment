# engine-harvest36-003-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DETACH while a statement is active on the attached schema -> database aux is locked (see harness).

## Observables

- `detach_locked` = `rc=1 err=database aux is locked`
- `detach_after_fin` = `rc=0 err=-`
