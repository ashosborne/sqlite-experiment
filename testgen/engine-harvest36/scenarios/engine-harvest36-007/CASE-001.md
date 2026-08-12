# engine-harvest36-007-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-007` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ANALYZE aux.t writes aux.sqlite_stat1 (main stays clean) (see harness).

## Observables

- `analyze_aux_table` = `rc=0 err=-`
- `aux_stat1` = `t,ti,3 1`
- `main_no_stat1` = `0`
