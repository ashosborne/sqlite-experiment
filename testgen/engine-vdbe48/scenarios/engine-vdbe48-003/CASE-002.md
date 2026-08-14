# engine-vdbe48-003-C002 — run-11 oneshot characterization

Feature: `engine-vdbe48-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: boundary pin: SELECT length(b) FROM t is an EXPRESSION, not this pack scan shape - the kitchen evaluator still owns it (rows pinned on C to freeze the boundary, not claimed as bytecode) (see harness).

## Observables

- `first_b_len` = `500/500/500/500/500/500/500/500/500/500/500/500`
