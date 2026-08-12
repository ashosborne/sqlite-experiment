# engine-attach33-002-C005 — run-11 oneshot characterization

Feature: `engine-attach33-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: body target only in main: no fallback, fire errors no such table: aux.log (see harness).

## Observables

- `mktrig` = `rc=0 err=-`
- `fire` = `rc=1 err=no such table: aux.log`
- `mainlog` = `0`
