# engine-orrollback-001-C003 — run-11 oneshot characterization

Feature: `engine-orrollback-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: FK orphan inside txn then explicit ROLLBACK undoes everything (see harness).

## Observables

- `begin.rc` = `0`
- `orphan.rc` = `19`
- `rollback.rc` = `0`
- `p` = `0`
- `c` = `0`
