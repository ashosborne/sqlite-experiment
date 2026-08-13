# engine-harvest40-004-C001 — run-11 oneshot characterization

Feature: `engine-harvest40-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: update_hook fires op 18/23/9 with rowids on a rowid table and is fully SUPPRESSED on a WITHOUT ROWID table (see harness).

## Observables

- `rowid_events` = `3`
- `rowid_log` = `[18:r:1][23:r:1][9:r:1]`
- `wr_events` = `0`
- `wr_log` = ``
