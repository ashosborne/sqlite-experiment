# engine-attach33-004-C001 — run-11 oneshot characterization

Feature: `engine-attach33-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DETACH tears the attached trigger down; re-ATTACH is fresh (see harness).

## Observables

- `gone` = `rc=1 err=no such table: aux.t`
- `fresh` = `0`
