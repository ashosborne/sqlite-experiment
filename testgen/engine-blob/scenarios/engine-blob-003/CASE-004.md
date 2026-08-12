# engine-blob-003-C004 — run-11 oneshot characterization

Feature: `engine-blob-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: WAL file: handle I/O works; journal mode untouched (see harness).

## Observables

- `write` = `rc=0 err=not an error`
- `mode` = `wal`
- `reopen` = `WALb`
