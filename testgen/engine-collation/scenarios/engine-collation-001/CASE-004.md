# engine-collation-001-C004 — run-11 oneshot characterization

Feature: `engine-collation-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: delete by NULL xCompare; later use -> no such collation (see harness).

## Observables

- `present` = `1`
- `del.rc` = `0`
- `deleted` = `prep.rc=1 err=no such collation sequence: gone`
