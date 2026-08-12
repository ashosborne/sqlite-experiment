# engine-harvest36-004-C002 — run-11 oneshot characterization

Feature: `engine-harvest36-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: flags=2 accepts JSON5 forms (bare keys, trailing comma, single quotes, comments, hex, +Infinity, .5); strict rejects (see harness).

## Observables

- `json5_accepts` = `1,1,1,1,1,1,1`
- `strict_rejects` = `0,0,0,0`
