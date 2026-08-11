# loadext-api-001-C001 — run-11 oneshot characterization

Feature: `loadext-api-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: load_extension with loading disabled -> rc + msg class (see harness).

## Observables

- `load.rc` = `1`
- `errmsg.not_authorized` = `1`
