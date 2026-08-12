# engine-analyze-001-C010 — run-11 oneshot characterization

Feature: `engine-analyze-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DROP INDEX / DROP TABLE clear matching stat1 rows (see harness).

## Observables

- `before` = `t,ia,3 2|u,~,1`
- `dropidx` = `rc=0 err=-`
- `after_dropidx` = `u,~,1`
- `droptbl` = `rc=0 err=-`
- `after_droptbl` = ``
