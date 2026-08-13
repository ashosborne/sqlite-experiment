# engine-harvest42-006-C002 — run-11 oneshot characterization

Feature: `engine-harvest42-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: circular reference: NAME for mutually-referencing CTEs; multiple references to recursive table: NAME; recursive aggregate queries not supported (see harness).

## Observables

- `e_circular` = `prep.rc=1 err=circular reference: a LOG=`
- `e_multi_rec` = `prep.rc=1 err=multiple references to recursive table: c LOG=`
- `e_agg` = `prep.rc=1 err=recursive aggregate queries not supported LOG=`
