# engine-analyze-002-C001 — run-11 oneshot characterization

Feature: `engine-analyze-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: WAL file: ANALYZE + reopen, mode untouched (see harness).

## Observables

- `analyze` = `rc=0 err=-`
- `mode` = `wal`
- `reopen` = `t,ia,2 1`
