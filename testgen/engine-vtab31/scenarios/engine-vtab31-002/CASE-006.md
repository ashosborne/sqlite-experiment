# engine-vtab31-002-C006 — run-11 oneshot characterization

Feature: `engine-vtab31-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: declare_vtab outside xCreate/xConnect -> SQLITE_MISUSE 21 (see harness).

## Observables

- `misuse` = `rc=21`
