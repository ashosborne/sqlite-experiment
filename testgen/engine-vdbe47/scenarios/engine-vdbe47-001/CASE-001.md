# engine-vdbe47-001-C001 — run-11 oneshot characterization

Feature: `engine-vdbe47-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: C EXPLAIN of SELECT 1 / SELECT 42: Init(0,4), Integer(n,1), ResultRow(1,1), Halt, Goto(0,1) — full row shape addr/opcode/p1/p2/p3/p4/p5/comment (see harness).

## Observables

- `x_select_1` = `0|Init|0|4|0|~|0|~/1|Integer|1|1|0|~|0|~/2|ResultRow|1|1|0|~|0|~/3|Halt|0|0|0|~|0|~/4|Goto|0|1|0|~|0|~`
- `x_select_42` = `0|Init|0|4|0|~|0|~/1|Integer|42|1|0|~|0|~/2|ResultRow|1|1|0|~|0|~/3|Halt|0|0|0|~|0|~/4|Goto|0|1|0|~|0|~`
