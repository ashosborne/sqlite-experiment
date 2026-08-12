# engine-vtab31-001-C007 — run-11 oneshot characterization

Feature: `engine-vtab31-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: fresh connection has no module -> re-register required (see harness).

## Observables

- `fresh_cvt` = `rc=1 err=no such module: intseries`
