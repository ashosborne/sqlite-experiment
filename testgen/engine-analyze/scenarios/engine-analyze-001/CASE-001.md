# engine-analyze-001-C001 — run-11 oneshot characterization

Feature: `engine-analyze-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: empty table: sqlite_stat1 created but holds no rows (see harness).

## Observables

- `analyze` = `rc=0 err=-`
- `exists` = `1`
- `cnt` = `0`
