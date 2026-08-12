# engine-txnfile-001-C002 — run-11 oneshot characterization

Feature: `engine-txnfile-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: file: ROLLBACK then reopen sees nothing (see harness).

## Observables

- `w.rc` = `0`
- `count` = `0`
