# engine-vdbe47-001-C002 — run-11 oneshot characterization

Feature: `engine-vdbe47-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: WHERE 1 folds to the same program; WHERE 0 inserts Goto->Halt so no row is produced (see harness).

## Observables

- `x_where_1` = `0|Init|0|4|0|~|0|~/1|Integer|1|1|0|~|0|~/2|ResultRow|1|1|0|~|0|~/3|Halt|0|0|0|~|0|~/4|Goto|0|1|0|~|0|~`
- `x_where_0` = `0|Init|0|5|0|~|0|~/1|Goto|0|4|0|~|0|~/2|Integer|1|1|0|~|0|~/3|ResultRow|1|1|0|~|0|~/4|Halt|0|0|0|~|0|~/5|Goto|0|1|0|~|0|~`
