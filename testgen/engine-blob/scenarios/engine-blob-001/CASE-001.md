# engine-blob-001-C001 — run-11 oneshot characterization

Feature: `engine-blob-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: open read-only; bytes; close (see harness).

## Observables

- `open` = `rc=0 err=not an error`
- `bytes` = `5`
- `close` = `0`
