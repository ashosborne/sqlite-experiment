# engine-attach30-002-C001 — run-11 oneshot characterization

Feature: `engine-attach30-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DETACH removes schema + its objects; qualified select fails (see harness).

## Observables

- `detach` = `rc=0 err=-`
- `dblist` = `main`
- `gone` = `prep.rc=1 err=no such table: aux.t`
