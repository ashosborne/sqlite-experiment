# engine-vdbe49-001-C001 — run-11 oneshot characterization

Feature: `engine-vdbe49-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: C EXPLAIN of SELECT b FROM t WHERE a=1: the compare is INVERTED into a jump-to-Next Ne(2,7,1,BINARY-8,84) with the literal loaded to r2 by an init-section Integer; Column a->r1, Column b->r3, ResultRow(3,1) (see harness).

## Observables

- `x_eq` = `0|Init|0|9|0|~|0|~/1|OpenRead|0|2|0|2|0|~/2|Rewind|0|8|0|~|0|~/3|Column|0|0|1|~|0|~/4|Ne|2|7|1|BINARY-8|84|~/5|Column|0|1|3|~|0|~/6|ResultRow|3|1|0|~|0|~/7|Next|0|3|0|~|1|~/8|Halt|0|0|0|~|0|~/9|Transaction|0|0|1|0|1|~/10|Integer|1|2|0|~|0|~/11|Goto|0|1|0|~|0|~`
