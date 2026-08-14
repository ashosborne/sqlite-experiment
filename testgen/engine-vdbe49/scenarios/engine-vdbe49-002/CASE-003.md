# engine-vdbe49-002-C003 — run-11 oneshot characterization

Feature: `engine-vdbe49-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: the family executes: <>, >, <, >=, <= each return C rows in rowid order (see harness).

## Observables

- `run_ne` = `two/three`
- `run_gt` = `two/three`
- `run_lt` = `one/uno`
- `run_ge` = `two/three`
- `run_le` = `one/uno`
