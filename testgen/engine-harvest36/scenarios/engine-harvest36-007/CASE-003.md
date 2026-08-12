# engine-harvest36-007-C003 — run-11 oneshot characterization

Feature: `engine-harvest36-007` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: PRAGMA optimize creates missing sqlite_stat1 for indexed tables (see harness).

## Observables

- `optimize_fresh` = `rc=0 err=-`
- `stat1_after` = `1`
- `stat1_rows` = `t,ti,4 1`
