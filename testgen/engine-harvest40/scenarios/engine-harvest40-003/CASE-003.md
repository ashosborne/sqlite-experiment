# engine-harvest40-003-C003 — run-11 oneshot characterization

Feature: `engine-harvest40-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: trace_v2 replaces both legacy callbacks; clearing trace_v2 silences everything (see harness).

## Observables

- `nlt` = `0`
- `nlp` = `0`
- `log` = `[V2:SELECT 1;]`
- `nlt2` = `0`
- `nlp2` = `0`
- `log2` = ``
