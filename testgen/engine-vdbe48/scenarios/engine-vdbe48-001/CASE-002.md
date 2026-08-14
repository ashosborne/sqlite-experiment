# engine-vdbe48-001-C002 — run-11 oneshot characterization

Feature: `engine-vdbe48-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: two columns widen to two Column ops + ResultRow(1,2) with OpenRead p4=2; SELECT b alone keeps p4=2 (hint = max used col + 1) with Column(0,1,1) (see harness).

## Observables

- `x_ab` = `0|Init|0|8|0|~|0|~/1|OpenRead|0|2|0|2|0|~/2|Rewind|0|7|0|~|0|~/3|Column|0|0|1|~|0|~/4|Column|0|1|2|~|0|~/5|ResultRow|1|2|0|~|0|~/6|Next|0|3|0|~|1|~/7|Halt|0|0|0|~|0|~/8|Transaction|0|0|1|0|1|~/9|Goto|0|1|0|~|0|~`
- `x_b` = `0|Init|0|7|0|~|0|~/1|OpenRead|0|2|0|2|0|~/2|Rewind|0|6|0|~|0|~/3|Column|0|1|1|~|0|~/4|ResultRow|1|1|0|~|0|~/5|Next|0|3|0|~|1|~/6|Halt|0|0|0|~|0|~/7|Transaction|0|0|1|0|1|~/8|Goto|0|1|0|~|0|~`
