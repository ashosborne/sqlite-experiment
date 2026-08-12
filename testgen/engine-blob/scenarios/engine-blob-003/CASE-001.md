# engine-blob-003-C001 — run-11 oneshot characterization

Feature: `engine-blob-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: file db: handle write persists across reopen (integrity ok) (see harness).

## Observables

- `write` = `rc=0 err=not an error`
- `reopen` = `HELLO!,6`
- `integ` = `ok`
