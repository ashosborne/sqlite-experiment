# engine-attach33-001-C003 — run-11 oneshot characterization

Feature: `engine-attach33-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: unqualified trigger name lives in main -> cannot reference aux.t (see harness).

## Observables

- `mktrig` = `rc=1 err=trigger trg cannot reference objects in database aux`
- `fire` = `rc=0 err=-`
- `auxlog` = `0`
