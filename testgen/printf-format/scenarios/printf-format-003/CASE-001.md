# printf-format-003-C001 — run-11 oneshot characterization

Feature: `printf-format-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: str_new/appendf/errcode/finish (see harness).

## Observables

- `errcode` = `0`
- `text` = `3/ab`
