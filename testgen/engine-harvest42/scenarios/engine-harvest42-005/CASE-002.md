# engine-harvest42-005-C002 — run-11 oneshot characterization

Feature: `engine-harvest42-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: outer LIMIT stops an unbounded recursion at the limit; outer ORDER BY sorts the queue output; a recursive CTE works as a subquery source (see harness).

## Observables

- `r_limit` = `rows=1|2|3|4 LOG=`
- `r_orderby` = `rows=1|2|3 LOG=`
- `r_sub` = `rows=4 LOG=`
