# engine-attach30-001-C004 — run-11 oneshot characterization

Feature: `engine-attach30-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: duplicate schema + reserved main/temp -> already in use (see harness).

## Observables

- `dup` = `rc=1 err=database aux is already in use`
- `resv_main` = `rc=1 err=database main is already in use`
- `resv_temp` = `rc=1 err=database temp is already in use`
