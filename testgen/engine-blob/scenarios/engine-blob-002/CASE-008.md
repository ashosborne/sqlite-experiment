# engine-blob-002-C008 — run-11 oneshot characterization

Feature: `engine-blob-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: expiry on UPDATE: rc 4 query aborted, bytes -> 0 (see harness).

## Observables

- `upd` = `rc=0 err=-`
- `read_after` = `rc=4 err=query aborted`
- `write_after` = `rc=4 err=query aborted`
- `bytes_after` = `0`
