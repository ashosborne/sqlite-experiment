# engine-harvest36-008-C002 — run-11 oneshot characterization

Feature: `engine-harvest36-008` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: unindexed WHERE still full-scans; expression-only statement scans nothing (see harness).

## Observables

- `where_fullscan` = `4`
- `no_table_fullscan` = `0`
- `run_one` = `1`
