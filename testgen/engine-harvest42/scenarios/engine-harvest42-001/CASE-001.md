# engine-harvest42-001-C001 — run-11 oneshot characterization

Feature: `engine-harvest42-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: non-recursive WITH: literal seed, real-table body, no-column-list form; column names come from the CTE declaration or the body projection (see harness).

## Observables

- `w_lit` = `rows=1 LOG=`
- `w_lit_cols` = `a`
- `w_tab` = `rows=2,y|3,z LOG=`
- `w_noname` = `rows=1|2 LOG=`
