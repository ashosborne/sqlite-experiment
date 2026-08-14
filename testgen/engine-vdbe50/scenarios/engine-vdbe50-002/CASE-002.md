# engine-vdbe50-002-C002 — run-11 oneshot characterization

Feature: `engine-vdbe50-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: second insert: rowids 1,2 in order; changes 1, total_changes 2 (see harness).

## Observables

- `after_two` = `1|1|one/2|2|two`
- `changes` = `1 total 2`
