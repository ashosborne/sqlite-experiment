# engine-idxfile-001-C004 — run-11 oneshot characterization

Feature: `engine-idxfile-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: partial UNIQUE index: dups outside predicate allowed, inside -> 19, after reopen (see harness).

## Observables

- `w.rc` = `0`
- `dup5.rc` = `0`
- `dup20.rc` = `19`
- `count` = `4`
