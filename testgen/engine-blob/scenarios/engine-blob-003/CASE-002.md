# engine-blob-003-C002 — run-11 oneshot characterization

Feature: `engine-blob-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: blob_read on a reopened file (see harness).

## Observables

- `read` = `rc=0 err=not an error`
- `data` = `HELLO!`
