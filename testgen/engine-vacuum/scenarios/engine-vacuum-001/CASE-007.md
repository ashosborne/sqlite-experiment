# engine-vacuum-001-C007 — run-11 oneshot characterization

Feature: `engine-vacuum-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: INTEGER PRIMARY KEY ids preserved across VACUUM (see harness).

## Observables

- `vac` = `rc=0 err=-`
- `ids` = `10,x|30,z`
- `rowids` = `10|30`
