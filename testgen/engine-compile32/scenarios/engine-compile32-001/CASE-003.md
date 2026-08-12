# engine-compile32-001-C003 — run-11 oneshot characterization

Feature: `engine-compile32-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: get(): first/second/last entries; past-end/far/negative -> NULL (see harness).

## Observables

- `first` = `get=ATOMIC_INTRINSICS=1`
- `second` = `get=COMPILER=gcc-13.3.0`
- `last` = `get=THREADSAFE=1`
- `past_end` = `get=NULL`
- `far` = `get=NULL`
- `negative` = `get=NULL`
