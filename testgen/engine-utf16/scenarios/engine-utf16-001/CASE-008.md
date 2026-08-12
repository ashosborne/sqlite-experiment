# engine-utf16-001-C008 — run-11 oneshot characterization

Feature: `engine-utf16-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: UTF-16 CREATE/INSERT then SELECT count/max (see harness).

## Observables

- `cnt` = `2`
- `mx` = `42`
