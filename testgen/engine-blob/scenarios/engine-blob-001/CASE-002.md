# engine-blob-001-C002 — run-11 oneshot characterization

Feature: `engine-blob-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: open read-write (flags=1) (see harness).

## Observables

- `open_rw` = `rc=0 err=not an error`
- `bytes` = `5`
- `close` = `0`
