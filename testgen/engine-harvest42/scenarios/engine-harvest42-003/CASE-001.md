# engine-harvest42-003-C001 — run-11 oneshot characterization

Feature: `engine-harvest42-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: recursive walk of a parent/child table is BREADTH-first (FIFO queue), both comma-join and JOIN..ON forms (see harness).

## Observables

- `r_bfs` = `rows=1|2|3|4|5|6|7 LOG=`
- `r_bfs2` = `rows=2|4|5 LOG=`
