# engine-attach30-002-C002 — run-11 oneshot characterization

Feature: `engine-attach30-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: cannot detach main; detach missing -> no such database (see harness).

## Observables

- `detach_main` = `rc=1 err=cannot detach database main`
- `detach_missing` = `rc=1 err=no such database: nope`
