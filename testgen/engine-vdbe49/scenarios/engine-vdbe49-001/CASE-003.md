# engine-vdbe49-001-C003 — run-11 oneshot characterization

Feature: `engine-vdbe49-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: the whole inverted family: <> emits Eq, > emits Le, < emits Ge, >= emits Lt, <= emits Gt - all with p4=BINARY-8 p5=84 jumping to Next (see harness).

## Observables

- `x_ne` = `0|Init|0|9|0|~|0|~/1|OpenRead|0|2|0|2|0|~/2|Rewind|0|8|0|~|0|~/3|Column|0|0|1|~|0|~/4|Eq|2|7|1|BINARY-8|84|~/5|Column|0|1|3|~|0|~/6|ResultRow|3|1|0|~|0|~/7|Next|0|3|0|~|1|~/8|Halt|0|0|0|~|0|~/9|Transaction|0|0|1|0|1|~/10|Integer|1|2|0|~|0|~/11|Goto|0|1|0|~|0|~`
- `x_gt` = `0|Init|0|9|0|~|0|~/1|OpenRead|0|2|0|2|0|~/2|Rewind|0|8|0|~|0|~/3|Column|0|0|1|~|0|~/4|Le|2|7|1|BINARY-8|84|~/5|Column|0|1|3|~|0|~/6|ResultRow|3|1|0|~|0|~/7|Next|0|3|0|~|1|~/8|Halt|0|0|0|~|0|~/9|Transaction|0|0|1|0|1|~/10|Integer|1|2|0|~|0|~/11|Goto|0|1|0|~|0|~`
- `x_lt` = `0|Init|0|9|0|~|0|~/1|OpenRead|0|2|0|2|0|~/2|Rewind|0|8|0|~|0|~/3|Column|0|0|1|~|0|~/4|Ge|2|7|1|BINARY-8|84|~/5|Column|0|1|3|~|0|~/6|ResultRow|3|1|0|~|0|~/7|Next|0|3|0|~|1|~/8|Halt|0|0|0|~|0|~/9|Transaction|0|0|1|0|1|~/10|Integer|2|2|0|~|0|~/11|Goto|0|1|0|~|0|~`
- `x_ge` = `0|Init|0|9|0|~|0|~/1|OpenRead|0|2|0|2|0|~/2|Rewind|0|8|0|~|0|~/3|Column|0|0|1|~|0|~/4|Lt|2|7|1|BINARY-8|84|~/5|Column|0|1|3|~|0|~/6|ResultRow|3|1|0|~|0|~/7|Next|0|3|0|~|1|~/8|Halt|0|0|0|~|0|~/9|Transaction|0|0|1|0|1|~/10|Integer|2|2|0|~|0|~/11|Goto|0|1|0|~|0|~`
- `x_le` = `0|Init|0|9|0|~|0|~/1|OpenRead|0|2|0|2|0|~/2|Rewind|0|8|0|~|0|~/3|Column|0|0|1|~|0|~/4|Gt|2|7|1|BINARY-8|84|~/5|Column|0|1|3|~|0|~/6|ResultRow|3|1|0|~|0|~/7|Next|0|3|0|~|1|~/8|Halt|0|0|0|~|0|~/9|Transaction|0|0|1|0|1|~/10|Integer|1|2|0|~|0|~/11|Goto|0|1|0|~|0|~`
