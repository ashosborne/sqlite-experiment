# engine-harvest40-007-C001 — run-11 oneshot characterization

Feature: `engine-harvest40-007` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: live completion candidates: keywords, schema names, tables+views, columns (the pragma/function/collation phases are dead in live C) (see harness).

## Observables

- `comp_sel` = `SELECT`
- `comp_wid` = `widgets`
- `comp_wcol` = `wcol1|wcol2`
- `comp_mai` = `main`
- `comp_wv` = `wview`
