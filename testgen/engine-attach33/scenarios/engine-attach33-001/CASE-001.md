# engine-attach33-001-C001 — run-11 oneshot characterization

Feature: `engine-attach33-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: CREATE TRIGGER aux.trg ON t fires; unqualified body writes aux.log (see harness).

## Observables

- `mktrig` = `rc=0 err=-`
- `fire` = `rc=0 err=-`
- `auxlog` = `41`
