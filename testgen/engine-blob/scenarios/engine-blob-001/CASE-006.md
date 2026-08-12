# engine-blob-001-C006 — run-11 oneshot characterization

Feature: `engine-blob-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: RW open refused on indexed column; RO allowed (see harness).

## Observables

- `rw_indexed` = `rc=1 err=cannot open indexed column for writing`
- `ro_indexed` = `rc=0 err=not an error`
