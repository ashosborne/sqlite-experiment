# engine-vdbe48-001-C001 — run-11 oneshot characterization

Feature: `engine-vdbe48-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: C EXPLAIN of SELECT a FROM t (single-CREATE file db, root 2): Init(0,7), OpenRead(0,root,0,p4=1), Rewind(0,6), Column(0,0,1), ResultRow(1,1), Next(0,3,p5=1), Halt, Transaction(0,0,cookie=1,p4=0,p5=1), Goto(0,1) (see harness).

## Observables

- `rootpage` = `2`
- `x_a` = `0|Init|0|7|0|~|0|~/1|OpenRead|0|2|0|1|0|~/2|Rewind|0|6|0|~|0|~/3|Column|0|0|1|~|0|~/4|ResultRow|1|1|0|~|0|~/5|Next|0|3|0|~|1|~/6|Halt|0|0|0|~|0|~/7|Transaction|0|0|1|0|1|~/8|Goto|0|1|0|~|0|~`
