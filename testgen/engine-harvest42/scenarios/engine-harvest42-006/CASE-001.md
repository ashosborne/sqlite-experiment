# engine-harvest42-006-C001 — run-11 oneshot characterization

Feature: `engine-harvest42-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: seed column count must match the declaration: table NAME has N values for M columns (recursive and plain WITH) (see harness).

## Observables

- `e_colcount` = `prep.rc=1 err=table c has 1 values for 2 columns LOG=`
- `e_seedcols` = `prep.rc=1 err=table x has 1 values for 2 columns LOG=`
