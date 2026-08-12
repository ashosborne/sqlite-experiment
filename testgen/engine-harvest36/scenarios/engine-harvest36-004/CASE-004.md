# engine-harvest36-004-C004 — run-11 oneshot characterization

Feature: `engine-harvest36-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: blob inputs: text flags reject; JSONB flags 4/8 validate bytes (x'1331', x'00' valid; x'' not); text with blob-only flags rejects (see harness).

## Observables

- `blob_flags` = `0,1,1,1,1,0`
- `text_with_blob_flags` = `0,0`
