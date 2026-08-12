# engine-analyze-002-C002 — run-11 oneshot characterization

Feature: `engine-analyze-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ANALYZE survives VACUUM; re-ANALYZE after VACUUM (see harness).

## Observables

- `before_vac` = `t,ia,2 1`
- `vac` = `rc=0 err=-`
- `after_vac` = `t,ia,2 1`
- `analyze2` = `rc=0 err=-`
- `after_re` = `t,ia,2 1`
