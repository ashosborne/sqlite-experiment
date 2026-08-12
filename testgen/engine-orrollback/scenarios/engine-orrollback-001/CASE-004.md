# engine-orrollback-001-C004 — run-11 oneshot characterization

Feature: `engine-orrollback-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: OR ROLLBACK in autocommit: statement rolled back, existing row remains (see harness).

## Observables

- `dup.rc` = `19`
- `autocommit` = `1`
- `count` = `1`
