# engine-harvest37-001-C006 — run-11 oneshot characterization

Feature: `engine-harvest37-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: urifuncs SQL surface ABSENT on the bare pin (no such function: uri_parameter) (see harness).

## Observables

- `sql_urifunc_absent` = `rc=1 err=no such function: uri_parameter`
