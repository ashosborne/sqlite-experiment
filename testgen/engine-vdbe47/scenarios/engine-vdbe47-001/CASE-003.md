# engine-vdbe47-001-C003 — run-11 oneshot characterization

Feature: `engine-vdbe47-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: 1+2 is NOT constant-folded (Add with init-section register loads after Halt); String8 carries the text in p4; two columns widen ResultRow(1,2) (see harness).

## Observables

- `x_add` = `0|Init|0|4|0|~|0|~/1|Add|3|2|1|~|0|~/2|ResultRow|1|1|0|~|0|~/3|Halt|0|0|0|~|0|~/4|Integer|1|2|0|~|0|~/5|Integer|2|3|0|~|0|~/6|Goto|0|1|0|~|0|~`
- `x_text` = `0|Init|0|4|0|~|0|~/1|String8|0|1|0|hi|0|~/2|ResultRow|1|1|0|~|0|~/3|Halt|0|0|0|~|0|~/4|Goto|0|1|0|~|0|~`
- `x_two_cols` = `0|Init|0|5|0|~|0|~/1|Integer|1|1|0|~|0|~/2|Integer|2|2|0|~|0|~/3|ResultRow|1|2|0|~|0|~/4|Halt|0|0|0|~|0|~/5|Goto|0|1|0|~|0|~`
