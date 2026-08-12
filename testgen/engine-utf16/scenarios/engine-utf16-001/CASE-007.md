# engine-utf16-001-C007 — run-11 oneshot characterization

Feature: `engine-utf16-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: syntax error rc + errmsg (see harness).

## Observables

- `prep.rc` = `1`
- `errmsg` = `near "SELECTT": syntax error`
