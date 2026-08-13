# engine-harvest40-004-C002 — run-11 oneshot characterization

Feature: `engine-harvest40-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DELETE without WHERE takes the truncate fast-path: zero update_hook events but changes() counts the rows; WHERE-qualified DELETE fires per row (see harness).

## Observables

- `trunc_events` = `0`
- `trunc_changes` = `3`
- `where_events` = `2`
- `where_changes` = `2`
- `where_log` = `[9:r:4][9:r:5]`
