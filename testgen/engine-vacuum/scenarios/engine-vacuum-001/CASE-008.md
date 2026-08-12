# engine-vacuum-001-C008 — run-11 oneshot characterization

Feature: `engine-vacuum-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: implicit rowids renumbered by the real rebuild (1,3,5 -> 1,2,3) (see harness).

## Observables

- `before` = `1,a|3,c|5,e`
- `vac` = `rc=0 err=-`
- `after` = `1,a|2,c|3,e`
