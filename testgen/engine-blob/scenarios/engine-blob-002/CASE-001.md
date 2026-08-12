# engine-blob-002-C001 — run-11 oneshot characterization

Feature: `engine-blob-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: full read matches INSERT (see harness).

## Observables

- `read` = `rc=0 err=not an error`
- `data` = `ABCDE`
