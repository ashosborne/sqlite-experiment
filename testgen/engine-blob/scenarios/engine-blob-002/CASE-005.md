# engine-blob-002-C005 — run-11 oneshot characterization

Feature: `engine-blob-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: write on read-only handle -> rc 8 readonly (see harness).

## Observables

- `ro_write` = `rc=8 err=attempt to write a readonly database`
