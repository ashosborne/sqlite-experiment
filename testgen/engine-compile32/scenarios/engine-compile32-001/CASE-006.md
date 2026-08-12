# engine-compile32-001-C006 — run-11 oneshot characterization

Feature: `engine-compile32-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: SQL twin sqlite_compileoption_get first/last/out-of-range NULL (see harness).

## Observables

- `sql_get` = `ATOMIC_INTRINSICS=1,THREADSAFE=1,~`
