# engine-vdbe48-003-C001 — run-11 oneshot characterization

Feature: `engine-vdbe48-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: stretch: after the 12-row overflow split (page_count 4, root 2 now interior) the scan listing is unchanged and OpenRead on the interior root still yields all rows in rowid order (see harness).

## Observables

- `page_count` = `4`
- `root_and_type` = `2`
- `x_split` = `0|Init|0|7|0|~|0|~/1|OpenRead|0|2|0|1|0|~/2|Rewind|0|6|0|~|0|~/3|Column|0|0|1|~|0|~/4|ResultRow|1|1|0|~|0|~/5|Next|0|3|0|~|1|~/6|Halt|0|0|0|~|0|~/7|Transaction|0|0|1|0|1|~/8|Goto|0|1|0|~|0|~`
- `run_a` = `10/20/30/40/50/60/70/80/90/100/110/120`
