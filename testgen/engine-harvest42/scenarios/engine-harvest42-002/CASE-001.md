# engine-harvest42-002-C001 — run-11 oneshot characterization

Feature: `engine-harvest42-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: recursive UNION ALL over integers: Queue/Current FIFO row order; multi-column recursion with text accumulation (see harness).

## Observables

- `r_int` = `rows=1|2|3|4|5 LOG=`
- `r_expr` = `rows=1,a|2,ab|3,abb LOG=`
