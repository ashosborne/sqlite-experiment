# engine-idxfile-001-C002 — run-11 oneshot characterization

Feature: `engine-idxfile-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: multi-column UNIQUE INDEX enforces duplicates after reopen (see harness).

## Observables

- `w.rc` = `0`
- `dup.rc` = `19`
- `ok.rc` = `0`
- `count` = `3`
