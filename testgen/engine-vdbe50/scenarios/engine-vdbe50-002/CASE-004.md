# engine-vdbe50-002-C004 — run-11 oneshot characterization

Feature: `engine-vdbe50-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: bound insert: DONE, last rowid 1, bound values land in the row (see harness).

## Observables

- `bound_ins_rc` = `101 last 1`
- `after_bound` = `5|five`
