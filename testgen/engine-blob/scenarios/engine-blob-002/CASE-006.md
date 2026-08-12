# engine-blob-002-C006 — run-11 oneshot characterization

Feature: `engine-blob-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: write past end -> SQL logic error; cell unchanged (see harness).

## Observables

- `too_long` = `rc=1 err=SQL logic error`
- `at_end` = `rc=1 err=SQL logic error`
- `cell` = `4141`
