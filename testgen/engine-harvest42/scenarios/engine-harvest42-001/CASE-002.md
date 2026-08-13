# engine-harvest42-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest42-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: multiple CTEs with an earlier-CTE reference (cross join); MATERIALIZED and NOT MATERIALIZED accepted with identical results (see harness).

## Observables

- `w_two` = `rows=10,15 LOG=`
- `w_mat` = `rows=7 LOG=`
- `w_notmat` = `rows=8 LOG=`
