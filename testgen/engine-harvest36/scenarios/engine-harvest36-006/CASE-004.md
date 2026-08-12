# engine-harvest36-006-C004 — run-11 oneshot characterization

Feature: `engine-harvest36-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: VACUUM <schema> rebuilds the attached schema; unknown database errors (see harness).

## Observables

- `vacuum_schema` = `rc=0 err=-`
- `aux_intact` = `5`
- `vacuum_badschema` = `rc=1 err=unknown database nosuch`
