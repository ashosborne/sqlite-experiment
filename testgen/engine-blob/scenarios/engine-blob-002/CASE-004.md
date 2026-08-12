# engine-blob-002-C004 — run-11 oneshot characterization

Feature: `engine-blob-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: write at offset; SELECT sees bytes; length unchanged (see harness).

## Observables

- `write` = `rc=0 err=not an error`
- `bytes` = `5`
- `cell` = `4178794141,5`
