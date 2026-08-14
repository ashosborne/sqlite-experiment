# engine-vdbe49-002-C005 — run-11 oneshot characterization

Feature: `engine-vdbe49-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: the WHERE scan sees a later INSERT (a new matching row appears on the next execution) (see harness).

## Observables

- `pre` = `one/uno`
- `post` = `one/uno/eins`
