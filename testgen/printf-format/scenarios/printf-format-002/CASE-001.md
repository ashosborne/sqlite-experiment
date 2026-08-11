# printf-format-002-C001 — run-11 oneshot characterization

Feature: `printf-format-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: mprintf %d-%Q with NULL (see harness).

## Observables

- `mprintf.text` = `5-NULL`
