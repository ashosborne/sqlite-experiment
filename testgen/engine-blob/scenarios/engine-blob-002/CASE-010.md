# engine-blob-002-C010 — run-11 oneshot characterization

Feature: `engine-blob-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: zero-length blob: bytes 0, n=0 read ok, n=1 read errors (see harness).

## Observables

- `open` = `rc=0 err=not an error`
- `bytes` = `0`
- `read0` = `rc=0 err=not an error`
- `read1` = `rc=1 err=SQL logic error`
