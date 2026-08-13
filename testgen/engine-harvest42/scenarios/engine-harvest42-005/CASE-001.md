# engine-harvest42-005-C001 — run-11 oneshot characterization

Feature: `engine-harvest42-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: recursive UNION (distinct) dedupes and terminates a cyclic graph (DistFifo: 1|2|3, the cycle-closing row is never re-enqueued) (see harness).

## Observables

- `r_union_distinct` = `rows=1|2|3 LOG=`
