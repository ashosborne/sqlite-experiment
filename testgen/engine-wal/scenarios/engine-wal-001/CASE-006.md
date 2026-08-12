# engine-wal-001-C006 — run-11 oneshot characterization

Feature: `engine-wal-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: several autocommit txns all visible after reopen (see harness).

## Observables

- `rows` = `3,6`
