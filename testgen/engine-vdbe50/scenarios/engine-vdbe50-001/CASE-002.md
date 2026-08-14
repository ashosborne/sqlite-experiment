# engine-vdbe50-001-C002 — run-11 oneshot characterization

Feature: `engine-vdbe50-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: omitting the column list emits the identical program (only the literal differs) (see harness).

## Observables

- `x_ins_plain` = `0|Init|0|8|0|~|0|~/1|OpenWrite|0|2|0|2|0|~/2|Integer|2|2|0|~|0|~/3|String8|0|3|0|two|0|~/4|NewRowid|0|1|0|~|0|~/5|MakeRecord|2|2|4|DB|0|~/6|Insert|0|4|1|t|57|~/7|Halt|0|0|0|~|0|~/8|Transaction|0|1|1|0|1|~/9|Goto|0|1|0|~|0|~`
